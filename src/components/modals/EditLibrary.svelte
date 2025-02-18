<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { AppController, SettingsController } from "@controllers";
  import { Button, PathField, TextField } from "@interactables";
  import { isFirstSetup, showEditLibraryModal } from "@stores/Modals";
  import { library } from "@stores/State";
  import { FileSelectionType, type Library } from "@types";

  let open = $state(true);

  let libraryPath = $state($library.libraryPath || "/");
  let romDir = $state($library.romDir || "roms");
  let emulatorDir = $state($library.emulatorDir || "emulators");
  let biosDir = $state($library.biosDir || "bios");

  let canSave = $derived(!!libraryPath && !!romDir && !!emulatorDir && !!biosDir);

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    open = false;

    const newLibrary: Library = {
      libraryPath: libraryPath,
      romDir: romDir,
      emulatorDir: emulatorDir,
      biosDir: biosDir
    }

    AppController.updateLibrary(newLibrary);
    $library = newLibrary;
    SettingsController.set("library", newLibrary);

    $isFirstSetup = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline={$isFirstSetup ? "Setup Library" : "Edit Library"} open={open} canClose={false} on:closeEnd={() => { $showEditLibraryModal = false }}>
  <div class="content">
    <PathField name="Library Path" type={FileSelectionType.FOLDER} bind:value={libraryPath} />
    <TextField name="Roms Directory" bind:value={romDir} />
    <TextField name="Emulators Directory" bind:value={emulatorDir} />
    <TextField name="Bios Files Directory" bind:value={biosDir} />
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel} disabled={$isFirstSetup}>Cancel</Button>
    <Button type="text" on:click={onConfirm} disabled={!canSave}>Save</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 300px;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
