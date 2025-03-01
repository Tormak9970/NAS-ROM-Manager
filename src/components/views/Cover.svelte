<script lang="ts">
  import { romMetadata } from "@stores/State";

  let { romId }: { romId: string } = $props();

  let metadata = $derived($romMetadata[romId]);
</script>

<div class="cover" style:--cover-url='url("{metadata.thumbPath === "No Grids" ? "" : metadata.thumbPath}")'></div>

<style>
  .cover {
    position: absolute;
    top: 0;
    left: 0;

    width: 100%;
    height: 100%;
    border-radius: var(--m3-util-rounding-medium);

    background-image: var(--cover-url);
    background-repeat: no-repeat;
    background-size: contain;
  }
</style>