<script>
  import { showAddRomModal, showChangeCoverModal, showControlledModal, showDownloadProgressModal, showEditLibraryModal, showEditRomModal, showEditRouteOrderModal, showFilePickerModal, showLoadingModal, showUploadProgressModal } from "@stores/Modals";
  import AddRom from "./AddRom.svelte";
  import ChangeCover from "./change-cover/ChangeCover.svelte";
  import Controlled from "./Controlled.svelte";
  import DownloadProgress from "./DownloadProgress.svelte";
  import EditLibrary from "./EditLibrary.svelte";
  import EditRom from "./EditRom.svelte";
  import EditRouteOrder from "./EditRouteOrder.svelte";
  import FilePicker from "./file-picker/FilePicker.svelte";
  import Loading from "./Loading.svelte";
  import UploadProgress from "./UploadProgress.svelte";
</script>

{#if $showControlledModal}
  <Controlled />
{/if}

{#if $showEditRouteOrderModal}
  <EditRouteOrder />
{/if}

{#if $showFilePickerModal}
  <FilePicker />
{/if}

{#if $showEditLibraryModal}
  <EditLibrary />
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

{#if $showLoadingModal}
  <Loading />
{/if}

{#if $showChangeCoverModal}
  <ChangeCover />
{/if}