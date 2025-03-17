<script>
  import { ContextMenu } from "@component-utils";
  import MediaQuery from "@component-utils/MediaQuery.svelte";
  import { AuthController, WebsocketController } from "@controllers";
  import { rememberMe } from "@stores/Auth";
  import { isLandscape, loadedApp, showInfoSnackbar, showWarningSnackbar } from "@stores/State";
  import { onMount } from "svelte";
  import "../app.css";
  import Modals from "../components/modals/Modals.svelte";
  import Sheets from "../components/sheets/Sheets.svelte";
  import InfoSnackbar from "../components/snackbars/InfoSnackbar.svelte";
  import WarningSnackbar from "../components/snackbars/WarningSnackbar.svelte";
  import Theme from "../components/theme/Theme.svelte";
  import "../lib/md-defs";

	let { children } = $props();

  onMount(() => {
    WebsocketController.init(
      async () => {
        const user = sessionStorage.getItem("user");
        const hash = sessionStorage.getItem("hash");

        if (user && hash && $rememberMe) {
          await AuthController.authenticate(user, hash);
        }

        $loadedApp = true;
      },
      AuthController.logout
    );
  });
</script>

<MediaQuery query="(orientation:landscape)" bind:matches={$isLandscape} />
<Theme>
  <ContextMenu />
  <Modals />
  <Sheets />
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