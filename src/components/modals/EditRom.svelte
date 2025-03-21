<script lang="ts">
  import { Icon, ModalBody } from "@component-utils";
  import { RomController } from "@controllers";
  import { Cached, DatabaseSearch } from "@icons";
  import { Button, Checkbox, TextField } from "@interactables";
  import { editIsPostUpload, romEditingId, showEditRomModal } from "@stores/Modals";
  import { romMetadata, roms } from "@stores/State";

  let open = $state(true);

  let rom = $roms[$romEditingId!];
  let metadata = $romMetadata[$romEditingId!];

  let title = $state(metadata.title);
  let sgdbId = $state(metadata.sgdbId);
  let igdbId = $state(metadata.igdbId);
  let coverPath = $state(metadata.coverPath);
  let thumbPath = $state(metadata.thumbPath);
  let isFavorite = $state(metadata.isFavorite);

  let canSave = $derived(!!title);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    if (title !== rom.title || isFavorite) {
      $romMetadata[$romEditingId!] = {
        title: title,
        coverPath: coverPath,
        thumbPath: thumbPath,
        sgdbId: sgdbId,
        igdbId: igdbId,
        metadata: metadata.metadata,
        isFavorite: isFavorite
      }

      $romMetadata = { ...$romMetadata };
    }
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
</script>

<ModalBody headline={$editIsPostUpload ? "Confirm Details" : "Edit ROM"} open={open} canClose={false} on:closeEnd={closeEnd}>
  <div class="content">
    <TextField name="Title" bind:value={title} />
    <TextField name="SGDB Id" bind:value={sgdbId} trailingIcon={DatabaseSearch} />
    <TextField name="IGDB Id" bind:value={igdbId} trailingIcon={DatabaseSearch} />
    <label>
      <div class="m3-font-title-medium">Favorite:</div>
      <Checkbox bind:checked={isFavorite} />
    </label>
    <div class="actions" style:--m3-button-shape="var(--m3-util-rounding-small)">
      <Button type="elevated" on:click={() => RomController.changeCover($romEditingId!)}>Change Cover</Button>
      <Button type="elevated" iconType="left" on:click={() => RomController.refreshMetadata($romEditingId!, sgdbId)}>
        <Icon icon={Cached} />
        Metadata
      </Button>
    </div>
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onSave} disabled={!canSave}>Save</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 300px;

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
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
