<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let currentPath: string = $state("");
  let isPathValid: boolean = $state(false);

  $inspect(currentPath);

  $effect(() => {
    updatePathValid();
  });

  onMount(async () => {
    // get current base directory
    const storedPath: string = await invoke("get_base_dir", {});
    if (storedPath.length !== 0) currentPath = storedPath;
  });

  const updatePathValid = async () => {
    try {
      const doesDirectoryExist: boolean = await invoke("dir_exists", {
        path: currentPath,
      });
      isPathValid = doesDirectoryExist;
    } catch (error) {
      isPathValid = false;
    }
  };

  const chooseBaseDir = async () => {
    const newDir = await open({
      multiple: false,
      directory: true,
    });
    if (newDir !== null) {
      console.log("Path updated...");
      currentPath = newDir.toString();
      updatePathValid();
    }
  };

  const setBaseDir = async () => {
    console.log("Saving new directory path...");
    try {
      const set: string = await invoke("set_base_dir", {
        dirStringToParse: currentPath,
      });
      console.log("New path set:", await set);
    } catch (error) {
      isPathValid = false;
      console.error(error);
    }
  };

</script>

<form onsubmit={async () => setBaseDir()}>
  <fieldset class="dir-group" name="set-dir-path">
    <legend>Directory Path</legend>
    <button onclick={() => chooseBaseDir()}>Open directory</button>
    <input
      class={"dir-input " + [!isPathValid && "border-danger"]}
      type="text"
      autocorrect="off"
      bind:value={currentPath}
      placeholder="Directory path..."
    />
  </fieldset>
  <br />
  <input value="Save path" type="submit" disabled={!isPathValid} />
</form>

<style scoped>
  .dir-group {
    display: flex;
    width: 100%;
    gap: 0.5em;
  }

  .dir-group > legend {
    margin-bottom: 0.5em;
  }

  .dir-input {
    flex-grow: 1;
    font-size: medium;
    border-radius: var(--rounding);
    padding: 0.5em;
    color: var(--text-primary);
    background-color: var(--bg-secondary);
    border: solid 1px var(--border-main);
  }
</style>
