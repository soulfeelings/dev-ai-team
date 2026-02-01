<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    open = false,
    title = '',
    onclose,
    children,
    footer
  }: {
    open: boolean;
    title: string;
    onclose: () => void;
    children: Snippet;
    footer?: Snippet;
  } = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      onclose();
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === 'Enter' && handleBackdropClick(e as unknown as MouseEvent)}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal">
      <div class="modal-header">
        <h2>{title}</h2>
      </div>
      <div class="modal-body">
        {@render children()}
      </div>
      {#if footer}
        <div class="modal-footer">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}
