# Implementation Strategy

- Use a single swap from `github.com/docker/docker` to `github.com/moby/moby/client`.
- Let Go regenerate checksums from the live module graph.
- Keep code changes mechanical: dependency, adapter comment, README note.
