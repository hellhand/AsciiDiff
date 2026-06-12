#!/bin/bash
# Launch AsciiDiff in a container with display forwarding.
# Works on X11 and Wayland hosts (including RHEL 9 with glibc 2.34).
#
# Usage:
#   ./asciidiff-container.sh [/path/to/git/repo]
#
# The script auto-detects X11 vs Wayland and forwards the appropriate socket.

set -euo pipefail

IMAGE="asciidiff:latest"
CONTAINER_NAME="asciidiff-$$"
REPO_PATH="${1:-}"

# Check if image exists
if ! docker image inspect "$IMAGE" &>/dev/null && ! podman image inspect "$IMAGE" &>/dev/null 2>&1; then
    echo "Image '$IMAGE' not found. Build it first:"
    echo "  docker build -f Dockerfile.run -t asciidiff ."
    exit 1
fi

# Detect container runtime (prefer podman on RHEL)
if command -v podman &>/dev/null; then
    RUNTIME=podman
elif command -v docker &>/dev/null; then
    RUNTIME=docker
else
    echo "Neither docker nor podman found. Install one first."
    exit 1
fi

# Common flags
ARGS=(
    --rm
    --name "$CONTAINER_NAME"
    --hostname asciidiff
    -e "WEBKIT_DISABLE_DMABUF_RENDERER=1"
)

# Display forwarding
if [ -n "${WAYLAND_DISPLAY:-}" ] && [ -e "${XDG_RUNTIME_DIR}/${WAYLAND_DISPLAY}" ]; then
    # Wayland
    ARGS+=(
        -e "WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
        -e "XDG_RUNTIME_DIR=/tmp/runtime"
        -v "${XDG_RUNTIME_DIR}/${WAYLAND_DISPLAY}:/tmp/runtime/${WAYLAND_DISPLAY}:rw"
    )
    # Also pass XDG_SESSION_TYPE if set
    [ -n "${XDG_SESSION_TYPE:-}" ] && ARGS+=(-e "XDG_SESSION_TYPE=$XDG_SESSION_TYPE")
elif [ -n "${DISPLAY:-}" ]; then
    # X11
    ARGS+=(
        -e "DISPLAY=$DISPLAY"
        -v /tmp/.X11-unix:/tmp/.X11-unix:rw
    )
    # Share Xauthority if it exists
    if [ -n "${XAUTHORITY:-}" ] && [ -f "$XAUTHORITY" ]; then
        ARGS+=(-e "XAUTHORITY=/tmp/.Xauthority" -v "$XAUTHORITY:/tmp/.Xauthority:ro")
    elif [ -f "$HOME/.Xauthority" ]; then
        ARGS+=(-e "XAUTHORITY=/tmp/.Xauthority" -v "$HOME/.Xauthority:/tmp/.Xauthority:ro")
    fi
else
    echo "No display server detected (neither WAYLAND_DISPLAY nor DISPLAY is set)."
    exit 1
fi

# GPU access (needed for hardware-accelerated rendering)
if [ -e /dev/dri ]; then
    ARGS+=(--device /dev/dri)
fi

# D-Bus (needed for GTK and appindicator)
if [ -n "${DBUS_SESSION_BUS_ADDRESS:-}" ]; then
    ARGS+=(-e "DBUS_SESSION_BUS_ADDRESS=$DBUS_SESSION_BUS_ADDRESS")
    # Extract socket path if unix:path=...
    DBUS_SOCK="${DBUS_SESSION_BUS_ADDRESS#unix:path=}"
    DBUS_SOCK="${DBUS_SOCK%%,*}"
    if [ -S "$DBUS_SOCK" ]; then
        ARGS+=(-v "$DBUS_SOCK:$DBUS_SOCK:rw")
    fi
fi

# Mount git repo if specified
if [ -n "$REPO_PATH" ]; then
    REPO_PATH="$(realpath "$REPO_PATH")"
    if [ ! -d "$REPO_PATH/.git" ] && [ ! -f "$REPO_PATH/HEAD" ]; then
        echo "Warning: $REPO_PATH does not look like a git repository."
    fi
    ARGS+=(-v "$REPO_PATH:/repo:ro")
    echo "Mounting $REPO_PATH as /repo (read-only)"
fi

echo "Starting AsciiDiff (${RUNTIME})..."
exec "$RUNTIME" run "${ARGS[@]}" "$IMAGE"
