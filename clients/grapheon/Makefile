# Makefile — thin shim that defers to justfile (preferred task runner)
# Install just: https://github.com/casey/just

.DEFAULT_GOAL := help
.PHONY: help link status unlink refresh verify

help:
	@echo "Grapheon client linkage — use 'just' for the canonical interface."
	@echo "This Makefile is a back-compat shim. Available targets:"
	@echo "  make link       # Create symlinks"
	@echo "  make status     # Show drift detection"
	@echo "  make unlink     # Remove symlinks (source repos untouched)"
	@echo "  make refresh    # Remove + recreate symlinks"
	@echo "  make verify     # Verify linkage + show contents"
	@just --list

link status unlink refresh verify:
	@just $(MAKECMDGOALS)
