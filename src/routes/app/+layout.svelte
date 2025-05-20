<script>
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { Icon, MediaQuery } from "@component-utils";
  import { Celebration } from "@icons";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { LandscapeNav, PortraitNav } from "@navigation";
  import { isSignedIn } from "@stores/Auth";
  import { showUpdateModal } from "@stores/Modals";
  import { isLandscape, loadedApp } from "@stores/State";
  import { updateManifest } from "@stores/Update";
  import Header from "@views/Header.svelte";
  import Modals from "../../components/modals/Modals.svelte";
  import Sheets from "../../components/sheets/Sheets.svelte";

	let { children } = $props();

  $effect(() => {
    if ($loadedApp && !$isSignedIn && page.url.pathname !== '/' && page.url.pathname !== '/error') {
      goto('/');
    }
  });

  let condenseDesktopNav = $state(false);
</script>

<Modals />
<Sheets />
<MediaQuery query="(max-width: 1200px)" bind:matches={condenseDesktopNav} />
{#if !$loadedApp}
  <div class="loading-container">
    <LoadingSpinner /> <div class="font-headline-small">Loading NRM...</div>
  </div>
{:else}
  <div id="app-layout">
    <Header />
    <div class="page-body" class:mobile={!$isLandscape} class:landscape={$isLandscape}>
      <div class="nav" style:width={$isLandscape ? (condenseDesktopNav ? "4rem" : "15rem") : "100%"}>
        {#if $isLandscape}
          <LandscapeNav condenseNav={condenseDesktopNav} />
        {:else}
          <PortraitNav />
        {/if}
        {#if $updateManifest?.available && !condenseDesktopNav}
          <div class="update-container" style:--m3-button-shape="var(--m3-util-rounding-small)">
            <Button
              iconType="left"
              type="outlined"
              extraOptions={{ style: "width: 100%" }}
              onclick={() => $showUpdateModal = true}
            >
              <Icon icon={Celebration} />
              Update Available
            </Button>
          </div>
        {/if}
      </div>
      <div
        style:width={$isLandscape ? (condenseDesktopNav ? "calc(100% - 5rem)" : "calc(100% - 16rem)") : "100%"}
        style:height={$isLandscape ? "100%" : "calc(100% - 56px)"}
        style:margin-left={$isLandscape ? "1rem" : "0"}
      >
        {@render children()}
      </div>
    </div>
  </div>
{/if}


<style>
  #app-layout {
    width: 100%;
    height: 100%;
  }

  .page-body {
    width: calc(100% - 1rem);
    height: calc(100% - 1rem - 60px);

    padding: 0.5rem;

    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: flex-start;
  }

  .mobile {
    flex-direction: column-reverse;
    align-items: center;

    padding: 0;
    width: 100%;
    height: calc(100% - 60px);
  }

  .mobile .nav {
    width: 100%;
  }

  .landscape .nav {
    display: flex;
    flex-direction: column;

    justify-content: space-between;

    height: 100%;
  }

  .update-container {
    margin-bottom: 4rem;
    width: 100%;
  }

  .update-container :global(.m3-container),
  .update-container :global(svg) {
    color: rgb(var(--m3-scheme-primary)) !important;
  }
</style>