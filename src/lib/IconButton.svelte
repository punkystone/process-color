<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        onClick,
        icon,
        color,
    }: {
        onClick: () => void;
        icon: Snippet;
        color?: string;
    } = $props();
</script>

<button style:background-color={color} onclick={onClick}
    >{@render icon()}</button
>

<style>
    button {
        display: inline-block;
        border-radius: 4px;
        background-color: #455362;
        border: none;
        color: #ffffff;
        text-align: center;
        font-size: 18px;
        width: 37px;
        height: 37px;
        padding: 5px;
        cursor: pointer;
        box-shadow: 0 6px 5px 0 #00000023;
    }
</style>
