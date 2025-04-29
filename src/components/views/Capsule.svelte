<script lang="ts">
  import { CapsulePlaceholder } from "@layout";

  let { src }: { src: string } = $props();
  
  let failedToLoad = $state(false);
  
  let oldSrc = $state(src);
  let reload = $state(false);

  $effect(() => {
    if (src !== oldSrc) {
      oldSrc = src;

      fetch(src, { cache: 'reload' }).then(() => {
        reload = !reload;
      });
    }
  })
</script>

<div class="capsule">
  {#key `${src}|${reload}`}
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