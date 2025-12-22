<script lang="ts">
  import type { LayoutProps } from "./$types";
  import "../main.css";
  import Modal from "../components/Modal.svelte";
  import DirectorySelector from "../components/DirectorySelector.svelte";
  import { setContext } from "svelte";

  type ModalTypes = "none" | "directory";
  let openModal: ModalTypes = $state("none");
  setContext("modalState", () => openModal);

  const openSettings = () => {
    openModal = "directory";
  };

  const closeModal = () => {
    openModal = "none";
  };

  const { children }: LayoutProps = $props();
  const notes: { title: String; date: Date }[] = $state([
    { title: "Note 1", date: new Date() },
  ]);
</script>

<section class="sidebar">
  <nav class="file-menu">
    {#each notes as note}
      <button class="file-selector">
        <span>{note.title}</span>
        <sub>{note.date.toLocaleDateString("en-US")}</sub>
      </button>
    {/each}
  </nav>
  <a href="./settings"><button class="settings-btn">Settings</button></a>
</section>

{#if (openModal as ModalTypes) == "directory"}
  <Modal title="Select a base directory" closeFn={closeModal}>
    <DirectorySelector></DirectorySelector>
  </Modal>
{/if}

{@render children()}

<style>
  :global(body) {
    max-width: 100%;
    min-height: 100%;
    display: flex;
    margin: 0;
  }

  .sidebar {
    display: flex;
    position: relative;
    flex-direction: column;
    width: 15em;
    padding: 1em;
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--bg-main-hover);
    gap: 0.25em;
  }

  .file-menu {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 0.25em;
    overflow-y: auto;
    flex-grow: 1;
  }

  .file-selector {
    display: flex;
    flex-direction: column;
    gap: 0.25em;
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

  .file-selector > span {
    font-weight: bold;
  }

  .file-selector > sub {
    color: var(--text-tertiary);
    font-size: smaller;
  }

  .settings-btn {
    justify-self: end;
  }

  a > .settings-btn {
    width: 100%;
  }
</style>
