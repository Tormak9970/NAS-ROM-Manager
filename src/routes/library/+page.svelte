<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { VirtualGrid } from "@layout";
  import { dbFilters, libraryGridType, romMetadata, roms } from "@stores/State";
  import { filterGrids, GRID_LAYOUTS } from "@utils";
  import Rom from "@views/library/Rom.svelte";
  import RomLoadingSkeleton from "@views/library/RomLoadingSkeleton.svelte";
  import { onMount } from "svelte";

  let layout = $derived(GRID_LAYOUTS[$libraryGridType]);

  let romIdList = $derived(Object.keys($roms));

  let romsLoaded = $state(0);

  let loadedKeys = $state<Record<string, boolean>>({});

  onMount(async () => {
    let saveMetadata = false;

    for (const romId of romIdList) {
      const metadata = $romMetadata[romId];

      if (metadata.thumbPath === "") {
        const grids = await SGDBController.getCoversForGame(metadata.sgdbId);
        const filtered = filterGrids(grids, $dbFilters);
        
        if (filtered.length) {
          const first = filtered[0];
          const images = await RestController.cacheCover(first.url.toString(), first.thumb.toString(), romId);
          
          metadata.thumbPath = images[0];
          metadata.coverPath = images[1];
        } else {
          metadata.thumbPath = "No Grids";
          metadata.coverPath = "No Grids";
        }

        $romMetadata[romId] = metadata;
        saveMetadata = true;
      }

      romsLoaded++;
      loadedKeys[romId] = true;
    }

    if (saveMetadata) {
      $romMetadata = { ...$romMetadata };
    }
  });
</script>

<svelte:head>
	<title>Library - NRM</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<div id="library">
  <VirtualGrid
    name="library"
    itemHeight={layout.height} 
    itemWidth={layout.width}
    items={romIdList}
    columnGap={layout.gap}
    rowGap={layout.gap}
    let:entry
  >
    {#if loadedKeys[entry]}
      <Rom romId={entry} />
    {:else}
      <RomLoadingSkeleton />
    {/if}
  </VirtualGrid>
</div>

<style>
  #library {
    width: 100%;
    height: 100%;
  }
</style>