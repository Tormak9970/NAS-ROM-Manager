<script lang="ts">
  import { romMetadata, roms } from "@stores/State";
  import type { ROM } from "@types";
  import RomsGrid from "@views/RomsGrid.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();


  let isLoading = $state(true);

  function filterRom(id:string, rom: ROM): boolean {
    const {
      textQuery,
      system,
      format,
      genre,
      publisher,
      developer,
      startReleaseDate,
      endReleaseDate
    } = data;

    const metadata = $romMetadata[id]?.metadata?.metadata;
    const releaseDate = metadata?.firstReleaseDate ? new Date(metadata.firstReleaseDate * 1000) : undefined;

    const startDateRange = startReleaseDate ? new Date(startReleaseDate) : undefined;
    const endDateRange = endReleaseDate ? new Date(endReleaseDate) : undefined;

    return (!textQuery || rom.title.includes(textQuery) || rom.systemFullName.includes(textQuery)) &&
      (!system || rom.system === system) &&
      (!format || rom.format === format) &&
      (!genre || (!!metadata?.genres && metadata.genres.includes(genre))) &&
      (!publisher || (!!metadata?.publishers && metadata.publishers.includes(publisher))) &&
      (!developer || (!!metadata?.developers && metadata.developers.includes(developer))) &&
      (!startDateRange || (!!releaseDate && releaseDate >= startDateRange)) &&
      (!endDateRange || (!!releaseDate && releaseDate <= endDateRange));
  }

  const romIdList = $derived.by(() => {
    const filteredKeys = Object.entries($roms).reduce((keys: string[], [key, value]) => {
      if (filterRom(key, value)) keys.push(key);

      return keys;
    }, []);

    return filteredKeys.sort((a: string, b: string) => {
      const metaA = $romMetadata[a];
      const metaB = $romMetadata[b];

      if (metaA.isFavorite && !metaB.isFavorite) return -1;
      if (metaB.isFavorite && !metaA.isFavorite) return 1;

      return a.localeCompare(b);
    });
  });
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