<script lang="ts">
    let {
        onClick,
        label,
        color,
    }: {
        onClick: () => void;
        label: string;
        color?: string;
    } = $props();
</script>

<button style:background-color={color} onclick={onClick}>{label}</button>

<style>
    button {
        display: inline-block;
        border-radius: 4px;
        background-color: #455362;
        border: none;
        color: #ffffff;
        text-align: center;
        font-size: 18px;
        padding: 10px;
        cursor: pointer;
        box-shadow: 0 6px 5px 0 #00000023;
    }
</style>
