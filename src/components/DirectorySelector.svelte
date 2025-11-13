<script lang="ts"> 
    import { open } from '@tauri-apps/plugin-dialog';
    import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
    let currentPath: string = $state('');
    let isPathValid: boolean = $state(false);

    $effect(() => {
        updatePathValid();
    })

    onMount(() => {
        // get current base directory 
        // currentPath = ....
    })

    const updatePathValid = async () => {
        try {
            const doesDirectoryExist: boolean = await invoke('dir_exists', { path: currentPath });
            isPathValid = doesDirectoryExist;
        } catch (error) {
            isPathValid = false;
        }
    }

    const chooseBaseDir = async () => {
        const newDir = await open({
            multiple: false,
            directory: true,
        });
        if (newDir !== null) {
            currentPath = newDir.toString();
            const res = await invoke('set_base_dir', { dirStringToParse: currentPath });
        }
    }

    const setBaseDir = async () => {
        console.log('Saving new directory path...');
        try {
            const set: boolean = await invoke('set_base_dir', { dir_string_to_parse: currentPath });
            isPathValid = doesDirectoryExist;
        } catch (error) {
            isPathValid = false;
        }
    }
</script>
<form onsubmit={() => setBaseDir()}>
    <fieldset class="dir-group" name="set-dir-path">
        <legend>Directory Path</legend>
        <button onclick={() => chooseBaseDir()}>Open directory</button>
        <input class={'dir-input ' + [!isPathValid && 'border-danger']} type="text" autocorrect="off" bind:value={currentPath} placeholder="Directory path...">
    </fieldset>
    <br/>
    <input value="Save path" type="submit" disabled={!isPathValid}>
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