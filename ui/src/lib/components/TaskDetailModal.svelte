<script lang="ts">
  import Modal from './Modal.svelte';
  import type { Task, AgentRun, AgentRole, TaskStatus } from '$lib/api/types';

  let {
    open = false,
    task,
    runs = [],
    onclose,
    onassign,
    onstatuschange
  }: {
    open: boolean;
    task: Task | null;
    runs: AgentRun[];
    onclose: () => void;
    onassign: (role: AgentRole) => void;
    onstatuschange: (status: TaskStatus) => void;
  } = $props();

  let selectedRole: AgentRole = $state('Dev');
  let assigning = $state(false);

  const roles: AgentRole[] = ['Planner', 'Dev', 'QA', 'Reviewer'];
  const statuses: TaskStatus[] = ['Backlog', 'InProgress', 'QA', 'Review', 'Done', 'Failed'];

  async function handleAssign() {
    assigning = true;
    try {
      await onassign(selectedRole);
    } finally {
      assigning = false;
    }
  }

  function formatDate(dateStr: string) {
    return new Date(dateStr).toLocaleString();
  }

  function getStatusClass(status: string): string {
    return `badge-${status.toLowerCase()}`;
  }
</script>

<Modal {open} title={task?.title || 'Task Details'} {onclose}>
  {#if task}
    <div class="task-detail">
      <div class="section">
        <h4>Status</h4>
        <select class="status-select" value={task.status} onchange={(e) => onstatuschange(e.currentTarget.value as TaskStatus)}>
          {#each statuses as status}
            <option value={status}>{status}</option>
          {/each}
        </select>
      </div>

      <div class="section">
        <h4>Description</h4>
        <p class="text-secondary">{task.description}</p>
      </div>

      <div class="section">
        <h4>Acceptance Criteria</h4>
        <pre class="criteria">{task.acceptance_criteria}</pre>
      </div>

      {#if task.branch_name}
        <div class="section">
          <h4>Branch</h4>
          <code>{task.branch_name}</code>
        </div>
      {/if}

      <div class="section">
        <h4>Assign Agent</h4>
        <div class="assign-row">
          <select bind:value={selectedRole}>
            {#each roles as role}
              <option value={role}>{role}</option>
            {/each}
          </select>
          <button class="btn btn-primary" onclick={handleAssign} disabled={assigning}>
            {assigning ? 'Assigning...' : 'Assign'}
          </button>
        </div>
      </div>

      {#if runs.length > 0}
        <div class="section">
          <h4>Agent Runs ({runs.length})</h4>
          <div class="runs-list">
            {#each runs as run}
              <div class="run-item">
                <div class="run-header">
                  <span class="badge {getStatusClass(run.status)}">{run.status}</span>
                  <span class="badge badge-backlog">{run.agent_role}</span>
                </div>
                <div class="run-meta text-sm text-muted">
                  Created: {formatDate(run.created_at)}
                  {#if run.completed_at}
                    <br>Completed: {formatDate(run.completed_at)}
                  {/if}
                </div>
                {#if run.error_message}
                  <div class="run-error text-sm">{run.error_message}</div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#snippet footer()}
    <button class="btn btn-secondary" onclick={onclose}>Close</button>
  {/snippet}
</Modal>

<style>
  .task-detail {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section h4 {
    font-size: 0.875rem;
    color: var(--text-muted);
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .criteria {
    background: var(--bg-primary);
    padding: 1rem;
    border-radius: var(--radius);
    white-space: pre-wrap;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  code {
    background: var(--bg-primary);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .assign-row {
    display: flex;
    gap: 0.5rem;
  }

  .assign-row select {
    flex: 1;
    padding: 0.5rem;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text-primary);
  }

  .status-select {
    padding: 0.5rem;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text-primary);
  }

  .runs-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .run-item {
    background: var(--bg-primary);
    padding: 0.75rem;
    border-radius: var(--radius);
  }

  .run-header {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .run-error {
    color: var(--error);
    margin-top: 0.5rem;
    padding: 0.5rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 4px;
  }
</style>
