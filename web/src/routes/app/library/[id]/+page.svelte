<script lang="ts">
  import { goto } from "$app/navigation";
  import { routes } from "$lib/routes";
  import { Icon } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { Download, Edit, FavoriteOff, FavoriteOn, Upload } from "@icons";
  import Button from "@interactables/Button.svelte";
  import { LoadingSpinner } from "@layout";
  import LibraryLoadGuard from "@layout/load-guards/LibraryLoadGuard.svelte";
  import { IGDBService, RomService } from "@services";
  import { isLandscape, romMetadata, roms, showWarningSnackbar, systems } from "@stores/State";
  import { NO_IGDB_RESULTS } from "@types";
  import { formatFileSize } from "@utils";
  import DetailsHeader from "@views/DetailsHeader.svelte";
  import Hero from "@views/Hero.svelte";
  import RomMetadata from "@views/library/details/RomMetadata.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const id = $derived(data.id);

  const rom = $derived($roms?.[id]);
  const metadata = $derived($romMetadata?.[id]);
  const heroPath = $derived($romMetadata?.[id]?.heroPath);
  const system = $derived($systems?.[rom?.system]);

  const genres = $derived(metadata?.metadata?.metadata?.genres);

  let portrait = $state(false);
  const isFavorite = $derived(metadata?.isFavorite);

  let isLoading = $state(true);

  async function loadMetadata() {
    const ids = await IGDBService.searchForGame(metadata.title || rom.title, system.igdbPlatformId);
    
    if (ids.length > 0) {
      const igdbId = ids[0].igdbId.toString();
      const igdbMetadata = await IGDBService.getMetadata(igdbId);
      
      const metadata = $romMetadata[id];
      metadata.igdbId = igdbId;
      metadata.metadata = igdbMetadata;
      
      $romMetadata = { ...$romMetadata };
    } else {
      metadata.igdbId = NO_IGDB_RESULTS;
      $romMetadata = { ...$romMetadata };
    }

    isLoading = false;
  }

  async function onLoad() {
    if (!$roms[id]) {
      $showWarningSnackbar({ message: `Couldn't find ${id} in library!` });
      goto(routes["Library"].path);
      return;
    }

    if (metadata?.igdbId === "") {
      loadMetadata();
    } else {
      isLoading = false;
    }
  }
</script>

<svelte:head>
	<title>{isLoading || !rom ? "Loading..." : metadata?.title || rom?.title}</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<MediaQuery query="(max-width: 900px)" bind:matches={portrait} />

<LibraryLoadGuard onLoad={onLoad}>
  <div id="rom-entry" class="styled-scrollbar" class:landscape={!portrait}>
    <Hero
      src={heroPath}
      portrait={portrait}
      onEdit={() => RomService.changeHero(id)}
    />
    <div class="content" class:portrait>
      <DetailsHeader
        title={metadata?.title || rom?.title || "Loading..."}
        capsuleSrc={metadata?.thumbCapsulePath}
        system={rom?.system}
        portrait={portrait}
      >
        {#snippet headerMetadata()}
          {#if portrait}
            <div class="first-row">
              <div>Added on {rom?.addDate}</div>•<div>{formatFileSize(rom.size)}</div>
            </div>
            <div>{genres?.join(", ") ?? "Unkown"}</div>
          {:else}
            <div>Added on {rom?.addDate}</div>•<div>{formatFileSize(rom.size)}</div>•<div>{genres?.join(", ") ?? "Unkown"}</div>
          {/if}
        {/snippet}
        {#snippet controls()}
          <Button iconType="full" type="text" onclick={() => RomService.toggleFavorite(id)}>
            <Icon icon={isFavorite ? FavoriteOn : FavoriteOff} />
          </Button>
          {#if $isLandscape}
            <Button
              type="filled"
              iconType="left"
              onclick={() => RomService.download(id)}
            >
              <Icon icon={Download} />
              Download
            </Button>
          {:else}
            <Button
              type="filled"
              iconType="full"
              onclick={() => RomService.download(id)}
            >
              <Icon icon={Download} />
            </Button>
          {/if}
          <Button
            type="filled"
            iconType="full"
            onclick={() => RomService.replaceFile(id)}
          >
            <Icon icon={Upload} />
          </Button>
          <Button iconType="full" type="filled" onclick={() => RomService.edit(id)}>
            <Icon icon={Edit} />
          </Button>
        {/snippet}
      </DetailsHeader>
      <div class="body" class:portrait>
        {#if isLoading}
          <div class="loading-container">
            <LoadingSpinner /> <div class="font-headline-small">Loading Metadata...</div>
          </div>
        {:else if metadata.igdbId === NO_IGDB_RESULTS}
          <div class="missing-message font-headline-small">
            No Metadata for <b>{metadata.title ?? rom.title}</b>
          </div>
        {:else}
          <RomMetadata metadata={metadata} portrait={portrait} />
        {/if}
      </div>
    </div>
  </div>
</LibraryLoadGuard>

<style>
  #rom-entry {
    width: 100%;
    height: 100%;

    overflow-y: auto;
    
    position: relative;
  }

  #rom-entry.landscape {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .content {
    width: calc(100% - 1rem);
    height: 100%;
  }
  .content.portrait {
    width: 100%;
  }

  .body {
    width: 100%;
    margin-top: 2rem;

    position: relative;
    z-index: 2;
  }

  .body.portrait {
    padding: 0rem 1rem;
    width: calc(100% - 2rem);
  }
  
  .loading-container {
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;

    margin-top: 2rem;
  }

  .missing-message {
    font-style: italic;
  }
</style>