<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import markdownit from "markdown-it";
  import { onDestroy } from "svelte";

  const md = markdownit();

  let name = $state("");
  let mode: "edit" | "read" = $state("read");
  let rawText: string = $state("");
  let parsedText = $derived(md.render(rawText));
  $inspect(mode);

  let editor: HTMLTextAreaElement | null = $state(null);
  $inspect(editor);

  const debugSaveBtn = () => {
    toggleEdit("read");
    invoke('save_markdown_file', { text: rawText });
  };

  const toggleEdit = (newMode: "edit" | "read") => {
    mode = newMode;
    
    if (newMode === "read" && editor !== null) editor.focus();
  };

  onDestroy(() => {
    
  })
</script>

<main class="container">
  <header class="editor-header">
    <h1>Page Title</h1>
    <h2>Date dd/mm/yyyy</h2>
    <div>
      <button onclick={() => debugSaveBtn()}>Save</button>
    </div>
  </header>
  <section id="editor-section" class="editor-container">
    {#if mode === "edit"}
      <textarea
        onblur={() => toggleEdit("read")}
        id="editor"
        class="text-field"
        bind:value={rawText}
        bind:this={editor}
      >
      </textarea>
    {:else if mode === "read"}
      <button
        class="text-field display-text"
        onclick={() => toggleEdit("edit")}
      >
        <span>{@html parsedText}</span>
      </button>
    {/if}
  </section>
</main>

<style>
  .container {
    margin: 0;
    max-height: 100%;
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    justify-content: center;
    background-color: var(--bg-main);
  }

  .editor-header {
    margin: 0 1em;
  }

  .editor-header > h2 {
    font-size: small;
    font-weight: normal;
    color: var(--text-tertiary);
  }

  .editor-container {
    margin-top: 1em;
    border-top: solid 1px var(--bg-main-hover);
    display: flex;
    flex-grow: 1;
  }

  .text-field {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: var(--text-secondary);
    font-size: medium;
    background-color: var(--bg-main);
    flex-grow: 1;
    max-width: 100%;
    border: none;
    background-color: transparent;
    resize: none;
    outline: none;
    resize: none;
    margin: 0;
    padding: 1em;
    text-align: left;
  }

  .display-text {
    overflow-y: scroll;
    display: flex;
    flex-direction: column;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }
</style>
