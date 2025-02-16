<script lang="ts">
  import { VirtualGrid } from "@layout";
  import { libraryGridType, roms } from "@stores/State";
  import { GRID_LAYOUTS } from "@utils";
  import Rom from "@views/library/Rom.svelte";

  let layout = $derived(GRID_LAYOUTS[$libraryGridType]);

  let romsList = $derived(Object.values($roms));
</script>

<svelte:head>
	<title>Library</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<div id="library">
  <VirtualGrid
    name="library"
    itemHeight={layout.height} 
    itemWidth={layout.width}
    items={romsList}
    columnGap={layout.gap}
    rowGap={layout.gap}
    let:entry
  >
    <Rom rom={entry} />
  </VirtualGrid>
</div>

<style>
  #library {
    width: 100%;
    height: 100%;
  }
</style>