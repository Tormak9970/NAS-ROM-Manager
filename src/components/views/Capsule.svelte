<script lang="ts">
  import { CapsulePlaceholder } from "@layout";

  let { src }: { src: string } = $props();
  
  let failedToLoad = $state(true);
</script>

<div class="capsule">
  {#key src}
    {#if !failedToLoad && src !== "No Grids"}
      <img src={src} alt="Capsule placeholder" onerror={() => failedToLoad = true}>
    {:else}
      <CapsulePlaceholder />
    {/if}
  {/key}
</div>

<style>
  .capsule {
    position: absolute;
    top: 0;
    left: 0;

    width: 100%;
    height: 100%;
    border-radius: var(--m3-util-rounding-medium);
    overflow: hidden;
  }

  .capsule img {
    object-fit: cover;
    width: 100%;
    height: 100%;
  }
</style>