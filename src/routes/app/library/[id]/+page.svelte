<script lang="ts">
  import { goto } from "$app/navigation";
  import { routes } from "$lib/routes";
  import { Icon } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { IGDBController, RomController } from "@controllers";
  import { BackArrow, Download, Edit, FavoriteOff, FavoriteOn } from "@icons";
  import Button from "@interactables/Button.svelte";
  import { LoadingSpinner } from "@layout";
  import LibraryLoadGuard from "@layout/load-guards/LibraryLoadGuard.svelte";
  import { romMetadata, roms, showWarningSnackbar, systems } from "@stores/State";
  import { NO_IGDB_RESULTS } from "@types";
  import { formatFileSize, GRID_LAYOUTS } from "@utils";
  import Capsule from "@views/Capsule.svelte";
  import Hero from "@views/Hero.svelte";
  import RomMetadata from "@views/library/details/RomMetadata.svelte";
  import SystemTag from "@views/SystemTag.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const id = $derived(data.id);

  const rom = $derived($roms?.[id]);
  const metadata = $derived($romMetadata?.[id]);
  const system = $derived($systems?.[rom?.system]);

  const genres = $derived(metadata?.metadata?.metadata?.genres);

  let portrait = $state(false);
  const isFavorite = $derived(metadata?.isFavorite);

  let isLoading = $state(true);

  async function loadMetadata() {
    const ids = await IGDBController.searchForGame(metadata.title || rom.title, system.igdbPlatformId);
    
    if (ids.length > 0) {
      const igdbId = ids[0].igdbId.toString();
      const igdbMetadata = await IGDBController.getMetadata(igdbId);
      
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
  <div id="rom-entry" class:landscape={!portrait}>
    {#if !isLoading}
      <Hero src={metadata?.heroPath} portrait={portrait} onEdit={() => RomController.changeHero(id)} />
      <div class="content" class:portrait>
        <div class="header" class:portrait>
          {#if portrait}
            <div class="back-button">
              <Button iconType="full" type="text" size="2.75rem" iconSize="1.75rem" onclick={() => window.history.back()}>
                <Icon icon={BackArrow} />
              </Button>
            </div>
          {/if}
          <div class="capsule" style="height: {GRID_LAYOUTS.portrait.height * 1.2}px;">
            {#key metadata?.thumbCapsulePath}
              <Capsule src={metadata?.thumbCapsulePath} />
            {/key}
          </div>
          <div class="info" class:portrait>
            <SystemTag system={rom?.system} />
            <div class="title m3-font-headline-{portrait ? "small" : "medium"}">
              {metadata?.title || rom?.title || "Loading..."}
            </div>
            <div class="header-metadata" class:portrait>
              {#if portrait}
                <div class="first-row">
                  <div>Added on {rom?.addDate}</div>•<div>{formatFileSize(rom.size)}</div>
                </div>
                <div>{genres?.join(", ") ?? "Unkown"}</div>
              {:else}
                <div>Added on {rom?.addDate}</div>•<div>{formatFileSize(rom.size)}</div>•<div>{genres?.join(", ") ?? "Unkown"}</div>
              {/if}
            </div>
          </div>
          <div class="controls" class:portrait style:--m3-button-shape="var(--m3-util-rounding-small)">
            <Button iconType="full" type="text" onclick={() => RomController.toggleFavorite(id)}>
              <Icon icon={isFavorite ? FavoriteOn : FavoriteOff} />
            </Button>
            <Button
              type="filled"
              iconType="left"
              onclick={() => RomController.download(id)}
            >
              <Icon icon={Download} />
              Download
            </Button>
            <Button iconType="full" type="filled" onclick={() => RomController.edit(id)}>
              <Icon icon={Edit} />
            </Button>
          </div>
        </div>
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
    {/if}
  </div>
</LibraryLoadGuard>

<style>
  #rom-entry {
    width: 100%;
    height: 100%;

    overflow-y: scroll;
    
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

  .header {
    width: 100%;

    display: flex;
    align-items: flex-end;

    position: relative;
    z-index: 2;

    gap: 1rem;

    pointer-events: none;
  }

  .header > * {
    pointer-events: all;
  }

  .header.portrait {
    flex-direction: column;
    align-items: center;
  }

  .back-button {
    position: absolute;

    left: 1rem;
    top: 0.75rem;
  }

  .portrait .title {
    text-align: center;
  }

  .capsule {
    position: relative;

    aspect-ratio: 2 / 3;

    margin-top: 0.5rem;
  }

  .info {
    display: flex;
    flex-direction: column;

    align-items: flex-start;
  }

  .info.portrait {
    align-items: center;
  }

  .header-metadata {
    font-size: 0.9rem;

    color: rgb(var(--m3-scheme-on-surface-variant));

    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  .header-metadata.portrait {
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }
  .first-row {
    display: flex;
    gap: 0.5rem;
  }

  .controls {
    width: fit-content;

    display: flex;

    gap: 0.5rem;

    margin-left: auto;
  }
  .controls.portrait {
    margin: 0;
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