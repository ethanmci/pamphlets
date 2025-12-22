<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface settingsVars {
    baseFilePath: string | undefined;
  }

  let settings: settingsVars = $state({
    baseFilePath: undefined
  })

  onMount(async () => {
    try {
      const storedFilePath: string = await invoke("get_base_dir", {});
      settings.baseFilePath = storedFilePath;
    } catch (error: unknown) {
      console.error(error);
    }
  });
</script>

<main class="container">
  <h1>Settings</h1>
  <h2>File System</h2>
  <h3>Base file path</h3>
  <section>
    {settings.baseFilePath}
    <button>Change file path</button>
  </section>
</main>

<style scoped>
  .container {
    margin: 0;
    height: 100%;
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    justify-content: center;
    background-color: var(--bg-main);
    padding: 2em;
  }
</style>
