<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { DatabaseSearch } from "@icons";
  import { Button, TextField } from "@interactables";
  import { igdbSearchPlatformOnSelect, igdbSearchPlatformTitle, showAddSystemModal, showSearchIGDBPlatformModal } from "@stores/Modals";
  import type { DownloadStrategy, IGDBMetadataPlatform, SystemTagConfig } from "@types";

  type ParserPattern = {
    glob: string;
    regex: string;
    downloadStrategy: DownloadStrategy;
  }

  let open = $state(true);

  let title = $state("");
  let igdbId = $state("");
  let abbreviation = $state("");
  let folder = $state("");
  let tagConfig = $state<SystemTagConfig | null>(null);
  let patterns = $state<ParserPattern[]>([]);

  const canSave = $derived(!!title && !!igdbId && !!abbreviation && !!folder && !!tagConfig && patterns.length > 0);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    open = false;

    const newParser = {
      name: title,
      abbreviation: abbreviation,
      igdbPlatformId: igdbId,
      folder: folder,
      tagConfig: tagConfig,
      patterns: patterns,
    }

    // TODO: save to backend
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
  
  function openIGDBSearch() {
    $igdbSearchPlatformTitle = title;
    $igdbSearchPlatformOnSelect = (platform: IGDBMetadataPlatform | null) => {
      if (platform) {
        title = platform.name;
        igdbId = platform.igdbId;
        abbreviation = platform.abbreviation;
      }
    }
    $showSearchIGDBPlatformModal = true;
  }
</script>

<ModalBody
  headline="Add System"
  open={open}
  canClose={false} 
  oncloseend={() => { $showAddSystemModal = false }}
>
  <div class="content">
    <TextField
      name="Name"
      bind:value={title}
      trailingIcon={DatabaseSearch}
      ontrailingClick={openIGDBSearch}
    />
    <!-- TODO: error if there is a platform with this name already -->
    <TextField
      name="IGDB Id"
      bind:value={igdbId}
    />
    <TextField
      name="Abbreviation"
      bind:value={abbreviation}
    />
    <TextField
      name="Folder"
      bind:value={folder}
    />
    <!-- TODO: message with link to emudeck wiki -->
    <!-- TODO: tag config -->
    <!-- TODO: parser patterns -->
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onSave} disabled={!canSave}>Create</Button>
    </div>
  {/snippet}
</ModalBody>

<style>
  .content {
    width: 100%;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
</style>
