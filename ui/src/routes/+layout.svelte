<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<div class="app">
  <header>
    <nav class="container">
      <a href="/" class="logo">AI Dev Team</a>
      <div class="nav-links">
        <a href="/">Projects</a>
      </div>
    </nav>
  </header>

  <main>
    {@render children()}
  </main>
</div>

<style>
  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  header {
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    padding: 1rem 0;
  }

  nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .logo {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .logo:hover {
    text-decoration: none;
  }

  .nav-links {
    display: flex;
    gap: 1.5rem;
  }

  .nav-links a {
    color: var(--text-secondary);
    transition: color 0.2s;
  }

  .nav-links a:hover {
    color: var(--text-primary);
    text-decoration: none;
  }

  main {
    flex: 1;
    padding: 2rem 0;
  }
</style>
