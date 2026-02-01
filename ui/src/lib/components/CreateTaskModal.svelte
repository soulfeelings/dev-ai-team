<script lang="ts">
  import Modal from './Modal.svelte';
  import type { CreateTaskRequest } from '$lib/api/types';

  let {
    open = false,
    onclose,
    oncreate
  }: {
    open: boolean;
    onclose: () => void;
    oncreate: (data: CreateTaskRequest) => void;
  } = $props();

  let title = $state('');
  let description = $state('');
  let acceptance_criteria = $state('');
  let priority = $state(0);
  let branch_name = $state('');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!title) return;

    loading = true;
    try {
      await oncreate({
        title,
        description: description || undefined,
        acceptance_criteria: acceptance_criteria || undefined,
        priority: priority || undefined,
        branch_name: branch_name || undefined
      });
      // Reset form
      title = '';
      description = '';
      acceptance_criteria = '';
      priority = 0;
      branch_name = '';
      onclose();
    } finally {
      loading = false;
    }
  }
</script>

<Modal {open} title="Create Task" {onclose}>
  <form onsubmit={handleSubmit}>
    <div class="form-group">
      <label for="title">Title</label>
      <input type="text" id="title" bind:value={title} placeholder="Implement login form" required />
    </div>

    <div class="form-group">
      <label for="description">Description (optional)</label>
      <textarea id="description" bind:value={description} placeholder="Detailed description of the task..."></textarea>
    </div>

    <div class="form-group">
      <label for="acceptance_criteria">Acceptance Criteria (optional)</label>
      <textarea id="acceptance_criteria" bind:value={acceptance_criteria} placeholder="- User can enter email and password&#10;- Form validates input&#10;- Shows error messages"></textarea>
    </div>

    <div class="form-group">
      <label for="priority">Priority (optional, higher = more important)</label>
      <input type="number" id="priority" bind:value={priority} min="0" />
    </div>

    <div class="form-group">
      <label for="branch_name">Branch Name (optional)</label>
      <input type="text" id="branch_name" bind:value={branch_name} placeholder="feature/login-form" />
    </div>
  </form>

  {#snippet footer()}
    <button class="btn btn-secondary" onclick={onclose} disabled={loading}>Cancel</button>
    <button class="btn btn-primary" onclick={handleSubmit} disabled={loading || !title}>
      {loading ? 'Creating...' : 'Create Task'}
    </button>
  {/snippet}
</Modal>
