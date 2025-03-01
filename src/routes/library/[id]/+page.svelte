<script lang="ts">
  import { Icon } from "@component-utils";
  import { Download, Edit, FavoriteOff, FavoriteOn } from "@icons";
  import Button from "@interactables/Button.svelte";
  import { downloadProgressRom, showDownloadProgressModal } from "@stores/Modals";
  import { isLandscape, romMetadata, roms } from "@stores/State";
  import { GRID_LAYOUTS } from "@utils";
  import Cover from "@views/Cover.svelte";
  import SystemTag from "@views/SystemTag.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  let id = $derived(data.id);

  let rom = $derived($roms[id]);
  let metadata = $derived($romMetadata[id]);

  let portrait = $derived(!$isLandscape);
  let isFavorite = $derived(metadata.isFavorite);
  
  function toggleFavorite() {
    $romMetadata[id].isFavorite = !isFavorite;
    $romMetadata = { ...$romMetadata };
  }

  function download() {
    $downloadProgressRom = rom;
    $showDownloadProgressModal = true;
  }
</script>

<svelte:head>
	<title>{rom.title}</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<div id="rom-entry">
  <div class="header" class:portrait>
    {#if portrait}
      <!-- TODO: display back button -->
    {/if}
    <div
      class="cover"
      style="height: {GRID_LAYOUTS.portrait.height * 1.2}px;"
    >
      <Cover romId={id} />
    </div>
    <div class="info" class:portrait>
      <SystemTag system={rom.system} />
      <div class="m3-font-headline-{portrait ? "small" : "medium"}">
        {metadata.title || rom.title}
      </div>
      <div class="metadata">
        <!-- TODO: release year -->
        <!-- TODO: metacritic rating -->
        <!-- TODO: size -->
      </div>
    </div>
    <div class="controls" class:portrait style:--m3-button-shape="var(--m3-util-rounding-small)">
      <!-- TODO: favorite -->
      <Button iconType="full" type="text" on:click={toggleFavorite}>
        <Icon icon={isFavorite ? FavoriteOn : FavoriteOff} />
      </Button>
      <!-- TODO: download -->
      <Button
        type="filled"
        iconType="left"
        on:click={download}
      >
        <Icon icon={Download} />
        Download
      </Button>
      <!-- TODO: edit -->
      <Button iconType="full" type="filled">
        <Icon icon={Edit} />
      </Button>
    </div>
  </div>
  <div class="body">

  </div>
</div>

<style>
  #rom-entry {
    width: 100%;
    height: 100%;
  }

  .header {
    width: 100%;

    display: flex;
    align-items: flex-end;

    gap: 1rem;
  }
  .header.portrait {
    flex-direction: column;
    align-items: center;
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
</style>