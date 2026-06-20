package docker_test

import (
	"testing"

	"github.com/KooshaPari/devenv-abstraction/pkg/adapters/docker"
	"github.com/KooshaPari/devenv-abstraction/pkg/domain"
)

// TestCompileTimeContract verifies that docker.Adapter implements domain.Environment.
func TestCompileTimeContract(t *testing.T) {
	// This is a compile-time check; if it compiles, the contract is satisfied.
	var _ domain.Environment = (*docker.Adapter)(nil)
}

// TestNew verifies that New() returns a valid adapter without error.
func TestNew(t *testing.T) {
	adapter, err := docker.New()
	if err != nil {
		// Docker may not be available in CI; this is expected to fail gracefully.
		t.Skipf("Docker not available (expected in CI without Docker): %v", err)
	}
	if adapter == nil {
		t.Fatal("expected non-nil adapter")
	}
}
