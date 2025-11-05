<script lang="ts">
  import type { LayoutProps } from "./$types";
  import '../main.css'

  const { children }: LayoutProps = $props();
  const notes: { title: String }[] = $state([
    { title: "Note 1" },
    { title: "Note 2" },
    { title: "Note 3" },
  ]);
</script>

<nav class="file-menu">
  {#each notes as note}
    <button class="file-selector">{note.title}</button>
  {/each}
</nav>

{@render children()}

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    margin: 0;

    color: var(--text-primary);
    background-color: var(--bg-main);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;

    height: 100%;
    width: 100%;
  }

  :global(body) {
    max-width: 100%;
    min-height: 100%;
    display: flex;
    margin: 0;
  }

  .file-menu {
    display: flex;
    flex-direction: column;
    width: 15em;
    padding: 1em;
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--bg-main-hover);
    gap: 0.25em;
  }

  .file-selector {
    font-size: medium;
    text-align: left;
    color: var(--text-primary);
    width: 100%;
    background-color: var(--bg-secondary);
    border-radius: 0.25em;
    border: none;
    cursor: pointer;
    padding: 0.5em;
    transition: background-color 100ms ease-out;
  }

  .file-selector:hover {
    background-color: var(--bg-main-hover);
  }
</style>
