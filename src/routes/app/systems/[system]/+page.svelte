<script lang="ts">
  import { goto } from "$app/navigation";
  import { routes } from "$lib/routes";
  import { Icon } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { SystemController } from "@controllers";
  import { BackArrow, Edit } from "@icons";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import LibraryLoadGuard from "@layout/load-guards/LibraryLoadGuard.svelte";
  import { romsBySystem, showWarningSnackbar, systems } from "@stores/State";
  import { GRID_LAYOUTS, pluralize } from "@utils";
  import Cover from "@views/Cover.svelte";
  import SystemDetails from "@views/systems/SystemDetails.svelte";
  import SystemTag from "@views/SystemTag.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const abbreviation = $derived(data.system);

  const system = $derived($systems[abbreviation]);
  const romIds = $derived($romsBySystem[abbreviation])

  let portrait = $state(false);

  let isLoading = $state(true);

  async function onLoad() {
    if (!$systems[abbreviation]) {
      $showWarningSnackbar({ message: `Couldn't find ${abbreviation} in library!` });
      goto(routes["Systems"].path);
      return;
    }

    isLoading = false;
  }
</script>

<svelte:head>
	<title>{system ? system.name : "Loading..."}</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<MediaQuery query="(max-width: 900px)" bind:matches={portrait} />

<LibraryLoadGuard onLoad={onLoad}>
  <div id="system-entry">
    {#if  !isLoading}
      <div class="header" class:portrait>
        {#if portrait}
          <div class="back-button">
            <Button iconType="full" type="text" size="2.75rem" iconSize="1.75rem" onclick={() => window.history.back()}>
              <Icon icon={BackArrow} />
            </Button>
          </div>
        {/if}
        <div class="cover" style="height: {GRID_LAYOUTS.portrait.height * 1.2}px;">
          <Cover thumbPath={system?.thumbPath} />
        </div>
        <div class="info" class:portrait>
          <SystemTag system={abbreviation} />
          <div class="title m3-font-headline-{portrait ? "small" : "medium"}">
            {system?.name || "Loading..."}
          </div>
          <div class="header-metadata" class:portrait>
            {#if portrait}
              <div class="first-row">
                <div>{romIds?.length ?? 0} {pluralize("rom", "roms", romIds?.length)}</div>•<div>{system?.patterns?.length ?? 0} {pluralize("parser", "parsers", system?.patterns?.length)}</div>
              </div>
              <div>Abbreviation: {system?.abbreviation}</div>
            {:else}
              <div>Abbreviation: {system?.abbreviation}</div>•<div>{romIds?.length ?? 0} {pluralize("rom", "roms", romIds?.length)}</div>•<div>{system?.patterns?.length ?? 0} {pluralize("parser", "parsers", system?.patterns?.length)}</div>
            {/if}
          </div>
        </div>
        <div class="controls" class:portrait style:--m3-button-shape="var(--m3-util-rounding-small)">
          <Button iconType="left" type="filled" onclick={() => SystemController.edit(abbreviation)}>
            <Icon icon={Edit} />
            Edit
          </Button>
        </div>
      </div>
      <div class="body" class:portrait>
        {#if isLoading}
          <div class="loading-container">
            <LoadingSpinner /> <div class="font-headline-small">Loading Metadata...</div>
          </div>
        {:else}
          <SystemDetails system={system} romIds={romIds} portrait={portrait} />
        {/if}
      </div>
    {/if}
  </div>
</LibraryLoadGuard>

<style>
  #system-entry {
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
    /* width: 100%; */
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
</style>