<script lang="ts">
  import type { Task, TaskStatus } from '$lib/api/types';

  let {
    task,
    onclick,
    ondelete
  }: {
    task: Task;
    onclick?: () => void;
    ondelete?: (id: string) => void;
  } = $props();

  function getStatusClass(status: TaskStatus): string {
    return `badge-${status.toLowerCase()}`;
  }

  function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    if (confirm('Delete this task?') && ondelete) {
      ondelete(task.id);
    }
  }
</script>

<div class="card task-card" onclick={onclick} onkeydown={(e) => e.key === 'Enter' && onclick?.()} role="button" tabindex="0">
  <div class="task-header">
    <span class="badge {getStatusClass(task.status)}">{task.status}</span>
    <button class="btn btn-danger btn-sm" onclick={handleDelete}>Delete</button>
  </div>
  <h4>{task.title}</h4>
  <p class="text-secondary text-sm description">{task.description}</p>
  {#if task.branch_name}
    <div class="task-meta">
      <span class="text-muted text-sm">Branch: {task.branch_name}</span>
    </div>
  {/if}
</div>

<style>
  .task-card {
    cursor: pointer;
    transition: transform 0.2s, border-color 0.2s;
  }

  .task-card:hover {
    transform: translateY(-2px);
    border-color: var(--accent);
  }

  .task-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  h4 {
    margin-bottom: 0.5rem;
    font-size: 1rem;
  }

  .description {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .task-meta {
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border);
  }
</style>
