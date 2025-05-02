<script lang="ts">
  import { goto } from "$app/navigation";
  import { routes } from "$lib/routes";
  import { Icon } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { SystemController } from "@controllers";
  import { Edit } from "@icons";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import LibraryLoadGuard from "@layout/load-guards/LibraryLoadGuard.svelte";
  import { romsBySystem, showWarningSnackbar, systems } from "@stores/State";
  import { pluralize } from "@utils";
  import DetailsHeader from "@views/DetailsHeader.svelte";
  import Hero from "@views/Hero.svelte";
  import SystemDetails from "@views/systems/SystemDetails.svelte";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const abbreviation = $derived(data.system);

  const system = $derived($systems[abbreviation]);
  const heroPath = $derived($systems[abbreviation]?.heroPath);
  const romIds = $derived($romsBySystem[abbreviation]);

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
	<title>{isLoading || !system ? "Loading..." : system.name}</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<MediaQuery query="(max-width: 900px)" bind:matches={portrait} />

<LibraryLoadGuard onLoad={onLoad}>
  <div id="system-entry" class="styled-scrollbar" class:landscape={!portrait}>
    {#if !isLoading}
      <Hero
        src={heroPath}
        portrait={portrait}
        onEdit={() => SystemController.changeHero(abbreviation)}
      />
      <div class="content" class:portrait>
        <DetailsHeader
          title={system?.name || "Loading..."}
          capsuleSrc={system?.thumbCapsulePath}
          system={abbreviation}
          portrait={portrait}
        >
          {#snippet headerMetadata()}
            {#if portrait}
              <div class="first-row">
                <div>{romIds?.length ?? 0} {pluralize("rom", "roms", romIds?.length)}</div>•<div>{system?.patterns?.length ?? 0} {pluralize("parser", "parsers", system?.patterns?.length)}</div>
              </div>
              <div>Abbreviation: {system?.abbreviation}</div>
            {:else}
              <div>Abbreviation: {system?.abbreviation}</div>•<div>{romIds?.length ?? 0} {pluralize("rom", "roms", romIds?.length)}</div>•<div>{system?.patterns?.length ?? 0} {pluralize("parser", "parsers", system?.patterns?.length)}</div>
            {/if}
          {/snippet}
          {#snippet controls()}
            <Button iconType="left" type="filled" onclick={() => SystemController.edit(abbreviation)}>
              <Icon icon={Edit} />
              Edit
            </Button>
          {/snippet}
        </DetailsHeader>
        <div class="body" class:portrait>
          {#if isLoading}
            <div class="loading-container">
              <LoadingSpinner /> <div class="font-headline-small">Loading Metadata...</div>
            </div>
          {:else}
            <SystemDetails system={system} romIds={romIds} portrait={portrait} />
          {/if}
        </div>
      </div>
    {/if}
  </div>
</LibraryLoadGuard>

<style>
  #system-entry {
    width: 100%;
    height: 100%;

    overflow-y: auto;
    
    position: relative;
  }

  #system-entry.landscape {
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
</style>