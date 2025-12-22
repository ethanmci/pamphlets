<script lang="ts">
  import { onDestroy, onMount, type Snippet } from "svelte";
  let {
    title,
    closeFn,
    children,
  }: { title: string; closeFn: () => void; children?: Snippet } = $props();
  let isOpen = $state(true);
</script>

<div id="overlay">
  <dialog class="modal shadow" open={isOpen}>
    <section class="header">
      <h2 class="header-text">{title}</h2>
      <button class="close-btn" onclick={closeFn}>X</button>
    </section>
    <hr />
    {@render children?.()}
  </dialog>
</div>

<style scoped>
  @import "../main.css";

  hr {
    border-color: var(--bg-secondary);
    background-color: var(--bg-secondary);
  }

  #overlay {
    position: absolute;
    height: 100%;
    width: 100%;
    background-color: hsla(0 0% 0% / 0.4);
    display: flex;
    align-items: center;
    justify-items: center;
  }

  .modal {
    z-index: 90;
    background-color: var(--bg-secondary);
    color: var(--text-primary);
    border-radius: var(--rounding-lg);
    box-shadow: 20px;
    border: 1px solid var(--bg-main-hover);
    height: auto;
    min-width: 50%;
    max-width: 1200px;
    position: relative;
  }

  .header {
    display: flex;
    align-items: center;
  }

  .header > h1,
  h2 {
    flex-grow: 1;
  }
  .close-btn {
    height: 2.5em;
    width: 2.5em;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: larger;
  }

  .close-btn:hover {
    background-color: var(--danger);
  }
</style>
