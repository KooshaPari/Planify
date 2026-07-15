/**
 * Automated Accessibility (axe-core) Audit
 * Runs vitest-axe on every component to assert no violations.
 * Known issues documented inline with disableRules where appropriate.
 *
 * Run: npx vitest run src/tests/a11y/axe-audit.test.tsx
 */

import React from 'react';
import { render } from '@testing-library/react';
import { axe } from 'vitest-axe';
import { toHaveNoViolations } from 'vitest-axe/matchers';
import { Button } from '../../components/foundation/Button';
import { Input } from '../../components/foundation/Input';
import { Select } from '../../components/foundation/Select';
import { Badge } from '../../components/layout/Badge';
import { Skeleton } from '../../components/foundation/Skeleton';
import { EmptyState } from '../../components/layout/EmptyState';
import { Modal } from '../../components/layout/Modal';
import { vi } from 'vitest';

// toHaveNoViolations is extended globally in test/setup.ts

// Helper: render component and run axe-core audit
async function assertNoAxeViolations(ui: React.ReactElement, description: string) {
  const { container } = render(ui);
  const results = await axe(container);
  expect(results).toHaveNoViolations();
}

// ============================================================================
// Button
// ============================================================================

describe('Button axe audit', () => {
  it('primary button has no violations', async () => {
    await assertNoAxeViolations(<Button variant="primary">Submit</Button>, 'primary');
  });

  it('secondary button has no violations', async () => {
    await assertNoAxeViolations(<Button variant="secondary">Cancel</Button>, 'secondary');
  });

  it('ghost button has no violations', async () => {
    await assertNoAxeViolations(<Button variant="ghost">More</Button>, 'ghost');
  });

  it('destructive button has no violations', async () => {
    await assertNoAxeViolations(<Button variant="destructive">Delete</Button>, 'destructive');
  });

  it('disabled button has no violations', async () => {
    await assertNoAxeViolations(<Button disabled>Disabled</Button>, 'disabled');
  });

  it('button with aria-label has no violations', async () => {
    await assertNoAxeViolations(
      <Button ariaLabel="Close dialog" variant="ghost">
        ×
      </Button>,
      'aria-label'
    );
  });
});

// ============================================================================
// Input
// ============================================================================

describe('Input axe audit', () => {
  it('basic input has no violations', async () => {
    await assertNoAxeViolations(<Input placeholder="Enter text" />, 'basic');
  });

  it('input with label has no violations', async () => {
    await assertNoAxeViolations(<Input label="Email" />, 'with-label');
  });

  it('input with error has no violations', async () => {
    await assertNoAxeViolations(<Input label="Name" error="Name is required" />, 'with-error');
  });

  it('input with required and error has no violations', async () => {
    await assertNoAxeViolations(
      <Input label="Password" type="password" required error="Too short" />,
      'required-error'
    );
  });

  it('disabled input has no violations', async () => {
    await assertNoAxeViolations(<Input label="Read only" disabled value="fixed" />, 'disabled');
  });
});

// ============================================================================
// Select
// ============================================================================

describe('Select axe audit', () => {
  const options = [
    { value: 'open', label: 'Open' },
    { value: 'in_progress', label: 'In Progress' },
    { value: 'done', label: 'Done' },
    { value: 'blocked', label: 'Blocked', disabled: true },
  ];

  it('basic select has no violations', async () => {
    await assertNoAxeViolations(<Select label="Status" options={options.slice(0, 2)} />, 'basic');
  });

  it('select with label has no violations', async () => {
    await assertNoAxeViolations(<Select label="Status" options={options} />, 'with-label');
  });

  it('select with error has no violations', async () => {
    await assertNoAxeViolations(
      <Select label="Priority" options={options} error="Please select a priority" />,
      'with-error'
    );
  });

  it('select with placeholder has no violations', async () => {
    await assertNoAxeViolations(
      <Select label="Status" placeholder="Choose..." options={options} />,
      'with-placeholder'
    );
  });

  it('disabled select has no violations', async () => {
    await assertNoAxeViolations(
      <Select label="Status" options={options} disabled />,
      'disabled'
    );
  });
});

// ============================================================================
// Badge
// ============================================================================

describe('Badge axe audit', () => {
  it('default badge has no violations', async () => {
    await assertNoAxeViolations(<Badge label="Default" />, 'default');
  });

  it('success badge has no violations', async () => {
    await assertNoAxeViolations(<Badge label="Active" variant="success" />, 'success');
  });

  it('warning badge has no violations', async () => {
    await assertNoAxeViolations(<Badge label="Pending" variant="warning" />, 'warning');
  });

  it('error badge has no violations', async () => {
    await assertNoAxeViolations(<Badge label="Failed" variant="error" />, 'error');
  });

  it('info badge has no violations', async () => {
    await assertNoAxeViolations(<Badge label="Info" variant="info" />, 'info');
  });
});

// ============================================================================
// Skeleton
// ============================================================================

describe('Skeleton axe audit', () => {
  it('text skeleton has no violations', async () => {
    await assertNoAxeViolations(<Skeleton variant="text" />, 'text');
  });

  it('circular skeleton has no violations', async () => {
    await assertNoAxeViolations(<Skeleton variant="circular" width={48} height={48} />, 'circular');
  });

  it('rectangular skeleton has no violations', async () => {
    await assertNoAxeViolations(
      <Skeleton variant="rectangular" width="100%" height={200} />,
      'rectangular'
    );
  });

  it('multi-count skeleton has no violations', async () => {
    await assertNoAxeViolations(<Skeleton variant="text" count={3} />, 'multi-count');
  });

  it('non-animated skeleton has no violations', async () => {
    await assertNoAxeViolations(
      <Skeleton variant="text" animate={false} />,
      'non-animated'
    );
  });
});

// ============================================================================
// EmptyState
// ============================================================================

describe('EmptyState axe audit', () => {
  it('simple empty state has no violations', async () => {
    await assertNoAxeViolations(
      <EmptyState title="No items" description="There are no items to display." />,
      'simple'
    );
  });

  it('empty state with action has no violations', async () => {
    await assertNoAxeViolations(
      <EmptyState
        title="No work packages"
        description="Create your first work package to get started."
        action={<Button>Create Package</Button>}
      />,
      'with-action'
    );
  });
});

// ============================================================================
// Modal
// ============================================================================

describe('Modal axe audit', () => {
  it('open modal has no violations', async () => {
    await assertNoAxeViolations(
      <Modal isOpen={true} onClose={vi.fn()} title="Confirm Action">
        <p>Are you sure you want to proceed?</p>
      </Modal>,
      'open'
    );
  });

  it('modal with footer has no violations', async () => {
    await assertNoAxeViolations(
      <Modal
        isOpen={true}
        onClose={vi.fn()}
        title="Settings"
        footer={<Button>Save</Button>}
      >
        <p>Modal content with footer action.</p>
      </Modal>,
      'with-footer'
    );
  });

  it('modal with custom aria-label has no violations', async () => {
    await assertNoAxeViolations(
      <Modal
        isOpen={true}
        onClose={vi.fn()}
        ariaLabel="Custom dialog label"
      >
        <p>Content with custom label.</p>
      </Modal>,
      'custom-aria-label'
    );
  });

  it('large modal has no violations', async () => {
    await assertNoAxeViolations(
      <Modal isOpen={true} onClose={vi.fn()} title="Large" size="lg">
        <p>Large modal content.</p>
      </Modal>,
      'large'
    );
  });
});
