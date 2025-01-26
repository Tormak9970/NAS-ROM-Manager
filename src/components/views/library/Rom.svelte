<script lang="ts">
  import { GRID_LAYOUTS } from "@utils";
  import { libraryGridType } from "@stores/State";
  import type { ROM } from "@types";

  type RomProps = {
    rom: ROM;
  }

  let { rom }: RomProps = $props();

  let layout = $derived(GRID_LAYOUTS[$libraryGridType]);
</script>

<div
  class="rom"
  style:width="{layout.width}px"
  style:height="{layout.height}px"
>
  <div class="cover">

  </div>
  <div class="title">{rom.title}</div>
</div>

<style>
  .rom {
    /* border: 1px solid rgb(var(--m3-scheme-outline)); */
    border-radius: var(--m3-util-rounding-medium);
  }

  .cover {
    width: calc(100% - 2px);
    height: calc(100% - 2px - 3rem);
    border: 1px solid rgb(var(--m3-scheme-outline));
    border-radius: var(--m3-util-rounding-medium);

    scale: 1;

    transition: scale 0.2s;
  }

  .rom:hover .cover {
    scale: 1.05;
  }

  .title {
    margin-top: 0.5rem;
    text-align: center;
  }
</style>