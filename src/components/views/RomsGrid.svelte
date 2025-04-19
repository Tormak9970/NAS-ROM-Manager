<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { VirtualGrid } from "@layout";
  import { changeCoverId } from "@stores/Modals";
  import { dbFilters, libraryGridType, loadedLibrary, romMetadata } from "@stores/State";
  import { filterGrids, GRID_LAYOUTS } from "@utils";
  import Rom from "@views/library/Rom.svelte";
  import RomLoadingSkeleton from "@views/library/RomLoadingSkeleton.svelte";
  import { onDestroy, onMount } from "svelte";
  import type { Unsubscriber } from "svelte/store";

  type Props = {
    gridName: string;
    romIds: string[];
  }
  
  let { gridName, romIds }: Props = $props();
  
  
  let libraryLoadUnsub: Unsubscriber;
  
  const layout = $derived(GRID_LAYOUTS[$libraryGridType]);
  let loadedKeys = $state<Record<string, boolean>>({});

  onMount(() => {
    libraryLoadUnsub = loadedLibrary.subscribe(async (loaded) => {
      if (loaded) {
        let saveMetadata = false;

        for (const romId of romIds) {
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

          loadedKeys[romId] = true;
        }

        if (saveMetadata) {
          $romMetadata = { ...$romMetadata };
        }
      }
    });
  });

  onDestroy(() => {
    if (libraryLoadUnsub) libraryLoadUnsub();
  });
</script>

<div class="roms-grid">
  <VirtualGrid
    name={gridName}
    itemHeight={layout.height} 
    itemWidth={layout.width}
    items={romIds}
    columnGap={layout.gap}
    rowGap={layout.gap}
    keyFunction={(entry) => `${entry.index}|${entry.data}|${entry.data === $changeCoverId}`}
  >
    {#snippet row(entry)}
      {#if loadedKeys[entry]}
        <Rom romId={entry} />
      {:else}
        <RomLoadingSkeleton />
      {/if}
    {/snippet}
  </VirtualGrid>
</div>

<style>
  .roms-grid {
    width: 100%;
    height: 100%;
  }
</style>