<script lang="ts">
    let {
        active,
    }: {
        active: boolean | null;
    } = $props();
</script>

<span
    class="status"
    class:running={active === true}
    class:stopped={active === false}
    class:pending={active === null}
></span>

<style>
    .status {
        height: 15px;
        width: 15px;
        border-radius: 50%;
        display: inline-block;
        box-shadow: 0 6px 5px 0 #00000023;
    }
    .status.running {
        background-color: green;
    }
    .status.stopped {
        background-color: red;
    }
    .status.pending {
        background-color: cornflowerblue;
    }
</style>
