<script lang="ts">
  import { romMetadata, roms } from "@stores/State";
  import RomsGrid from "@views/RomsGrid.svelte";

  let isLoading = $state(true);

  const romIdList = $derived(Object.keys($roms).sort((a: string, b: string) => {
    const metaA = $romMetadata[a];
    const metaB = $romMetadata[b];

    if (metaA.isFavorite && !metaB.isFavorite) return -1;
    if (metaB.isFavorite && !metaA.isFavorite) return 1;

    return a.localeCompare(b);
  }));
</script>

<svelte:head>
	<title>{isLoading ? "Loading..." : "Library - NRM"}</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<div id="library">
  <RomsGrid gridName="library" romIds={romIdList} bind:isLoading />
</div>

<style>
  #library {
    width: 100%;
    height: 100%;
  }
</style>