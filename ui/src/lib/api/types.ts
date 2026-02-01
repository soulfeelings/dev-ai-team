export interface Project {
  id: string;
  name: string;
  github_url: string;
  local_path: string | null;
  default_branch: string;
  railway_project_id: string | null;
  railway_service_id: string | null;
  railway_environment_id: string | null;
  deployment_url: string | null;
  deployment_status: string;
  created_at: string;
  updated_at: string;
}

export interface DeployResponse {
  railway_project_id: string;
  railway_service_id: string;
  deployment_status: string;
  message: string;
}

export interface DeploymentStatusResponse {
  status: string;
  deployment_url: string | null;
}

export interface CreateProjectRequest {
  name: string;
  github_url: string;
  local_path?: string;
  default_branch?: string;
}

export type TaskStatus = 'Backlog' | 'InProgress' | 'QA' | 'Review' | 'Done' | 'Failed';
export type AgentRole = 'Planner' | 'Dev' | 'QA' | 'Reviewer';
export type AgentRunStatus = 'Pending' | 'Running' | 'Completed' | 'Failed';

export interface Task {
  id: string;
  project_id: string;
  title: string;
  description: string;
  acceptance_criteria: string;
  status: TaskStatus;
  priority: number;
  parent_task_id: string | null;
  branch_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateTaskRequest {
  title: string;
  description?: string;
  acceptance_criteria?: string;
  priority?: number;
  parent_task_id?: string;
  branch_name?: string;
}

export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  acceptance_criteria?: string;
  status?: TaskStatus;
  priority?: number;
  branch_name?: string;
}

export interface AgentRun {
  id: string;
  task_id: string;
  agent_role: AgentRole;
  status: AgentRunStatus;
  runner_id: string | null;
  started_at: string | null;
  completed_at: string | null;
  patch_content: string | null;
  error_message: string | null;
  created_at: string;
}

export interface AssignAgentRequest {
  agent_role: AgentRole;
  priority?: number;
}

export interface ChatMessage {
  id: string;
  task_id: string | null;
  project_id: string;
  sender_type: string;
  sender_name: string;
  content: string;
  created_at: string;
}

export interface CreateChatMessageRequest {
  task_id?: string;
  sender_type: string;
  sender_name: string;
  content: string;
}
