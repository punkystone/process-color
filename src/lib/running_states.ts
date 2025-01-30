import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { writable } from "svelte/store";

export const runningStates = writable<boolean[]>([]);
let listener: UnlistenFn | null = null;

export const initRunningStates = async () => {
    listener = await listen<boolean[]>("running_states", (event) => {
        runningStates.set(event.payload);
    });
};

export const stopRunningStates = () => {
    listener?.();
    listener = null;
};




