<script lang="ts">
  import { RestController, SGDBController } from "@controllers";
  import { ProgressIndicator } from "@interactables";
  import { VirtualGrid } from "@layout";
  import { checkRomArtwork, libraryGridType, romCustomizations, roms } from "@stores/State";
  import { GRID_LAYOUTS } from "@utils";
  import Rom from "@views/library/Rom.svelte";
  import { onMount } from "svelte";

  let layout = $derived(GRID_LAYOUTS[$libraryGridType]);

  let romIdList = $derived(Object.keys($roms));

  let romsLoaded = $state(0);

  onMount(async () => {
    $checkRomArtwork = true;
    if ($checkRomArtwork) {
      for (const romId of romIdList) {
        const customization = $romCustomizations[romId];

        if (customization.gridPath === "") {
          const grids = await SGDBController.getCoversForGame(customization.sgdbId);
          
          if (grids.length) {
            const first = grids[0];
            customization.gridPath = await RestController.cacheCover(first.url.toString(), romId);
          } else {
            customization.gridPath = "No Grids";
          }

          $romCustomizations[romId] = customization;
        }

        romsLoaded++;
      }

      $romCustomizations = { ...$romCustomizations };
      $checkRomArtwork = false;
    }
  });
</script>

<svelte:head>
	<title>Library - NRM</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<div id="library">
  {#if $checkRomArtwork}
    <div class="progress-container">
      <div class="progress-bar-container">
        <ProgressIndicator percent={romsLoaded / romIdList.length * 100} extraOptions={{ style: "height: 1rem" }} />
        <div class="m3-font-title-large info">{romsLoaded} of {romIdList.length} roms loaded.</div>
      </div>
    </div>
  {:else}
    <VirtualGrid
      name="library"
      itemHeight={layout.height} 
      itemWidth={layout.width}
      items={romIdList}
      columnGap={layout.gap}
      rowGap={layout.gap}
      let:entry
    >
      <Rom romId={entry} />
    </VirtualGrid>
  {/if}
</div>

<style>
  #library {
    width: 100%;
    height: 100%;
  }

  .progress-container {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .progress-bar-container {
    width: calc(100% - 14rem);
    max-width: 40rem;

    padding: 0rem 7rem;

    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .info {
    font-size: 1.25rem;
  }
</style>