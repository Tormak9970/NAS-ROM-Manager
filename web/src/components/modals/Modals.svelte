<script>
  import { showAddBiosFileModal, showAddExtraFileModal, showAddRomModal, showAddSystemModal, showChangeGridsModal, showChangelogModal, showControlledModal, showDownloadProgressModal, showEditLibraryModal, showEditRomModal, showEditRouteOrderModal, showEditSystemModal, showFilePickerModal, showLoadingModal, showReplaceBiosFileModal, showReplaceExtraFileModal, showReplaceRomModal, showSearchFiltersModal, showSearchIGDBPlatformModal, showSearchIGDBRomModal, showSearchSGDBModal, showUpdateModal, showUploadProgressModal } from "@stores/Modals";
  import AddBiosFile from "./AddBiosFile.svelte";
  import AddExtraFile from "./AddExtraFile.svelte";
  import AddRom from "./AddRom.svelte";
  import ChangeGrids from "./change-grids/ChangeGrids.svelte";
  import Changelog from "./Changelog.svelte";
  import Controlled from "./Controlled.svelte";
  import DownloadProgress from "./DownloadProgress.svelte";
  import EditLibrary from "./EditLibrary.svelte";
  import EditRom from "./EditRom.svelte";
  import EditRouteOrder from "./EditRouteOrder.svelte";
  import FilePicker from "./file-picker/FilePicker.svelte";
  import IgdbSearchPlatform from "./IGDBSearchPlatform.svelte";
  import IGDBSearchRom from "./IGDBSearchRom.svelte";
  import Loading from "./Loading.svelte";
  import ReplaceBiosFile from "./ReplaceBiosFile.svelte";
  import ReplaceExtraFile from "./ReplaceExtraFile.svelte";
  import ReplaceRom from "./ReplaceRom.svelte";
  import SearchFilters from "./SearchFilters.svelte";
  import SGDBSearch from "./SGDBSearch.svelte";
  import AddSystem from "./systems/AddSystem.svelte";
  import EditSystem from "./systems/EditSystem.svelte";
  import UpdateModal from "./UpdateModal.svelte";
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

{#if $showAddSystemModal}
  <AddSystem />
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

{#if $showChangeGridsModal}
  <ChangeGrids />
{/if}

{#if $showSearchSGDBModal}
  <SGDBSearch />
{/if}

{#if $showSearchIGDBRomModal}
  <IGDBSearchRom />
{/if}

{#if $showSearchIGDBPlatformModal}
  <IgdbSearchPlatform />
{/if}

{#if $showEditSystemModal}
  <EditSystem />
{/if}

{#if $showSearchFiltersModal}
  <SearchFilters />
{/if}

{#if $showUpdateModal}
  <UpdateModal />
{/if}

{#if $showChangelogModal}
  <Changelog />
{/if}

{#if $showAddBiosFileModal}
  <AddBiosFile />
{/if}

{#if $showReplaceRomModal}
  <ReplaceRom />
{/if}

{#if $showReplaceBiosFileModal}
  <ReplaceBiosFile />
{/if}

{#if $showAddExtraFileModal}
  <AddExtraFile />
{/if}

{#if $showReplaceExtraFileModal}
  <ReplaceExtraFile />
{/if}