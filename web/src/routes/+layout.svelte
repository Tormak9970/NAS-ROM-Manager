<script>
  import { ContextMenu } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { AuthService, WebsocketService } from "@services";
  import { rememberMe } from "@stores/Auth";
  import { isLandscape, loadedApp, showInfoSnackbar, showWarningSnackbar } from "@stores/State";
  import { onMount } from "svelte";
  import "../app.css";
  import InfoSnackbar from "../components/snackbars/InfoSnackbar.svelte";
  import WarningSnackbar from "../components/snackbars/WarningSnackbar.svelte";
  import Theme from "../components/theme/Theme.svelte";
  import "../lib/md-defs";

	let { children } = $props();

  onMount(() => {
    WebsocketService.init(
      async () => {
        const user = sessionStorage.getItem("user");
        const hash = sessionStorage.getItem("hash");

        if (user && hash && $rememberMe) {
          await AuthService.authenticate(user, hash);
        }

        $loadedApp = true;
      },
      AuthService.logout
    );
  });
</script>

<MediaQuery query="(orientation:landscape)" bind:matches={$isLandscape} />
<Theme>
  <ContextMenu />
  <div id="root-layout">
    {@render children()}
  </div>
  <InfoSnackbar bind:show={$showInfoSnackbar} />
  <WarningSnackbar bind:show={$showWarningSnackbar} />
</Theme>

<style>
  #root-layout {
    width: 100%;
    height: 100%;
  }
</style>