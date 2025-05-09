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
	<title>Search - NRM</title>
</svelte:head>

<div id="search">
  <RomsGrid gridName="search" romIds={romIdList} bind:isLoading />
</div>

<style>
  #search {
    width: 100%;
    height: 100%;
  }
</style>