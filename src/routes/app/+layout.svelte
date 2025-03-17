<script>
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { MediaQuery } from "@component-utils";
  import { LoadingSpinner } from "@layout";
  import { LandscapeNav, PortraitNav } from "@navigation";
  import { isSignedIn } from "@stores/Auth";
  import { isLandscape, loadedApp } from "@stores/State";
  import Header from "@views/Header.svelte";

	let { children } = $props();

  $effect(() => {
    if ($loadedApp && !$isSignedIn && page.url.pathname !== '/' && page.url.pathname !== '/error') {
      goto('/');
    }
  });

  let condenseDesktopNav = $state(false);
</script>

<MediaQuery query="(max-width: 1200px)" bind:matches={condenseDesktopNav} />
{#if !$loadedApp}
  <div class="loading-container">
    <LoadingSpinner /> <div class="font-headline-small">Loading NRM...</div>
  </div>
{:else}
  <div id="app-layout">
    <Header />
    <div class="page-body" class:mobile={!$isLandscape}>
      <div class="nav" style:width={$isLandscape ? (condenseDesktopNav ? "4rem" : "15rem") : "100%"}>
        {#if $isLandscape}
          <LandscapeNav condenseNav={condenseDesktopNav} />
        {:else}
          <PortraitNav />
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

  .loading-container {
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;
  }
</style>