<script lang="ts">
  import { onDestroy, onMount, type Snippet } from "svelte";
  let { title, children }: { title: string; children?: Snippet } = $props();
  let isOpen = $state(true);

</script>

<div id="overlay">
  <dialog class="modal shadow" open={isOpen}>
    <h2>{title}</h2>
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
    background-color: hsla(0 0% 0% / 0.2);
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
  }
</style>
