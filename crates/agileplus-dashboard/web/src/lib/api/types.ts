/** JSON shapes returned by agileplus-api `/api/v1/*` endpoints. */

export interface ProjectResponse {
  id: number;
  slug: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface EpicResponse {
  id: number;
  project_id: number;
  title: string;
  description: string | null;
  status: string;
  owner_id: number | null;
  created_at: string;
  updated_at: string;
}

export interface StoryResponse {
  id: number;
  epic_id: number;
  project_id: number;
  title: string;
  description: string | null;
  status: string;
  points: number | null;
  assignee_id: number | null;
  created_at: string;
  updated_at: string;
}

export interface FeatureResponse {
  id: number;
  slug: string;
  name: string;
  state: string;
  target_branch: string;
  created_at: string;
  updated_at: string;
}

export interface WorkPackageResponse {
  id: number;
  feature_id: number;
  title: string;
  state: string;
  sequence: number;
  acceptance_criteria: string;
  pr_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface ApiErrorBody {
  error?: string;
  message?: string;
}
