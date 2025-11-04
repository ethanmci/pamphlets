<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import markdownit from "markdown-it";
  import { onMount } from "svelte";

  const md = markdownit();

  interface textPortion {
    rawText: string;
  }

  let name = $state("");
  let mode: "edit" | "read" = $state("read");
  let rawText: string = $state("");
  $inspect(mode);

  let editor: HTMLTextAreaElement | null = $state(null);
  $inspect(editor);

  const editorOnBlur = () => {
    console.log("editor blurred!");
    if (mode !== "read") mode = "read";
  };

  const toggleEdit = (newMode: "edit" | "read") => {
    mode = newMode;
    
    if (newMode === "read" && editor !== null) editor.focus();
  };
</script>

<main class="container">
  <header class="editor-header">
    <h1>Page Title</h1>
    <div>
      <button onclick={() => toggleEdit("read")}>Save</button>
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
        <span>{rawText}</span>
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
    background-color: var(--bg-main-colour);
  }

  .editor-header {
    margin: 0 1em;
  }

  .editor-container {
    margin-top: 1em;
    border-top: solid 1px var(--bg-main-colour-hover);
    display: flex;
    flex-grow: 1;
  }

  .text-field {
    color: var(--text-light);
    font-size: medium;
    background-color: var(--bg-main-colour);
    flex-grow: 1;
    max-width: 100%;
    border: none;
    background-color: transparent;
    resize: none;
    outline: none;
    resize: none;
    margin: 0;
    padding: 1em;
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
