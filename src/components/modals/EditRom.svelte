<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, Checkbox, TextField } from "@interactables";
  import { editIsPostUpload, romEditingId, showEditRomModal } from "@stores/Modals";
  import { romCustomizations, roms } from "@stores/State";

  let open = $state(true);

  let rom = $roms[$romEditingId!];
  let romCustomization = $romCustomizations[$romEditingId!];

  let title = $state(romCustomization?.title ?? rom.title);
  let isFavorite = $state(romCustomization?.isFavorite ?? false);

  let canSave = $derived(!!title);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    if (title !== rom.title || isFavorite) {
      $romCustomizations[$romEditingId!] = {
        path: rom.path,
        title: title !== rom.title ? title : "",
        gridPath: romCustomization.gridPath ?? "",
        isFavorite: isFavorite
      }

      $romCustomizations = { ...$romCustomizations };
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
    <label>
      <div class="m3-font-title-medium">Favorite:</div>
      <Checkbox bind:checked={isFavorite} />
    </label>
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

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
