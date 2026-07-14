#!/bin/bash
set -e

IMAGE_NAME="asciidiff-builder"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CONTAINER_CMD="${CONTAINER_CMD:-$(command -v podman 2>/dev/null || command -v docker 2>/dev/null)}"

echo "==> Building container image (openSUSE Leap 15.5, glibc 2.31)..."
"$CONTAINER_CMD" build -t "$IMAGE_NAME" -f "$PROJECT_DIR/Dockerfile.build" "$PROJECT_DIR"

echo "==> Building AppImage inside container..."
"$CONTAINER_CMD" run --rm \
    -v "$PROJECT_DIR":/app:Z \
    -e APPIMAGE_EXTRACT_AND_RUN=1 \
    "$IMAGE_NAME" \
    bash -c "cd /app/frontend && npm ci && npx tauri build --bundles appimage"

echo "==> Done! AppImage is at:"
ls -lh "$PROJECT_DIR/frontend/src-tauri/target/release/bundle/appimage/"*.AppImage
