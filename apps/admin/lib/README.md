# Planify/apps/admin/lib

Re-exports shared components from `@space/lib` for use in the admin application.

## Overview

This library provides a thin re-export layer that allows the admin application to use components defined in the space application. This approach enables code sharing between the two applications while maintaining separation of concerns.

## Contents

```
lib/
├── index.ts           # Barrel export
└── b-progress/        # Progress bar component re-export
    ├── index.tsx     # Re-export from space
    └── AppProgressBar.tsx
```

## Usage

```typescript
import { AppProgressBar } from './lib';

// Use the progress bar component
function MyComponent() {
  return (
    <AppProgressBar
      progress={0.75}
      label="Processing..."
    />
  );
}
```

## Components

### AppProgressBar

Re-exported from `@space/lib/b-progress`.

A progress bar component for displaying task completion status.

**Props:**

| Prop | Type | Description |
|------|------|-------------|
| `progress` | `number` | Progress value between 0 and 1 |
| `label` | `string` | Optional label text |
| `showPercentage` | `boolean` | Show percentage text (default: true) |

**Example:**

```typescript
import { AppProgressBar } from '@admin/lib';

function LoadingView() {
  return (
    <AppProgressBar
      progress={0.5}
      label="Loading data"
      showPercentage={true}
    />
  );
}
```

## Architecture

This library follows the **re-export pattern** for cross-app component sharing:

1. Components are defined once in `@space/lib`
2. Admin app re-exports via `@admin/lib`
3. Both apps use the same component source

This ensures:
- Single source of truth for shared components
- Consistent behavior across applications
- Easy updates propagate to all consumers

## Dependencies

- `@space/lib/b-progress` - Source component
- `react` - UI framework

## Notes

- The `b-progress` directory contains re-export code only
- All actual implementation lives in `Planify/apps/space/lib/b-progress`
- Changes to the source component automatically affect admin app
