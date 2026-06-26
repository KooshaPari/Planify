import { useEffect } from 'react';
import { fetchDashboardWorkPackages } from '../lib/api';
import { useAgilePlusStore } from '../stores/agileplus';

// ============================================================================
// useWorkPackages Hook
// Fetch and manage work package data from agileplus-api
// ============================================================================

interface UseWorkPackagesOptions {
  skip?: boolean;
}

/**
 * Hook to fetch and manage work packages from agileplus-api v1 endpoints.
 */
export function useWorkPackages(options: UseWorkPackagesOptions = {}) {
  const { skip = false } = options;
  const { workPackages, setWorkPackages, setLoading } = useAgilePlusStore();

  useEffect(() => {
    if (skip) return;

    let cancelled = false;

    const fetchWorkPackages = async () => {
      setLoading(true);
      try {
        const packages = await fetchDashboardWorkPackages();
        if (!cancelled) {
          setWorkPackages(packages);
        }
      } catch (error) {
        console.error('Failed to fetch work packages:', error);
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    void fetchWorkPackages();

    return () => {
      cancelled = true;
    };
  }, [skip, setWorkPackages, setLoading]);

  return {
    workPackages,
    loading: useAgilePlusStore((state) => state.loading),
  };
}

export default useWorkPackages;
