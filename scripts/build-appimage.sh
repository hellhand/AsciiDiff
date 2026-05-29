#!/bin/bash
set -e

IMAGE_NAME="asciidiff-builder"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "==> Building Docker image (AlmaLinux 9 base, glibc 2.34)..."
docker build -t "$IMAGE_NAME" -f "$PROJECT_DIR/Dockerfile.build" "$PROJECT_DIR"

echo "==> Building AppImage inside container..."
docker run --rm \
    -v "$PROJECT_DIR":/app:Z \
    -e APPIMAGE_EXTRACT_AND_RUN=1 \
    "$IMAGE_NAME" \
    bash -c "cd /app/frontend && npm ci && npx tauri build --bundles appimage"

echo "==> Done! AppImage is at:"
ls -lh "$PROJECT_DIR/frontend/src-tauri/target/release/bundle/appimage/"*.AppImage
