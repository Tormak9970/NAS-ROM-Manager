<script lang="ts">
  import { Icon } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { IGDBController } from "@controllers";
  import { BackArrow, Download, Edit, FavoriteOff, FavoriteOn } from "@icons";
  import Button from "@interactables/Button.svelte";
  import { LoadingSpinner } from "@layout";
  import LibraryLoadGuard from "@layout/load-guards/LibraryLoadGuard.svelte";
  import { downloadProgressRom, romEditingId, showDownloadProgressModal, showEditRomModal } from "@stores/Modals";
  import { romMetadata, roms, showInfoSnackbar, systems } from "@stores/State";
  import { NO_IGDB_RESULTS } from "@types";
  import { formatFileSize, GRID_LAYOUTS } from "@utils";
  import Cover from "@views/Cover.svelte";
  import RomMetadata from "@views/library/details/RomMetadata.svelte";
  import SystemTag from "@views/SystemTag.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  let id = $derived(data.id);

  let rom = $derived($roms?.[id]);
  let metadata = $derived($romMetadata?.[id]);
  let system = $derived($systems?.[rom?.system]);

  let genres = $derived(metadata?.metadata?.metadata?.genres);

  let portrait = $state(false);
  let isFavorite = $derived(metadata?.isFavorite);

  let isLoading = $state(true);
  
  function toggleFavorite() {
    $romMetadata[id].isFavorite = !isFavorite;
    $romMetadata = { ...$romMetadata };
  }
  
  function editRom() {
    $romEditingId = id;
    $showEditRomModal = true;
  }

  function download() {
    $downloadProgressRom = rom;
    $showDownloadProgressModal = true;
  }

  async function loadMetadata() {
    const ids = await IGDBController.searchForGame(metadata.title || rom.title, system.igdbPlatformId);
    
    if (ids.length > 0) {
      metadata.igdbId = ids[0].igdbId.toString();

      const igdbMetadata = await IGDBController.getMetadata(metadata.igdbId);
      metadata.metadata = igdbMetadata;

      $romMetadata = { ...$romMetadata };
    } else {
      metadata.igdbId = NO_IGDB_RESULTS;
    }

    isLoading = false;
  }

  async function onLoad() {
    if (!$roms[id]) {
      $showInfoSnackbar({ message: "Couldn't find rom in library!" });
      // TODO: redirect to error page
    }

    if (metadata.igdbId === "") {
      loadMetadata();
    } else {
      isLoading = false;
    }
  }
</script>

<svelte:head>
	<title>{rom ? metadata?.title || rom?.title : "Loading..."}</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<MediaQuery query="(max-width: 900px)" bind:matches={portrait} />

<LibraryLoadGuard onLoad={onLoad}>
  <div id="rom-entry">
    <div class="header" class:portrait>
      {#if portrait}
        <div class="back-button">
          <Button iconType="full" type="text" size="2.75rem" iconSize="1.75rem" on:click={() => window.history.back()}>
            <Icon icon={BackArrow} />
          </Button>
        </div>
      {/if}
      <div class="cover" style="height: {GRID_LAYOUTS.portrait.height * 1.2}px;">
        <Cover romId={id} />
      </div>
      <div class="info" class:portrait>
        <SystemTag system={rom.system} />
        <div class="title m3-font-headline-{portrait ? "small" : "medium"}">
          {metadata.title || rom.title}
        </div>
        <div class="header-metadata">
          Added on {rom.addDate}  •  {formatFileSize(rom.size)}  •  {genres?.join(", ") ?? "Unkown"}
        </div>
      </div>
      <div class="controls" class:portrait style:--m3-button-shape="var(--m3-util-rounding-small)">
        <Button iconType="full" type="text" on:click={toggleFavorite}>
          <Icon icon={isFavorite ? FavoriteOn : FavoriteOff} />
        </Button>
        <Button
          type="filled"
          iconType="left"
          on:click={download}
        >
          <Icon icon={Download} />
          Download
        </Button>
        <Button iconType="full" type="filled" on:click={editRom}>
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
        <RomMetadata metadata={metadata} />
      {/if}
    </div>
  </div>
</LibraryLoadGuard>

<style>
  #rom-entry {
    width: 100%;
    height: 100%;

    overflow-y: scroll;
  }

  .header {
    width: 100%;

    display: flex;
    align-items: flex-end;

    position: relative;

    gap: 1rem;
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

  .cover {
    position: relative;

    aspect-ratio: 2 / 3;

    margin-top: 1rem;
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
    white-space: pre;
    font-size: 0.9rem;

    color: rgb(var(--m3-scheme-on-surface-variant));
  }

  .controls {
    width: fit-content;

    display: flex;

    gap: 0.5rem;

    margin-left: auto;
    margin-right: 1rem;
  }
  .controls.portrait {
    margin: 0;
  }

  .body {
    width: 100%;
    margin-top: 2rem;
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