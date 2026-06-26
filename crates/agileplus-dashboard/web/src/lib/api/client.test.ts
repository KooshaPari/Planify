import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';
import {
  ApiClientError,
  fetchDashboardEpicsStories,
  getApiBaseUrl,
} from './client';

describe('api client', () => {
  beforeEach(() => {
    vi.stubEnv('VITE_API_BASE', 'http://localhost:3000');
    vi.stubEnv('VITE_API_KEY', 'test-key');
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it('getApiBaseUrl defaults to localhost when unset', () => {
    vi.unstubAllEnvs();
    expect(getApiBaseUrl()).toBe('http://localhost:3000');
  });

  it('getApiBaseUrl trims trailing slash', () => {
    vi.stubEnv('VITE_API_BASE', 'http://api.example.com/');
    expect(getApiBaseUrl()).toBe('http://api.example.com');
  });

  it('fetchDashboardEpicsStories maps API responses to dashboard models', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        new Response(JSON.stringify([{ id: 1, slug: 'proj', name: 'Proj', description: null, created_at: '', updated_at: '' }]), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify([{ id: 10, project_id: 1, title: 'Epic A', description: null, status: 'active', owner_id: null, created_at: '', updated_at: '' }]), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      )
      .mockResolvedValueOnce(
        new Response(JSON.stringify([{ id: 100, epic_id: 10, project_id: 1, title: 'Story A', description: null, status: 'done', points: null, assignee_id: null, created_at: '', updated_at: '' }]), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      );

    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchDashboardEpicsStories();

    expect(result.epics).toEqual([
      { id: 10, title: 'Epic A', status: 'In Progress', requirement_id: null },
    ]);
    expect(result.stories).toEqual([
      { id: 100, epic_id: 10, title: 'Story A', status: 'Done', requirement_id: null },
    ]);

    expect(fetchMock).toHaveBeenCalledWith(
      'http://localhost:3000/api/v1/projects',
      expect.objectContaining({
        headers: expect.any(Headers),
      }),
    );
  });

  it('throws ApiClientError on non-OK responses', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ error: 'Unauthorized' }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    );

    await expect(fetchDashboardEpicsStories()).rejects.toBeInstanceOf(ApiClientError);
  });
});
