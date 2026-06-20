# Testing Strategy

- Run `go test ./...` after the dependency bump.
- If the module graph changes, verify `go mod tidy` leaves the tree clean.
- Confirm the commit contains only the intended dependency update and documentation refresh.
