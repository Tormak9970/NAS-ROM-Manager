<script>
  import "../app.css";
  import { onDestroy, onMount } from "svelte";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import Theme from "../components/theme/Theme.svelte";
  import { isLandscape } from "@stores/State";
  import { DesktopNav } from "@navigation";
  import MobileNav from "@navigation/MobileNav.svelte";
  import { AppController, RustInterop } from "@controllers";

  import jsSHA from "jssha";

  export const ssr = false;

	let { children } = $props();

  let condenseDesktopNav = $state(false);

  onMount(() => {
    AppController.init();
    RustInterop.init();

    setTimeout(async () => {

      const shaObj = new jsSHA("SHA-256", "TEXT", { encoding: "UTF8" });
      shaObj.update("test123");

      const hash = shaObj.getHash("HEX");

      const result = await RustInterop.authenticate("Admin", hash);

      console.log("Authenticated:", result);
    }, 1000);
  });

  onDestroy(() => {
    AppController.destroy();
  });
</script>

<MediaQuery query="(orientation:landscape)" bind:matches={$isLandscape} />
<MediaQuery query="(max-width: 1200px)" bind:matches={condenseDesktopNav} />
<Theme>
  <div class="layout" class:mobile={!$isLandscape}>
    {#if $isLandscape}
      
    {/if}
    <div class="nav" style:width={$isLandscape ? (condenseDesktopNav ? "3.5rem" : "10rem") : "100%"}>
      {#if $isLandscape}
        <DesktopNav condenseNav={condenseDesktopNav} />
      {:else}
        <MobileNav />
      {/if}
    </div>
    <div class="content" style:width={condenseDesktopNav ? "calc(100% - 3.5rem)" : "calc(100% - 10rem)"}>
      {@render children()}
    </div>
  </div>
</Theme>

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
  }

  .mobile .nav {
    width: 100%;
  }

  .content {
    height: 100%;
  }
</style>