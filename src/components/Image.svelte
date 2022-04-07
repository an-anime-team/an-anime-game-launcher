<script lang="ts">
    export let src: string = '';
    export let width: number = 0;
    export let height: number = 0;
    export let orignialURI: string = '';

    export let click: () => void = () => {};
</script>

<!-- svelte-ignore a11y-img-redundant-alt -->
<img
    src={src}
    data-originalURI={orignialURI}
    width={width}
    height={height}
    alt="no image"
    on:click={click}
/>