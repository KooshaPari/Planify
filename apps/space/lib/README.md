# Planify/apps/space/lib

React context providers for the Plane Space application.

## Overview

This library provides the core React context providers for the Plane Space application:

- **StoreProvider**: MobX store context with singleton management
- **InstanceProvider**: Instance information and authentication state
- **ToastProvider**: Toast notification system

## Usage

```tsx
import {
  StoreProvider,
  InstanceProvider,
  ToastProvider,
  StoreContext,
  type StoreProviderProps,
} from './lib';

// Wrap your app with providers (order matters)
function App({ children }) {
  return (
    <StoreProvider>
      <InstanceProvider>
        <ToastProvider>
          {children}
        </ToastProvider>
      </InstanceProvider>
    </StoreProvider>
  );
}

// Access store from context
import { useContext } from 'react';
const store = useContext(StoreContext);
```

## Providers

### StoreProvider

Provides the MobX RootStore singleton. Handles SSR/hydration and ensures a single store instance on the client.

```tsx
<StoreProvider initialState={initialData}>
  {children}
</StoreProvider>
```

**Props:**

```typescript
interface StoreProviderProps {
  children: React.ReactNode;
  initialState?: any;  // Optional hydration state
}
```

### InstanceProvider

Handles instance information fetching and user authentication. Shows loading spinner while fetching, or error view on failure.

```tsx
<InstanceProvider>
  {children}
</InstanceProvider>
```

**Features:**
- Fetches instance info via SWR
- Fetches current user
- Shows `LogoSpinner` during loading
- Shows `InstanceFailureView` on error
- Renders children only when instance is loaded

### ToastProvider

Provides toast notifications using the Plane Toast component.

```tsx
<ToastProvider>
  {children}
</ToastProvider>
```

**Features:**
- Integrates with next-themes for theme-aware toasts
- Resolves general theme from resolved theme

## Store Context

```typescript
import { StoreContext } from './lib';

// Access from any component
const rootStore = useContext(StoreContext);

// Available stores (from RootStore)
rootStore.userStore;
rootStore.projectStore;
rootStore.issueStore;
// etc.
```

## Architecture Notes

- Uses **MobX** for state management with `mobx-react` for React integration
- Uses **SWR** for data fetching
- Uses **next-themes** for theme resolution
- Store is a **singleton** on the client, new instance on SSR

## Dependencies

- `react`
- `mobx-react`
- `react-router`
- `next-themes`
- `swr`
- `@plane/constants`
- `@plane/propel/icons`
- `@plane/propel/toast`
- `@plane/utils`
- `@/hooks/store/*`
- `@/components/*`

## Files

- `index.ts` - Barrel export
- `store-provider.tsx` - MobX store provider
- `instance-provider.tsx` - Instance/user provider
- `toast-provider.tsx` - Toast notification provider

## Known Issues

The `b-progress` directory contains duplicate component code. This requires an architectural decision to resolve the duplication.
