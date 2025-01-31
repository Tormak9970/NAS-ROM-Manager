<script>
  import { showAddLibraryModal, showControlledModal, showDownloadProgressModal } from "@stores/Modals";
  import AddLibrary from "./AddLibrary.svelte";
  import Controlled from "./Controlled.svelte";
  import DownloadProgress from "./DownloadProgress.svelte";
</script>

{#if $showControlledModal}
  <Controlled />
{/if}

{#if $showAddLibraryModal}
  <AddLibrary />
{/if}

{#if $showDownloadProgressModal}
  <DownloadProgress />
{/if}