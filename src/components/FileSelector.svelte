<script lang="ts"> 
    import { open } from '@tauri-apps/plugin-dialog';
    import { invoke } from "@tauri-apps/api/core";
    let activePath: string = $state('');
    let isPathValid = $derived.by(async (): Promise<boolean> => {
        try {
            const doesDirectoryExist: boolean = await invoke('dir_exists', { path: activePath });
            return doesDirectoryExist;
        } catch (error) {
            console.error(error);
            return false;
        }
    });
    $inspect(isPathValid);

    const setBaseDir = async () => {
        const newDir = await open({
            multiple: false,
            directory: true,
        });
        if (newDir !== null) {
            activePath = newDir.toString();
            const res = await invoke('set_base_dir', { dirStringToParse: activePath });
            console.log(res);
        }
    }
</script>
<form>
    <fieldset class="dir-group" name="set-dir-path">
        <legend>Directory Path</legend>
        <button onclick={() => setBaseDir()}>Open directory</button>
        <input class="dir-input" type="text" autocorrect="off" bind:value={activePath}>
    </fieldset>
    <input type="submit">
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