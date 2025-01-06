<script>
  import "../app.css";
  import { onDestroy, onMount } from "svelte";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import Theme from "../components/theme/Theme.svelte";
  import { isLandscape, showErrorSnackbar, showInfoSnackbar } from "@stores/State";
  import { DesktopNav } from "@navigation";
  import MobileNav from "@navigation/MobileNav.svelte";
  import { AppController, RustInterop } from "@controllers";
  import { isSignedIn } from "@stores/Auth";
  import { page } from '$app/state';
  import { goto } from "$app/navigation";
  import InfoSnackbar from "../components/snackbars/InfoSnackbar.svelte";
  import ErrorSnackbar from "../components/snackbars/ErrorSnackbar.svelte";

	let { children } = $props();

  let condenseDesktopNav = $state(false);

  let validatingCredentials = $state(true);

  $effect(() => {
    if (!$isSignedIn && page.url.pathname !== '/') {
      goto('/');
    } else if (!validatingCredentials && $isSignedIn && page.url.pathname === '/') {
      goto('/dashboard');
    }
  });

  onMount(() => {
    AppController.init();
    RustInterop.init(async () => {
      const user = sessionStorage.getItem("user");
      const hash = sessionStorage.getItem("hash");

      if (user && hash) {
        RustInterop.authenticate(user, hash).then(() => {
          validatingCredentials = false;
        });
      } else {
        validatingCredentials = false;
      }
    });
  });

  onDestroy(() => {
    AppController.destroy();
  });
</script>

<MediaQuery query="(orientation:landscape)" bind:matches={$isLandscape} />
<MediaQuery query="(max-width: 1200px)" bind:matches={condenseDesktopNav} />
<Theme>
  <div class="layout" class:mobile={!$isLandscape}>
    {#if $isSignedIn}
      <!-- {#if $isLandscape}
        
      {/if} -->
      <div class="nav" style:width={$isLandscape ? (condenseDesktopNav ? "3.5rem" : "10rem") : "100%"}>
        {#if $isLandscape}
          <DesktopNav condenseNav={condenseDesktopNav} />
        {:else}
          <MobileNav />
        {/if}
      </div>
    {/if}
    <div class="content" style:width={condenseDesktopNav ? "calc(100% - 3.5rem)" : "calc(100% - 10rem)"}>
      {@render children()}
    </div>
  </div>
</Theme>
<InfoSnackbar bind:show={$showInfoSnackbar} />
<ErrorSnackbar bind:show={$showErrorSnackbar} />

<style>
  .layout {
    width: calc(100% - 1rem);
    height: calc(100% - 1rem);

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
    height: 100%;
  }

  .mobile .nav {
    width: 100%;
  }

  .content {
    height: 100%;
  }
</style>