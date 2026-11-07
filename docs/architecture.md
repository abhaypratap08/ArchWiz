# ARCHWIZ Architecture

## Pipeline

Filesystem -> Scanner -> Project Model -> Classifier -> Graph Generator -> Layout Engine -> Terminal Renderer.

## Modules

- `scanner`: recursive traversal, exclusions, depth limits, symlink policy.
- `model`: `Project`, `DirectoryNode`, `Node`, `FileNode`.
- `classifier`: file kind and source language detection.
- `graph`: builds `Graph` (nodes + edges) independent of rendering.
- `layout`: computes node positions and connector strokes.
- `theme`: semantic color tokens.
- `renderer`: canvas, terminal width/color, drawing.