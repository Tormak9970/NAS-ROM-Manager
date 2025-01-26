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
  {rom.title}
</div>

<style>
  .rom {
    border: 1px solid rgb(var(--m3-scheme-outline));
    border-radius: var(--m3-util-rounding-medium);
  }
</style>