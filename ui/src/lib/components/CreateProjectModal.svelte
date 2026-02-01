<script lang="ts">
  import Modal from './Modal.svelte';

  let {
    open = false,
    onclose,
    oncreate
  }: {
    open: boolean;
    onclose: () => void;
    oncreate: (data: { name: string; github_url: string; default_branch: string }) => void;
  } = $props();

  let name = $state('');
  let github_url = $state('');
  let default_branch = $state('main');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name || !github_url) return;

    loading = true;
    try {
      await oncreate({ name, github_url, default_branch });
      name = '';
      github_url = '';
      default_branch = 'main';
      onclose();
    } finally {
      loading = false;
    }
  }
</script>

<Modal {open} title="Create Project" {onclose}>
  <form onsubmit={handleSubmit}>
    <div class="form-group">
      <label for="name">Project Name</label>
      <input type="text" id="name" bind:value={name} placeholder="My Project" required />
    </div>

    <div class="form-group">
      <label for="github_url">GitHub URL</label>
      <input type="url" id="github_url" bind:value={github_url} placeholder="https://github.com/user/repo" required />
    </div>

    <div class="form-group">
      <label for="default_branch">Default Branch</label>
      <input type="text" id="default_branch" bind:value={default_branch} placeholder="main" />
    </div>
  </form>

  {#snippet footer()}
    <button class="btn btn-secondary" onclick={onclose} disabled={loading}>Cancel</button>
    <button class="btn btn-primary" onclick={handleSubmit} disabled={loading || !name || !github_url}>
      {loading ? 'Creating...' : 'Create Project'}
    </button>
  {/snippet}
</Modal>
