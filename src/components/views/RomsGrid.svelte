<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { VirtualGrid } from "@layout";
  import { changeGridsId } from "@stores/Modals";
  import { dbFilters, libraryGridType, loadedLibrary, romMetadata } from "@stores/State";
  import { filterGrids, GRID_LAYOUTS } from "@utils";
  import Rom from "@views/library/Rom.svelte";
  import RomLoadingSkeleton from "@views/library/RomLoadingSkeleton.svelte";
  import { onDestroy, onMount } from "svelte";
  import type { Unsubscriber } from "svelte/store";

  type Props = {
    gridName: string;
    romIds: string[];
    isLoading?: boolean;
  }
  
  let { gridName, romIds, isLoading = $bindable(true) }: Props = $props();
  
  
  let libraryLoadUnsub: Unsubscriber;
  
  const layout = $derived(GRID_LAYOUTS[$libraryGridType]);
  let loadedKeys = $state<Record<string, boolean>>({});

  onMount(() => {
    libraryLoadUnsub = loadedLibrary.subscribe(async (loaded) => {
      if (loaded) {
        let saveMetadata = false;

        for (const romId of romIds) {
          const metadata = $romMetadata[romId];

          if (metadata.thumbCapsulePath === "") {
            const grids = await SGDBController.getCapsulesForGame(metadata.sgdbId);
            const filtered = filterGrids(grids, $dbFilters["Capsule"]);
            
            if (filtered.length) {
              const first = filtered[0];
              const images = await RestController.cacheCapsule(first.url.toString(), first.thumb.toString(), romId);
              
              metadata.thumbCapsulePath = images[0];
              metadata.fullCapsulePath = images[1];
            } else {
              metadata.thumbCapsulePath = "No Grids";
              metadata.fullCapsulePath = "No Grids";
            }

            $romMetadata[romId] = metadata;
            saveMetadata = true;
          }
          
          if (metadata.heroPath === "") {
            const grids = await SGDBController.getHeroesForGame(metadata.sgdbId);
            const filtered = filterGrids(grids, $dbFilters["Hero"]);
            
            if (filtered.length) {
              const first = filtered[0];
              const image = await RestController.cacheHero(first.url.toString(), romId);
              
              metadata.heroPath = image;
            } else {
              metadata.heroPath = "No Grids";
            }

            $romMetadata[romId] = metadata;
            saveMetadata = true;
          }

          loadedKeys[romId] = true;
        }

        if (saveMetadata) {
          $romMetadata = { ...$romMetadata };
        }

        isLoading = false;
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
    keyFunction={(entry) => `${entry.index}|${entry.data}|${entry.data === $changeGridsId}`}
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