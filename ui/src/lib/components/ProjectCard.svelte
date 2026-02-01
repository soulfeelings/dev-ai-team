<script lang="ts">
  import type { Project } from '$lib/api/types';

  let { project, ondelete }: { project: Project; ondelete?: (id: string) => void } = $props();

  function formatDate(dateStr: string) {
    return new Date(dateStr).toLocaleDateString();
  }

  function handleDelete(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (confirm('Delete this project?') && ondelete) {
      ondelete(project.id);
    }
  }
</script>

<a href="/projects/{project.id}" class="card project-card">
  <div class="project-header">
    <h3>{project.name}</h3>
    <button class="btn btn-danger btn-sm" onclick={handleDelete}>Delete</button>
  </div>
  <p class="text-secondary text-sm">{project.github_url}</p>
  <div class="project-meta">
    <span class="badge badge-backlog">{project.default_branch}</span>
    <span class="text-muted text-sm">Created {formatDate(project.created_at)}</span>
  </div>
</a>

<style>
  .project-card {
    display: block;
    transition: transform 0.2s, border-color 0.2s;
  }

  .project-card:hover {
    transform: translateY(-2px);
    border-color: var(--accent);
    text-decoration: none;
  }

  .project-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.5rem;
  }

  .project-header h3 {
    color: var(--text-primary);
    font-size: 1.125rem;
  }

  .project-meta {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-top: 1rem;
  }
</style>
