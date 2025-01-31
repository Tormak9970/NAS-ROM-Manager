<script>
  import { goto } from "$app/navigation";
  import { page } from '$app/state';
  import { ContextMenu } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { AuthController, RustInterop } from "@controllers";
  import { ApiController } from "@controllers/ApiController";
  import { DesktopNav } from "@navigation";
  import MobileNav from "@navigation/MobileNav.svelte";
  import { isSignedIn, rememberMe } from "@stores/Auth";
  import { isLandscape, showErrorSnackbar, showInfoSnackbar } from "@stores/State";
  import Header from "@views/Header.svelte";
  import { onMount } from "svelte";
  import "../app.css";
  import Modals from "../components/modals/Modals.svelte";
  import ErrorSnackbar from "../components/snackbars/ErrorSnackbar.svelte";
  import InfoSnackbar from "../components/snackbars/InfoSnackbar.svelte";
  import Theme from "../components/theme/Theme.svelte";
  import "../lib/md-defs";

	let { children } = $props();

  let condenseDesktopNav = $state(false);

  let validatingCredentials = $state(true);

  let showDecorations = $state(false);

  $effect(() => {
    if (!$isSignedIn && page.url.pathname !== '/') {
      goto('/');
    } else if (!validatingCredentials && $isSignedIn && page.url.pathname === '/') {
      goto('/loading');
    }

    showDecorations = page.url.pathname !== '/' && page.url.pathname !== '/loading' && page.url.pathname !== '/error';
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

        setTimeout(() => {
          console.log("testing cover caching");
          ApiController.cacheCover("https://cdn2.steamgriddb.com/thumb/3c64afe806cd466dd1ffecbe3e2e8cce.jpg", "test");
        }, 5000);
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
    {#if showDecorations}
      <Header />
    {/if}
    <div class="page-body" class:mobile={!$isLandscape}>
      {#if showDecorations}
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
</style>