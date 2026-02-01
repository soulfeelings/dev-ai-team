import type {
  Project,
  CreateProjectRequest,
  Task,
  CreateTaskRequest,
  UpdateTaskRequest,
  AgentRun,
  AssignAgentRequest,
  ChatMessage,
  CreateChatMessageRequest,
  DeployResponse,
  DeploymentStatusResponse
} from './types';

const API_BASE = 'http://127.0.0.1:3000/api';

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers
    }
  });

  if (!res.ok) {
    const error = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(error.error || 'Request failed');
  }

  if (res.status === 204) {
    return undefined as T;
  }

  return res.json();
}

// Projects
export const projectsApi = {
  list: () => request<Project[]>('/projects'),

  get: (id: string) => request<Project>(`/projects/${id}`),

  create: (data: CreateProjectRequest) =>
    request<Project>('/projects', {
      method: 'POST',
      body: JSON.stringify(data)
    }),

  update: (id: string, data: Partial<CreateProjectRequest>) =>
    request<Project>(`/projects/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(data)
    }),

  delete: (id: string) =>
    request<void>(`/projects/${id}`, { method: 'DELETE' }),

  deploy: (id: string) =>
    request<DeployResponse>(`/projects/${id}/deploy`, { method: 'POST' }),

  getDeploymentStatus: (id: string) =>
    request<DeploymentStatusResponse>(`/projects/${id}/deployment`)
};

// Tasks
export const tasksApi = {
  list: (projectId: string) => request<Task[]>(`/projects/${projectId}/tasks`),

  get: (id: string) => request<Task>(`/tasks/${id}`),

  create: (projectId: string, data: CreateTaskRequest) =>
    request<Task>(`/projects/${projectId}/tasks`, {
      method: 'POST',
      body: JSON.stringify(data)
    }),

  update: (id: string, data: UpdateTaskRequest) =>
    request<Task>(`/tasks/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(data)
    }),

  delete: (id: string) =>
    request<void>(`/tasks/${id}`, { method: 'DELETE' }),

  assignAgent: (id: string, data: AssignAgentRequest) =>
    request<AgentRun>(`/tasks/${id}/assign-agent`, {
      method: 'POST',
      body: JSON.stringify(data)
    }),

  getRuns: (id: string) => request<AgentRun[]>(`/tasks/${id}/runs`)
};

// Chat
export const chatApi = {
  getProjectChat: (projectId: string, limit?: number) =>
    request<ChatMessage[]>(`/projects/${projectId}/chat${limit ? `?limit=${limit}` : ''}`),

  getTaskChat: (taskId: string, limit?: number) =>
    request<ChatMessage[]>(`/tasks/${taskId}/chat${limit ? `?limit=${limit}` : ''}`),

  send: (projectId: string, data: CreateChatMessageRequest) =>
    request<ChatMessage>(`/projects/${projectId}/chat`, {
      method: 'POST',
      body: JSON.stringify(data)
    })
};
