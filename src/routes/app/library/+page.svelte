<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { VirtualGrid } from "@layout";
  import { changeCoverId } from "@stores/Modals";
  import { dbFilters, libraryGridType, loadedLibrary, romMetadata, roms } from "@stores/State";
  import { filterGrids, GRID_LAYOUTS } from "@utils";
  import Rom from "@views/library/Rom.svelte";
  import RomLoadingSkeleton from "@views/library/RomLoadingSkeleton.svelte";
  import { onDestroy, onMount } from "svelte";
  import type { Unsubscriber } from "svelte/store";

  let libraryLoadUnsub: Unsubscriber;

  let layout = $derived(GRID_LAYOUTS[$libraryGridType]);

  let romIdList = $derived(Object.keys($roms).sort((a: string, b: string) => {
    const metaA = $romMetadata[a];
    const metaB = $romMetadata[b];

    if (metaA.isFavorite && !metaB.isFavorite) return -1;
    if (metaB.isFavorite && !metaA.isFavorite) return 1;

    return a.localeCompare(b);
  }));

  let romsLoaded = $state(0);

  let loadedKeys = $state<Record<string, boolean>>({});

  onMount(() => {
    libraryLoadUnsub = loadedLibrary.subscribe(async (loaded) => {
      if (loaded) {
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
      }
    });
  });

  onDestroy(() => {
    if (libraryLoadUnsub) libraryLoadUnsub();
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
    keyFunction={(entry) => `${entry.index}|${entry.data}|${entry.data === $changeCoverId}`}
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