<script>
  import "../app.css";
  import "../lib/md-defs";
  import { onMount } from "svelte";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import Theme from "../components/theme/Theme.svelte";
  import { isLandscape, showErrorSnackbar, showInfoSnackbar } from "@stores/State";
  import { DesktopNav } from "@navigation";
  import MobileNav from "@navigation/MobileNav.svelte";
  import { AuthController, RustInterop } from "@controllers";
  import { isSignedIn, rememberMe } from "@stores/Auth";
  import { page } from '$app/state';
  import { goto } from "$app/navigation";
  import InfoSnackbar from "../components/snackbars/InfoSnackbar.svelte";
  import ErrorSnackbar from "../components/snackbars/ErrorSnackbar.svelte";
  import Header from "@views/Header.svelte";
  import { ContextMenu } from "@component-utils";
  import Modals from "../components/modals/Modals.svelte";

	let { children } = $props();

  let condenseDesktopNav = $state(false);

  let validatingCredentials = $state(true);

  $effect(() => {
    if (!$isSignedIn && page.url.pathname !== '/') {
      goto('/');
    } else if (!validatingCredentials && $isSignedIn && page.url.pathname === '/') {
      goto('/loading');
    }
  });

  onMount(() => {
    RustInterop.init(
      async () => {
        const user = sessionStorage.getItem("user");
        const hash = sessionStorage.getItem("hash");

        if (user && hash && $rememberMe) {
          await AuthController.authenticate(user, hash).then(() => {
            validatingCredentials = false;
          });
        } else {
          validatingCredentials = false;
        }
      },
      AuthController.logout
    );
  });
</script>

<MediaQuery query="(orientation:landscape)" bind:matches={$isLandscape} />
<MediaQuery query="(max-width: 1200px)" bind:matches={condenseDesktopNav} />
<Theme>
  <ContextMenu />
  <Modals />
  <div class="layout">
    {#if page.url.pathname !== '/' && page.url.pathname !== '/loading'}
      <Header />
    {/if}
    <div class="page-body" class:mobile={!$isLandscape}>
      {#if page.url.pathname !== '/' && page.url.pathname !== '/loading'}
        <div class="nav" style:width={$isLandscape ? (condenseDesktopNav ? "4rem" : "15rem") : "100%"}>
          {#if $isLandscape}
            <DesktopNav condenseNav={condenseDesktopNav} />
          {:else}
            <MobileNav />
          {/if}
        </div>
      {/if}
      <div
        style:width={$isLandscape ? (condenseDesktopNav ? "calc(100% - 5rem)" : "calc(100% - 16rem)") : "100%"}
        style:height={$isLandscape ? "100%" : "calc(100% - 56px)"}
        style:margin-left={$isLandscape ? "1rem" : "0"}
      >
        {@render children()}
      </div>
    </div>
  </div>
</Theme>
<InfoSnackbar bind:show={$showInfoSnackbar} />
<ErrorSnackbar bind:show={$showErrorSnackbar} />

<style>
  .layout {
    width: 100%;
    height: 100%;
  }

  .page-body {
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
    height: calc(100% - 60px);
  }

  .mobile .nav {
    width: 100%;
  }
</style>