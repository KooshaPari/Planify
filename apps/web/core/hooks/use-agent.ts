/**
 * Copyright (c) 2023-present Plane contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 */

import { useCallback, useEffect, useState } from "react";

export type TAgentStatus = "active" | "inactive" | "training" | string;

export type TAgent = {
  id: string;
  name: string;
  status: TAgentStatus;
  description?: string | null;
  last_active?: string | null;
};

type TAgentListResponse = TAgent[] | { results?: unknown };

const isAgent = (value: unknown): value is TAgent => {
  if (!value || typeof value !== "object") return false;

  const candidate = value as Partial<TAgent> & { id?: unknown; name?: unknown; status?: unknown };
  return typeof candidate.id === "string" && typeof candidate.name === "string" && typeof candidate.status === "string";
};

const normalizeAgents = (value: unknown): TAgent[] => {
  if (Array.isArray(value)) {
    return value.filter(isAgent);
  }

  if (value && typeof value === "object" && "results" in value) {
    const { results } = value as TAgentListResponse;
    return Array.isArray(results) ? results.filter(isAgent) : [];
  }

  return [];
};

export const useAgents = (workspaceSlug: string) => {
  const [agents, setAgents] = useState<TAgent[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(Boolean(workspaceSlug));
  const [error, setError] = useState<string | null>(null);

  const fetchAgents = useCallback(async () => {
    if (!workspaceSlug) {
      setAgents([]);
      setError(null);
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(`/api/v1/workspaces/${workspaceSlug}/agents/`);

      if (!response.ok) {
        throw new Error(`Failed to fetch agents (${response.status})`);
      }

      const data: unknown = await response.json();
      setAgents(normalizeAgents(data));
    } catch (fetchError) {
      setAgents([]);
      setError(fetchError instanceof Error ? fetchError.message : "Failed to load agents");
    } finally {
      setIsLoading(false);
    }
  }, [workspaceSlug]);

  useEffect(() => {
    void fetchAgents();
  }, [fetchAgents]);

  return {
    agents,
    isLoading,
    error,
    fetchAgents,
  };
};
