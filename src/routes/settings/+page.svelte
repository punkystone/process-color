<script lang="ts">
    import Button from "$lib/Button.svelte";
    import IconButton from "$lib/IconButton.svelte";
    import ExitIcon from "$lib/icons/ExitIcon.svelte";
    import SaveIcon from "$lib/icons/SaveIcon.svelte";
    import Status from "$lib/Status.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    let connected: boolean | null = $state(null);
    let listener: UnlistenFn | null = null;
    let ip: string | null = $state(null);
    let port: string | null = $state(null);
    let autostart: boolean | null = $state(null);

    onMount(async () => {
        listener = await listen<boolean>("mqtt_connection_state", (event) => {
            connected = event.payload;
        });
        const mqttSettings = (await invoke("get_mqtt_connection")) as {
            ip: string;
            port: string;
        };
        ip = mqttSettings.ip;
        port = mqttSettings.port;
        autostart = (await invoke("get_autostart")) as boolean;
    });
    onDestroy(() => {
        listener?.();
    });

    const reconnect = () => {
        invoke("mqtt_connect");
    };

    const openConfig = () => {
        invoke("open_config");
    };
    const saveMqtt = () => {
        invoke("save_mqtt_connection", {
            ip: ip,
            port: Number(port),
        });
    };
    const setAutoStart = async (enabled: boolean) => {
        await invoke("set_autostart", { enabled: enabled });
        autostart = (await invoke("get_autostart")) as boolean;
    };
</script>

<div class="main">
    <div class="container">
        <div class="container"></div>
        <div>
            <Status active={connected} />
        </div>
        <div class="spacing status">
            MQTT {connected === null
                ? "..."
                : connected
                  ? "Connected"
                  : "Disconnected"}
        </div>
        <a href="/" class="exit">
            <ExitIcon />
        </a>
    </div>
    <div class="mqtt-settings">
        <input type="text" bind:value={ip} />
        <input type="text" bind:value={port} />
        {#snippet saveIcon()}
            <SaveIcon />
        {/snippet}

        <IconButton icon={saveIcon} onClick={saveMqtt} />
    </div>

    <Button label="Reconnect" onClick={reconnect} />

    <hr />
    {#if autostart}
        <Button
            label="Disable Autostart"
            onClick={() => setAutoStart(false)}
            color="#bf0000"
        />
    {:else}
        <Button
            label="Enable Autostart"
            onClick={() => setAutoStart(true)}
            color="#0a5d00"
        />
    {/if}

    <hr />
    <Button label="Config" onClick={openConfig} />
</div>

<style>
    .main {
        margin: 15px;
    }
    .container {
        display: flex;
        align-items: center;
        margin-left: 2px;
        margin-bottom: 15px;
    }
    .spacing {
        margin-left: 10px;
    }
    .status {
        font-size: 18px;
        margin-right: auto;
    }
    .exit {
        color: #ffffff;
    }
    hr {
        margin: 20px 0;
    }
    .mqtt-settings {
        display: flex;
        align-items: center;
        margin-left: 2px;
        margin-bottom: 20px;
        column-gap: 10px;
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
</style>
