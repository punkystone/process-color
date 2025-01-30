<script lang="ts">
    import IconButton from "./IconButton.svelte";
    import DeleteIcon from "./icons/DeleteIcon.svelte";
    import EditIcon from "./icons/EditIcon.svelte";
    import SaveIcon from "./icons/SaveIcon.svelte";
    import { runningStates } from "./running_states";
    import Status from "./Status.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { ProcessEntryWithIndex } from "./types";

    let {
        index,
        name,
        topic,
        value,
        offValue,
        deleteEntry,
    }: ProcessEntryWithIndex = $props();
    let processes: string[] = $state([]);
    let isEditing: boolean = $state(false);

    const loadProcesses = async () => {
        const newProcesses = (await invoke("get_processes")) as string[];
        processes = newProcesses.filter((newProcess) => newProcess !== name);
    };
    const selectProcess = (e: Event) => {
        const selected = e.target as HTMLSelectElement;
        if (selected.value === name) return;
        name = selected.value;
    };

    const toggleEditing = () => {
        if (isEditing) {
            invoke("update_process_entry", {
                index: index,
                name: name,
                topic: topic.trim(),
                value: value.trim(),
                offValue: offValue.trim(),
            });
        }
        isEditing = !isEditing;
    };

    const deleteProcessEntry = async () => {
        await invoke("delete_process_entry", {
            index: index,
        });
        deleteEntry();
    };
</script>

<div class="process-entry">
    <div class="status">
        <Status active={$runningStates[index] ?? null} />
    </div>
    <select
        name="options"
        onclick={loadProcesses}
        onchange={selectProcess}
        disabled={!isEditing}
        class="flex-grow"
    >
        <option value={name}>{name}</option>
        {#each processes as option (option)}
            <option value={option}>{option}</option>
        {/each}
    </select>

    <input
        type="text"
        bind:value={topic}
        disabled={!isEditing}
        class="flex-grow"
        placeholder="Topic"
    />

    <input
        type="text"
        bind:value
        disabled={!isEditing}
        class="flex-grow"
        placeholder="On Value"
    />

    <input
        type="text"
        bind:value={offValue}
        disabled={!isEditing}
        class="flex-grow"
        placeholder="Off Value"
    />
    {#snippet editIcon()}
        <EditIcon />
    {/snippet}

    {#snippet saveIcon()}
        <SaveIcon />
    {/snippet}
    {#snippet deleteIcon()}
        <DeleteIcon />
    {/snippet}

    <IconButton
        icon={isEditing ? saveIcon : editIcon}
        onClick={toggleEditing}
    />
    <IconButton
        icon={deleteIcon}
        onClick={deleteProcessEntry}
        color="#bf0000"
    />
</div>

<style>
    .process-entry {
        display: flex;
        align-items: center;
        margin-bottom: 10px;
        column-gap: 10px;
    }

    .flex-grow {
        flex: 1;
    }
    select {
        height: 35px;
        width: 100%;
        background-color: #a9a9a9a9;
        color: white;
        border-radius: 5px;
        font-size: 17px;
        box-shadow: 0 6px 5px 0 #00000023;
    }

    option {
        background: #666666a9 !important;
        color: white !important;
    }

    input {
        height: 35px;
        padding: 0 10px;
        border-radius: 4px;
        background-color: #a9a9a9a9;
        color: white;
        border: none;
        border-radius: 5px;
        font-size: 17px;
        box-shadow: 0 6px 5px 0 #00000023;
        cursor: pointer;
    }
    input:disabled {
        background-color: #686868;
        color: #b2b2b2;
    }
    input:focus {
        outline-color: #0095ff;
    }
    ::placeholder {
        color: #9b9b9b;
    }
</style>
