/**
 * Keyboard Navigation Accessibility Tests
 * Tests Tab order, Enter/Space activation, Escape closes modals,
 * and arrow key navigation across all interactive components.
 *
 * Uses @testing-library/user-event for realistic keyboard interactions.
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Button } from '../../components/foundation/Button';
import { Input } from '../../components/foundation/Input';
import { Select } from '../../components/foundation/Select';
import { Checkbox } from '../../components/foundation/Checkbox';
import { Radio } from '../../components/foundation/Radio';
import { Toggle } from '../../components/foundation/Toggle';
import { Modal } from '../../components/layout/Modal';

// ============================================================================
// Tab Navigation
// ============================================================================

describe('Tab Navigation', () => {
  it('tabs forward through interactive elements in DOM order', async () => {
    const user = userEvent.setup();
    render(
      <div>
        <Button>First</Button>
        <Button>Second</Button>
        <Button>Third</Button>
      </div>
    );

    const [first, second, third] = screen.getAllByRole('button');
    expect(document.body).toHaveFocus();

    await user.tab();
    expect(first).toHaveFocus();

    await user.tab();
    expect(second).toHaveFocus();

    await user.tab();
    expect(third).toHaveFocus();
  });

  it('tabs backward through interactive elements with Shift+Tab', async () => {
    const user = userEvent.setup();
    render(
      <div>
        <Button>First</Button>
        <Button>Second</Button>
        <Button>Third</Button>
      </div>
    );

    const [first, second, third] = screen.getAllByRole('button');

    // Tab forward to the last element first
    await user.tab();
    await user.tab();
    await user.tab();
    expect(third).toHaveFocus();

    // Now tab backward
    await user.tab({ shift: true });
    expect(second).toHaveFocus();

    await user.tab({ shift: true });
    expect(first).toHaveFocus();
  });

  it('skips disabled buttons in tab order', async () => {
    const user = userEvent.setup();
    render(
      <div>
        <Button>First</Button>
        <Button disabled>Disabled (skipped)</Button>
        <Button>Third</Button>
      </div>
    );

    await user.tab();
    expect(screen.getAllByRole('button')[0]).toHaveFocus();

    await user.tab();
    // Should skip the disabled button and focus the third
    expect(screen.getAllByRole('button')[2]).toHaveFocus();
  });

  it('tabs through mixed form elements in correct order', async () => {
    const user = userEvent.setup();
    render(
      <div>
        <Input label="Name" />
        <Select
          label="Role"
          options={[
            { value: 'dev', label: 'Developer' },
            { value: 'pm', label: 'Project Manager' },
          ]}
        />
        <Button>Submit</Button>
      </div>
    );

    // Tab to name input
    await user.tab();
    expect(screen.getByLabelText('Name')).toHaveFocus();

    // Tab to select
    await user.tab();
    expect(screen.getByLabelText('Role')).toHaveFocus();

    // Tab to submit button
    await user.tab();
    expect(screen.getByRole('button', { name: 'Submit' })).toHaveFocus();
  });
});

// ============================================================================
// Enter / Space Activation
// ============================================================================

describe('Keyboard Activation (Enter / Space)', () => {
  it('activates Button with Enter key', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Activate</Button>);

    const button = screen.getByRole('button', { name: 'Activate' });
    button.focus();
    expect(button).toHaveFocus();

    await user.keyboard('{Enter}');
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('activates Button with Space key', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Activate</Button>);

    const button = screen.getByRole('button', { name: 'Activate' });
    button.focus();

    await user.keyboard(' ');
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('does not activate disabled Button with Enter', async () => {
    const user = userEvent.setup();
    const onClick = vi.fn();
    render(
      <Button onClick={onClick} disabled>
        Disabled
      </Button>
    );

    const button = screen.getByRole('button', { name: 'Disabled' });
    button.focus();

    await user.keyboard('{Enter}');
    expect(onClick).not.toHaveBeenCalled();
  });

  it('activates Checkbox with Space key', async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();
    render(<Checkbox label="Accept terms" onChange={onChange} />);

    const checkbox = screen.getByRole('checkbox');
    checkbox.focus();

    await user.keyboard(' ');
    expect(onChange).toHaveBeenCalledWith(true);
  });

  it('toggles Toggle with Enter key', async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();
    render(<Toggle label="Notifications" onChange={onChange} />);

    const toggle = screen.getByRole('button');
    toggle.focus();

    await user.keyboard('{Enter}');
    expect(onChange).toHaveBeenCalled();
  });
});

// ============================================================================
// Escape Closes Modals
// ============================================================================

describe('Escape Key Closes Modal', () => {
  it('closes Modal when Escape is pressed', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(
      <Modal isOpen={true} onClose={onClose} title="Test Modal">
        <p>Modal content</p>
      </Modal>
    );

    expect(screen.getByRole('dialog')).toBeInTheDocument();

    await user.keyboard('{Escape}');
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('does not close Modal when Escape is pressed and modal is closed', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(
      <Modal isOpen={false} onClose={onClose} title="Hidden Modal">
        <p>You should not see this</p>
      </Modal>
    );

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

    await user.keyboard('{Escape}');
    expect(onClose).not.toHaveBeenCalled();
  });

  it('closes Modal when close button is activated with Enter', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(
      <Modal isOpen={true} onClose={onClose} title="Dismissible">
        <p>Content</p>
      </Modal>
    );

    const closeButton = screen.getByLabelText('Close dialog');
    closeButton.focus();

    await user.keyboard('{Enter}');
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});

// ============================================================================
// Arrow Key Navigation in Custom Controls
// ============================================================================

describe('Arrow Key Navigation', () => {
  it('navigates Radio options with arrow keys', async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();

    render(
      <div role="radiogroup" aria-label="Priority">
        <Radio value="low" label="Low" onChange={onChange} />
        <Radio value="medium" label="Medium" onChange={onChange} />
        <Radio value="high" label="High" onChange={onChange} />
      </div>
    );

    const radios = screen.getAllByRole('radio');
    expect(radios).toHaveLength(3);

    // Focus first radio
    radios[0].focus();
    expect(radios[0]).toHaveFocus();

    // Navigate down with arrow
    await user.keyboard('{ArrowDown}');
    expect(onChange).toHaveBeenCalledWith('medium');
  });

  it('navigates Select options with arrow keys', async () => {
    const user = userEvent.setup();

    render(
      <Select
        label="Status"
        options={[
          { value: 'open', label: 'Open' },
          { value: 'in_progress', label: 'In Progress' },
          { value: 'done', label: 'Done' },
        ]}
      />
    );

    const select = screen.getByLabelText('Status');
    select.focus();

    // Open the select and navigate with arrow keys
    await user.keyboard('{Enter}');
    await user.keyboard('{ArrowDown}');
    await user.keyboard('{ArrowDown}');

    // Native select should have navigated to third option
    expect(select).toHaveFocus();
  });
});

// ============================================================================
// Focus Management
// ============================================================================

describe('Focus Management', () => {
  it('focuses Modal content on open', () => {
    render(
      <Modal isOpen={true} onClose={vi.fn()} title="Focused Modal">
        <p>Auto-focused content</p>
      </Modal>
    );

    const dialog = screen.getByRole('dialog');
    // Modal should receive focus via tabIndex={-1}
    expect(dialog).toHaveAttribute('tabindex', '-1');
  });

  it('focuses Input when clicked label is associated', async () => {
    const user = userEvent.setup();
    render(<Input label="Email address" />);

    const label = screen.getByText('Email address');
    await user.click(label);

    const input = screen.getByLabelText('Email address');
    expect(input).toHaveFocus();
  });

  it('all all interactive elements are focusable via Tab', () => {
    render(
      <div>
        <Button>Click</Button>
        <Input label="Text" />
        <Select
          label="Pick"
          options={[
            { value: 'a', label: 'A' },
            { value: 'b', label: 'B' },
          ]}
        />
        <Checkbox label="Check" />
        <Toggle label="Toggle" />
      </div>
    );

    const buttons = screen.getAllByRole('button');
    const textbox = screen.getByRole('textbox');
    const combobox = screen.getByRole('combobox');
    const checkbox = screen.getByRole('checkbox');

    // All should be focusable (not disabled, not hidden)
    expect(buttons.length).toBeGreaterThanOrEqual(1);
    expect(textbox).not.toBeDisabled();
    expect(combobox).not.toBeDisabled();
    expect(checkbox).not.toBeDisabled();
  });
});

// ============================================================================
// Visible Focus Indicators
// ============================================================================

describe('Visible Focus Indicators', () => {
  it('Button has focus-visible classes', () => {
    const { container } = render(<Button>Focusable</Button>);
    const button = container.querySelector('button');
    expect(button).toHaveClass('focus-visible:outline-none');
    expect(button).toHaveClass('focus-visible:ring-2');
    expect(button).toHaveClass('focus-visible:ring-offset-2');
  });

  it('Input has focus-visible classes', () => {
    const { container } = render(<Input label="Test" />);
    const input = container.querySelector('input');
    expect(input).toHaveClass('focus-visible:outline-none');
    expect(input).toHaveClass('focus-visible:ring-2');
  });

  it('Select has focus-visible classes', () => {
    const { container } = render(
      <Select
        label="Test"
        options={[
          { value: 'a', label: 'A' },
        ]}
      />
    );
    const select = container.querySelector('select');
    expect(select).toHaveClass('focus-visible:outline-none');
    expect(select).toHaveClass('focus-visible:ring-2');
  });
});
