<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { projectsApi, tasksApi } from '$lib/api/client';
  import { tasks } from '$lib/stores/tasks';
  import type { Project, Task, AgentRun, AgentRole, TaskStatus, CreateTaskRequest } from '$lib/api/types';
  import TaskCard from '$lib/components/TaskCard.svelte';
  import CreateTaskModal from '$lib/components/CreateTaskModal.svelte';
  import TaskDetailModal from '$lib/components/TaskDetailModal.svelte';

  let project: Project | null = $state(null);
  let loading = $state(true);
  let error = $state('');
  let deploying = $state(false);

  let showCreateModal = $state(false);
  let showDetailModal = $state(false);
  let selectedTask: Task | null = $state(null);
  let selectedTaskRuns: AgentRun[] = $state([]);

  const projectId = $derived($page.params.id);

  onMount(async () => {
    await loadData();
    // Poll deployment status if deploying
    if (project?.deployment_status === 'deploying') {
      pollDeploymentStatus();
    }
  });

  async function loadData() {
    loading = true;
    error = '';
    try {
      const [projectData] = await Promise.all([
        projectsApi.get(projectId),
        tasks.load(projectId)
      ]);
      project = projectData;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load project';
    } finally {
      loading = false;
    }
  }

  async function handleCreateTask(data: CreateTaskRequest) {
    try {
      await tasks.add(projectId, data);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to create task';
      throw e;
    }
  }

  async function handleDeleteTask(id: string) {
    try {
      await tasks.remove(id);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to delete task';
    }
  }

  async function openTaskDetail(task: Task) {
    selectedTask = task;
    selectedTaskRuns = [];
    showDetailModal = true;

    try {
      selectedTaskRuns = await tasks.getRuns(task.id);
    } catch (e) {
      console.error('Failed to load runs:', e);
    }
  }

  async function handleAssignAgent(role: AgentRole) {
    if (!selectedTask) return;

    try {
      const run = await tasks.assignAgent(selectedTask.id, role);
      selectedTaskRuns = [run, ...selectedTaskRuns];
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to assign agent';
    }
  }

  async function handleStatusChange(status: TaskStatus) {
    if (!selectedTask) return;

    try {
      const updated = await tasks.update(selectedTask.id, { status });
      selectedTask = updated;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to update status';
    }
  }

  async function handleDeploy() {
    if (!project) return;
    deploying = true;
    error = '';

    try {
      await projectsApi.deploy(projectId);
      // Update local project state
      project = { ...project, deployment_status: 'deploying' };
      // Start polling for status
      pollDeploymentStatus();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to deploy';
      deploying = false;
    }
  }

  async function pollDeploymentStatus() {
    const poll = async () => {
      try {
        const status = await projectsApi.getDeploymentStatus(projectId);
        if (project) {
          project = {
            ...project,
            deployment_status: status.status,
            deployment_url: status.deployment_url
          };
        }
        // Keep polling if still deploying
        if (status.status === 'DEPLOYING' || status.status === 'deploying' || status.status === 'BUILDING') {
          setTimeout(poll, 5000);
        } else {
          deploying = false;
        }
      } catch (e) {
        console.error('Failed to get deployment status:', e);
        deploying = false;
      }
    };
    poll();
  }

  const deploymentStatusLabel = $derived.by(() => {
    if (!project) return '';
    const status = project.deployment_status;
    switch (status) {
      case 'not_deployed': return 'Not Deployed';
      case 'deploying': return 'Deploying...';
      case 'DEPLOYING': return 'Deploying...';
      case 'BUILDING': return 'Building...';
      case 'SUCCESS': return 'Deployed';
      case 'FAILED': return 'Failed';
      default: return status;
    }
  });

  // Group tasks by status
  const tasksByStatus = $derived.by(() => {
    const grouped: Record<string, Task[]> = {
      Backlog: [],
      InProgress: [],
      QA: [],
      Review: [],
      Done: [],
      Failed: []
    };

    for (const task of $tasks) {
      if (grouped[task.status]) {
        grouped[task.status].push(task);
      }
    }

    return grouped;
  });

  const statusLabels: Record<string, string> = {
    Backlog: 'Backlog',
    InProgress: 'In Progress',
    QA: 'QA',
    Review: 'Review',
    Done: 'Done',
    Failed: 'Failed'
  };
</script>

<div class="container">
  {#if loading}
    <div class="empty-state">Loading project...</div>
  {:else if error}
    <div class="error-banner">
      {error}
      <button onclick={() => error = ''}>Dismiss</button>
    </div>
  {:else if project}
    <div class="page-header">
      <div>
        <a href="/" class="back-link">&larr; Projects</a>
        <h1>{project.name}</h1>
        <p class="text-secondary">{project.github_url}</p>
        <div class="deployment-info">
          <span class="deployment-status" class:deployed={project.deployment_status === 'SUCCESS'} class:deploying={deploying}>
            {deploymentStatusLabel}
          </span>
          {#if project.deployment_url}
            <a href={project.deployment_url} target="_blank" rel="noopener" class="deployment-url">
              {project.deployment_url}
            </a>
          {/if}
        </div>
      </div>
      <div class="header-actions">
        <button
          class="btn btn-secondary"
          onclick={handleDeploy}
          disabled={deploying}
        >
          {deploying ? 'Deploying...' : (project.railway_service_id ? 'Redeploy' : 'Deploy to Railway')}
        </button>
        <button class="btn btn-primary" onclick={() => showCreateModal = true}>
          + New Task
        </button>
      </div>
    </div>

    {#if $tasks.length === 0}
      <div class="empty-state">
        <p>No tasks yet</p>
        <button class="btn btn-primary mt-2" onclick={() => showCreateModal = true}>
          Create your first task
        </button>
      </div>
    {:else}
      <div class="kanban">
        {#each Object.entries(tasksByStatus) as [status, statusTasks]}
          <div class="kanban-column">
            <div class="column-header">
              <h3>{statusLabels[status]}</h3>
              <span class="count">{statusTasks.length}</span>
            </div>
            <div class="column-tasks">
              {#each statusTasks as task (task.id)}
                <TaskCard
                  {task}
                  onclick={() => openTaskDetail(task)}
                  ondelete={handleDeleteTask}
                />
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<CreateTaskModal
  open={showCreateModal}
  onclose={() => showCreateModal = false}
  oncreate={handleCreateTask}
/>

<TaskDetailModal
  open={showDetailModal}
  task={selectedTask}
  runs={selectedTaskRuns}
  onclose={() => { showDetailModal = false; selectedTask = null; }}
  onassign={handleAssignAgent}
  onstatuschange={handleStatusChange}
/>

<style>
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 2rem;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .back-link {
    display: inline-block;
    margin-bottom: 0.5rem;
    color: var(--text-muted);
    font-size: 0.875rem;
  }

  .back-link:hover {
    color: var(--accent);
  }

  h1 {
    font-size: 1.75rem;
    margin-bottom: 0.25rem;
  }

  .deployment-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .deployment-status {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    border-radius: 9999px;
    background: var(--bg-tertiary);
    color: var(--text-muted);
  }

  .deployment-status.deployed {
    background: rgba(34, 197, 94, 0.2);
    color: var(--success);
  }

  .deployment-status.deploying {
    background: rgba(59, 130, 246, 0.2);
    color: var(--accent);
  }

  .deployment-url {
    font-size: 0.875rem;
    color: var(--accent);
  }

  .error-banner {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--error);
    color: var(--error);
    padding: 1rem;
    border-radius: var(--radius);
    margin-bottom: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .error-banner button {
    background: none;
    border: none;
    color: var(--error);
    cursor: pointer;
  }

  .kanban {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 1rem;
    overflow-x: auto;
    padding-bottom: 1rem;
  }

  @media (max-width: 1400px) {
    .kanban {
      grid-template-columns: repeat(3, minmax(250px, 1fr));
    }
  }

  @media (max-width: 900px) {
    .kanban {
      grid-template-columns: repeat(2, minmax(250px, 1fr));
    }
  }

  .kanban-column {
    background: var(--bg-secondary);
    border-radius: var(--radius);
    padding: 1rem;
    min-height: 400px;
  }

  .column-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border);
  }

  .column-header h3 {
    font-size: 0.875rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
  }

  .count {
    background: var(--bg-tertiary);
    padding: 0.125rem 0.5rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .column-tasks {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
