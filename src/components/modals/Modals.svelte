<script>
  import { showAddLibraryModal, showAddRomModal, showControlledModal, showDownloadProgressModal, showEditRomModal, showFilePickerModal, showUploadProgressModal } from "@stores/Modals";
  import AddLibrary from "./AddLibrary.svelte";
  import AddRom from "./AddRom.svelte";
  import Controlled from "./Controlled.svelte";
  import DownloadProgress from "./DownloadProgress.svelte";
  import EditRom from "./EditRom.svelte";
  import FilePicker from "./file-picker/FilePicker.svelte";
  import UploadProgress from "./UploadProgress.svelte";
</script>

{#if $showControlledModal}
  <Controlled />
{/if}

{#if $showFilePickerModal}
  <FilePicker />
{/if}

{#if $showAddLibraryModal}
  <AddLibrary />
{/if}

{#if $showDownloadProgressModal}
  <DownloadProgress />
{/if}

{#if $showAddRomModal}
  <AddRom />
{/if}

{#if $showUploadProgressModal}
  <UploadProgress />
{/if}

{#if $showEditRomModal}
  <EditRom />
{/if}