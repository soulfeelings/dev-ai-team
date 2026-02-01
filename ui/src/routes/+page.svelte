<script lang="ts">
  import { onMount } from 'svelte';
  import { projects } from '$lib/stores/projects';
  import ProjectCard from '$lib/components/ProjectCard.svelte';
  import CreateProjectModal from '$lib/components/CreateProjectModal.svelte';

  let showCreateModal = $state(false);
  let loading = $state(true);
  let error = $state('');

  onMount(async () => {
    try {
      await projects.load();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load projects';
    } finally {
      loading = false;
    }
  });

  async function handleCreate(data: { name: string; github_url: string; default_branch: string }) {
    try {
      await projects.add(data);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to create project';
      throw e;
    }
  }

  async function handleDelete(id: string) {
    try {
      await projects.remove(id);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to delete project';
    }
  }
</script>

<div class="container">
  <div class="page-header">
    <h1>Projects</h1>
    <button class="btn btn-primary" onclick={() => showCreateModal = true}>
      + New Project
    </button>
  </div>

  {#if error}
    <div class="error-banner">
      {error}
      <button onclick={() => error = ''}>Dismiss</button>
    </div>
  {/if}

  {#if loading}
    <div class="empty-state">Loading projects...</div>
  {:else if $projects.length === 0}
    <div class="empty-state">
      <p>No projects yet</p>
      <button class="btn btn-primary mt-2" onclick={() => showCreateModal = true}>
        Create your first project
      </button>
    </div>
  {:else}
    <div class="grid grid-2">
      {#each $projects as project (project.id)}
        <ProjectCard {project} ondelete={handleDelete} />
      {/each}
    </div>
  {/if}
</div>

<CreateProjectModal
  open={showCreateModal}
  onclose={() => showCreateModal = false}
  oncreate={handleCreate}
/>

<style>
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  h1 {
    font-size: 1.75rem;
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
</style>
