<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { VirtualGrid } from "@layout";
  import { changeGridsId } from "@stores/Modals";
  import { dbFilters, libraryGridType, loadedLibrary, systems } from "@stores/State";
  import { filterGrids, GRID_LAYOUTS } from "@utils";
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

          if (system.thumbCapsulePath === "") {
            const grids = await SGDBController.getCapsulesForGame(system.sgdbId);
            const filtered = filterGrids(grids, $dbFilters["Capsule"]);
            
            if (filtered.length) {
              const first = filtered[0];
              const images = await RestController.cacheCapsule(first.url.toString(), first.thumb.toString(), system.abbreviation);
              
              system.thumbCapsulePath = images[0];
              system.fullCapsulePath = images[1];
            } else {
              system.thumbCapsulePath = "No Grids";
              system.fullCapsulePath = "No Grids";
            }

            saveMetadata = true;
          }
          
          if (system.heroPath === "") {
            const grids = await SGDBController.getHeroesForGame(system.sgdbId);
            const filtered = filterGrids(grids, $dbFilters["Hero"]);
            
            if (filtered.length) {
              const first = filtered[0];
              const image = await RestController.cacheHero(first.url.toString(), system.abbreviation);
              
              system.heroPath = image;
            } else {
              system.heroPath = "No Grids";
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
    keyFunction={(entry) => `${entry.index}|${entry.data}|${entry.data === $changeGridsId}`}
  >
    {#snippet row(entry)}
      {#if loadedKeys[entry]}
        <System abbreviation={entry} />
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