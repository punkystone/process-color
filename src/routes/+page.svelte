<script lang="ts">
  import type { ProcessEntry } from "../lib/types";
  import ProcessEntryComponent from "../lib/ProcessEntryComponent.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import { initRunningStates, stopRunningStates } from "$lib/running_states";
  import Button from "$lib/Button.svelte";
  import { goto } from "$app/navigation";
  import AddIcon from "$lib/icons/AddIcon.svelte";
  import IconButton from "$lib/IconButton.svelte";

  let processEntrys: ProcessEntry[] = $state([]);
  onMount(async () => {
    processEntrys = await invoke("get_process_entrys");
    initRunningStates();
  });

  onDestroy(() => {
    stopRunningStates();
  });
  const add = async () => {
    await invoke("add_process_entry");
    processEntrys = await invoke("get_process_entrys");
  };
  const deleteEntry = async () => {
    processEntrys = await invoke("get_process_entrys");
  };
</script>

<div class="mqtt">
  <Button
    label="Settings"
    onClick={() => {
      goto("/settings");
    }}
  />
</div>
<main class="container">
  {#each processEntrys as processEntry, i (processEntry)}
    <ProcessEntryComponent
      index={i}
      name={processEntry.name}
      topic={processEntry.topic}
      value={processEntry.value}
      offValue={processEntry.off_value}
      {deleteEntry}
    />
  {/each}

  {#snippet addIcon()}
    <AddIcon />
  {/snippet}
  <div class="add">
    <IconButton icon={addIcon} onClick={add} />
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;

    color: #ffffff;
    background-color: #2e2e2e;
    line-height: 1.5;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
  .add {
    display: flex;
    justify-content: center;
    padding-top: 15px;
  }
  .container {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
  .mqtt {
    display: flex;
    justify-content: end;
    margin-bottom: 20px;
  }
</style>
