<script lang="ts">
  import { Icon, ModalBody } from "@component-utils";
  import { RomController } from "@controllers";
  import { Cached, DatabaseSearch } from "@icons";
  import { Button, Checkbox, TextField } from "@interactables";
  import { editIsPostUpload, igdbSearchOnSelect, igdbSearchPlatformId, igdbSearchTitle, romEditingId, sgdbSearchOnSelect, sgdbSearchTitle, showEditRomModal, showSearchIGDBModal, showSearchSGDBModal } from "@stores/Modals";
  import { romMetadata, roms, systems } from "@stores/State";

  let open = $state(true);

  let rom = $roms[$romEditingId!];
  let system = $systems[rom.system];
  let metadata = $romMetadata[$romEditingId!];

  let title = $state(metadata.title);
  let sgdbId = $state(metadata.sgdbId);
  let igdbId = $state(metadata.igdbId);
  let coverPath = $state(metadata.coverPath);
  let thumbPath = $state(metadata.thumbPath);
  let isFavorite = $state(metadata.isFavorite);
  let igdbMetadata = $state(metadata.metadata);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    $romMetadata[$romEditingId!] = {
      title: title,
      coverPath: coverPath,
      thumbPath: thumbPath,
      sgdbId: sgdbId,
      igdbId: igdbId,
      metadata: igdbMetadata,
      isFavorite: isFavorite
    }

    $romMetadata = { ...$romMetadata };

    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }

  function closeEnd() {
    $showEditRomModal = false;
    $editIsPostUpload = false;
    $romEditingId = null;
  }

  async function refreshMetadata() {
    igdbMetadata = await RomController.refreshMetadata(igdbId);
  }

  function openSGDBSearch() {
    $sgdbSearchTitle = title ?? rom.title;
    $sgdbSearchOnSelect = (id: string) => {
      sgdbId = id;
    }
    $showSearchSGDBModal = true;
  }

  function openIGDBSearch() {
    $igdbSearchTitle = title ?? rom.title;
    $igdbSearchOnSelect = (id: string) => {
      igdbId = id;
    }
    $igdbSearchPlatformId = system.igdbPlatformId;
    $showSearchIGDBModal = true;
  }
</script>

<ModalBody
  headline={$editIsPostUpload ? "Confirm Details" : "Edit ROM"}
  open={open}
  canClose={false}
  on:closeEnd={closeEnd}
>
  <div class="content">
    <TextField name="Title" bind:value={title} />
    <TextField
      name="SGDB Id"
      bind:value={sgdbId}
      trailingIcon={DatabaseSearch}
      on:trailingClick={openSGDBSearch}
    />
    <TextField
      name="IGDB Id"
      bind:value={igdbId}
      trailingIcon={DatabaseSearch}
      on:trailingClick={openIGDBSearch}
    />
    <label>
      <div class="m3-font-title-medium">Favorite:</div>
      <Checkbox bind:checked={isFavorite} />
    </label>
    <div class="actions" style:--m3-button-shape="var(--m3-util-rounding-small)">
      <Button type="elevated" on:click={() => RomController.changeCover($romEditingId!)}>
        Edit Cover
      </Button>
      <Button type="elevated" iconType="left" on:click={refreshMetadata}>
        <Icon icon={Cached} />
        Metadata
      </Button>
    </div>
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onSave}>Save</Button>
  </div>
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

  .actions {
    width: 100%;

    display: flex;
    justify-content: space-between;

    gap: 0.5rem;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
