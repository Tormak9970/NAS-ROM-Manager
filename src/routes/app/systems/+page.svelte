<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { VirtualGrid } from "@layout";
  import { changeCoverId } from "@stores/Modals";
  import { dbFilters, libraryGridType, loadedLibrary, systems } from "@stores/State";
  import { filterGrids, GRID_LAYOUTS, systemToParser } from "@utils";
  import RomLoadingSkeleton from "@views/library/RomLoadingSkeleton.svelte";
  import System from "@views/systems/System.svelte";
  import { onDestroy, onMount } from "svelte";
  import type { Unsubscriber } from "svelte/store";

  let libraryLoadUnsub: Unsubscriber;

  const layout = $derived(GRID_LAYOUTS[$libraryGridType]);
  const systemNamesList = $derived(Object.keys($systems).sort());

  let systemsLoaded = $state(0);

  let loadedKeys = $state<Record<string, boolean>>({});

  onMount(() => {
    libraryLoadUnsub = loadedLibrary.subscribe(async (loaded) => {
      if (loaded) {
        let saveMetadata = false;

        for (const systemName of systemNamesList) {
          const system = $systems[systemName];

          if (system.thumbPath === "") {
            const grids = await SGDBController.getCoversForGame(system.sgdbId);
            const filtered = filterGrids(grids, $dbFilters);
            
            if (filtered.length) {
              const first = filtered[0];
              const images = await RestController.cacheCover(first.url.toString(), first.thumb.toString(), systemToParser(systemName));
              
              system.thumbPath = images[0];
              system.coverPath = images[1];
            } else {
              system.thumbPath = "No Grids";
              system.coverPath = "No Grids";
            }

            saveMetadata = true;
          }

          systemsLoaded++;
          loadedKeys[systemName] = true;
        }

        if (saveMetadata) {
          $systems = { ...$systems };
        }
      }
    });
  });

  onDestroy(() => {
    if (libraryLoadUnsub) libraryLoadUnsub();
  });
</script>

<svelte:head>
	<title>Systems - NRM</title>
  <meta name="description" content="Game systems in your library." />
</svelte:head>

<div id="systems">
  <VirtualGrid
    name="systems"
    itemHeight={layout.height} 
    itemWidth={layout.width}
    items={systemNamesList}
    columnGap={layout.gap}
    rowGap={layout.gap}
    keyFunction={(entry) => `${entry.index}|${entry.data}|${entry.data === $changeCoverId}`}
  >
    {#snippet row(entry)}
      {#if loadedKeys[entry]}
        <System systemName={entry} />
      {:else}
        <RomLoadingSkeleton />
      {/if}
    {/snippet}
  </VirtualGrid>
</div>

<style>
  #systems {
    width: 100%;
    height: 100%;
  }
</style>