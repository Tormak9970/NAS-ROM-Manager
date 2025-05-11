<script lang="ts">
  import { Icon } from "@component-utils";
  import { CalendarToday, Category, GameAsset, Groups, Sell, SupervisorAccount } from "@icons";
  import Button from "@interactables/Button.svelte";
  import { showDevelopersFilterSheet, showFormatsFilterSheet, showGenresFilterSheet, showPublishersFilterSheet, showReleaseDateFilterSheet, showSystemsFilterSheet } from "@stores/Sheets";
  import { isLandscape, romMetadata, roms } from "@stores/State";
  import type { ROM, System } from "@types";
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

  function filterSystem(system: System) {
    const { textQuery } = data;

    return (!textQuery || system.abbreviation.includes(textQuery) || system.name.includes(textQuery));
  }

  function filterEmulator() {

  }

  function filterBiosFiles() {

  }

  const searchResults = $derived.by(() => {
    const filteredKeys = Object.entries($roms).reduce((keys: string[], [key, value]) => {
      if (filterRom(key, value)) keys.push(key);

      return keys;
    }, []);

    const entries = filteredKeys.sort((a: string, b: string) => {
      const metaA = $romMetadata[a];
      const metaB = $romMetadata[b];

      if (metaA.isFavorite && !metaB.isFavorite) return -1;
      if (metaB.isFavorite && !metaA.isFavorite) return 1;

      return a.localeCompare(b);
    });

    return entries;
  });
</script>

<svelte:head>
	<title>Search - NRM</title>
</svelte:head>

<div id="search">
  {#if !$isLandscape}
    <div class="header-container">
      <div class="header">
        <Button type="outlined" iconType="left" onclick={() => $showSystemsFilterSheet = true}>
          <Icon icon={GameAsset} />
          Systems
        </Button>
        <Button type="outlined" iconType="left" onclick={() => $showFormatsFilterSheet = true}>
          <Icon icon={Sell} />
          Formats
        </Button>
        <Button type="outlined" iconType="left" onclick={() => $showGenresFilterSheet = true}>
          <Icon icon={Category} />
          Genres
        </Button>
        <Button type="outlined" iconType="left" onclick={() => $showDevelopersFilterSheet = true}>
          <Icon icon={Groups} />
          Developers
        </Button>
        <Button type="outlined" iconType="left" onclick={() => $showPublishersFilterSheet = true}>
          <Icon icon={SupervisorAccount} />
          Publisher
        </Button>
        <Button type="outlined" iconType="left" extraOptions={{ style: "width: 11rem"}} onclick={() => $showReleaseDateFilterSheet = true}>
          <Icon icon={CalendarToday} />
          Release Date
        </Button>
      </div>
    </div>
  {/if}
  <div class="results" style:height={$isLandscape ? "100%" : "calc(100% - 4.5rem)"}>
    <RomsGrid gridName="search" romIds={searchResults} bind:isLoading />
  </div>
</div>

<style>
  #search {
    width: 100%;
    height: 100%;
  }

  .header-container {
    width: 100%;
    overflow-x: auto;
    padding: 1rem 0;
  }

  .header {
    width: fit-content;
    
    padding: 0 1rem;

    display: flex;
    gap: 0.5rem;
  }

  .results {
    width: 100%;
    height: 100%;
  }
</style>