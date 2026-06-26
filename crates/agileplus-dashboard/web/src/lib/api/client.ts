import type {
  ApiErrorBody,
  EpicResponse,
  FeatureResponse,
  ProjectResponse,
  StoryResponse,
  WorkPackageResponse,
} from './types';

const DEFAULT_BASE = 'http://localhost:3000';
const DEFAULT_TIMEOUT_MS = 30_000;

export class ApiClientError extends Error {
  readonly status: number;
  readonly body: ApiErrorBody | undefined;

  constructor(message: string, status: number, body?: ApiErrorBody) {
    super(message);
    this.name = 'ApiClientError';
    this.status = status;
    this.body = body;
  }
}

export function getApiBaseUrl(): string {
  const base = import.meta.env.VITE_API_BASE?.trim();
  return base && base.length > 0 ? base.replace(/\/$/, '') : DEFAULT_BASE;
}

function getApiKey(): string | undefined {
  const key = import.meta.env.VITE_API_KEY?.trim();
  return key && key.length > 0 ? key : undefined;
}

function getTimeoutMs(): number {
  const raw = import.meta.env.VITE_API_TIMEOUT;
  if (!raw) return DEFAULT_TIMEOUT_MS;
  const parsed = Number.parseInt(raw, 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : DEFAULT_TIMEOUT_MS;
}

function buildUrl(path: string): string {
  const normalized = path.startsWith('/') ? path : `/${path}`;
  return `${getApiBaseUrl()}${normalized}`;
}

async function parseJson<T>(response: Response): Promise<T> {
  const text = await response.text();
  if (!text) {
    return undefined as T;
  }
  return JSON.parse(text) as T;
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const headers = new Headers(init?.headers);
  if (!headers.has('Accept')) {
    headers.set('Accept', 'application/json');
  }

  const apiKey = getApiKey();
  if (apiKey && !headers.has('X-API-Key')) {
    headers.set('X-API-Key', apiKey);
  }

  const controller = new AbortController();
  const timeout = window.setTimeout(() => controller.abort(), getTimeoutMs());

  try {
    const response = await fetch(buildUrl(path), {
      ...init,
      headers,
      signal: controller.signal,
    });

    if (!response.ok) {
      let body: ApiErrorBody | undefined;
      try {
        body = await parseJson<ApiErrorBody>(response);
      } catch {
        body = undefined;
      }
      const message =
        body?.error ?? body?.message ?? `Request failed (${response.status})`;
      throw new ApiClientError(message, response.status, body);
    }

    return parseJson<T>(response);
  } catch (error) {
    if (error instanceof ApiClientError) {
      throw error;
    }
    if (error instanceof DOMException && error.name === 'AbortError') {
      throw new ApiClientError('Request timed out', 408);
    }
    throw error;
  } finally {
    window.clearTimeout(timeout);
  }
}

export const apiClient = {
  get<T>(path: string): Promise<T> {
    return request<T>(path, { method: 'GET' });
  },
};

/** Dashboard view models (mapped from API responses). */
export interface DashboardEpic {
  id: number;
  title: string;
  status: string;
  requirement_id: string | null;
}

export interface DashboardStory {
  id: number;
  epic_id: number | null;
  title: string;
  status: string;
  requirement_id: string | null;
}

export interface DashboardWorkPackage {
  id: string;
  title: string;
  status: 'planned' | 'in_progress' | 'completed' | 'blocked';
  priority: 'low' | 'medium' | 'high' | 'critical';
  assignee?: string;
}

function formatEpicStatus(status: string): string {
  switch (status.toLowerCase()) {
    case 'done':
      return 'Done';
    case 'active':
    case 'review':
      return 'In Progress';
    case 'backlog':
      return 'Planned';
    case 'cancelled':
      return 'Blocked';
    default:
      return status;
  }
}

function formatStoryStatus(status: string): string {
  switch (status.toLowerCase()) {
    case 'done':
      return 'Done';
    case 'in_progress':
    case 'review':
      return 'In Progress';
    case 'todo':
      return 'Planned';
    case 'blocked':
      return 'Blocked';
    default:
      return status;
  }
}

function mapWorkPackageState(
  state: string,
): DashboardWorkPackage['status'] {
  switch (state.toLowerCase()) {
    case 'done':
      return 'completed';
    case 'doing':
    case 'review':
      return 'in_progress';
    case 'blocked':
      return 'blocked';
    case 'planned':
    default:
      return 'planned';
  }
}

function resolveProjectSlug(
  projects: ProjectResponse[],
  explicit?: string,
): string | undefined {
  if (explicit) {
    return explicit;
  }
  const fromEnv = import.meta.env.VITE_PROJECT_SLUG?.trim();
  if (fromEnv) {
    return fromEnv;
  }
  return projects[0]?.slug;
}

/**
 * Load epics and stories for the dashboard from agileplus-api v1 endpoints.
 *
 * Flow: projects → project epics → stories per epic.
 */
export async function fetchDashboardEpicsStories(): Promise<{
  epics: DashboardEpic[];
  stories: DashboardStory[];
}> {
  const projects = await apiClient.get<ProjectResponse[]>('/api/v1/projects');
  const slug = resolveProjectSlug(projects);
  if (!slug) {
    return { epics: [], stories: [] };
  }

  const epicResponses = await apiClient.get<EpicResponse[]>(
    `/api/v1/projects/${encodeURIComponent(slug)}/epics`,
  );

  const storyGroups = await Promise.all(
    epicResponses.map((epic) =>
      apiClient.get<StoryResponse[]>(
        `/api/v1/epics/${epic.id}/stories`,
      ),
    ),
  );

  const epics: DashboardEpic[] = epicResponses.map((epic) => ({
    id: epic.id,
    title: epic.title,
    status: formatEpicStatus(epic.status),
    requirement_id: null,
  }));

  const stories: DashboardStory[] = storyGroups.flatMap((group, index) => {
    const epicId = epicResponses[index]?.id ?? null;
    return group.map((story) => ({
      id: story.id,
      epic_id: epicId,
      title: story.title,
      status: formatStoryStatus(story.status),
      requirement_id: null,
    }));
  });

  return { epics, stories };
}

/** Load work packages across all features for the Zustand store. */
export async function fetchDashboardWorkPackages(): Promise<DashboardWorkPackage[]> {
  const features = await apiClient.get<FeatureResponse[]>('/api/v1/features');
  const groups = await Promise.all(
    features.map((feature) =>
      apiClient.get<WorkPackageResponse[]>(
        `/api/v1/features/${encodeURIComponent(feature.slug)}/work-packages`,
      ),
    ),
  );

  return groups.flat().map((wp) => ({
    id: String(wp.id),
    title: wp.title,
    status: mapWorkPackageState(wp.state),
    priority: 'medium',
  }));
}
