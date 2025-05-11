<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { AppController, SettingsController } from "@controllers";
  import { Button, PathField, TextField } from "@interactables";
  import { isFirstSetup, showEditLibraryModal } from "@stores/Modals";
  import { library, loadedLibrary } from "@stores/State";
  import { FileSelectionType, type Library } from "@types";

  let open = $state(true);

  let libraryPath = $state($library.libraryPath || "/");
  let romDir = $state($library.romDir || "roms");
  let emulatorDir = $state($library.emulatorDir || "emulators");
  let biosDir = $state($library.biosDir || "bios");

  const canSave = $derived(!!libraryPath && !!romDir && !!emulatorDir && !!biosDir);

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

    $loadedLibrary = false;
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

<ModalBody maxWidth="24rem" headline={$isFirstSetup ? "Setup Library" : "Edit Library"} open={open} canClose={!$isFirstSetup} onclose={() => { $showEditLibraryModal = false }}>
  <div class="content">
    <PathField
      name="Library Path"
      placeholder="Library root folder"
      type={FileSelectionType.FOLDER}
      bind:value={libraryPath}
    />
    <TextField
      name="Roms Directory"
      placeholder="Library subfolder with ROMs"
      bind:value={romDir}
    />
    <TextField
      name="Emulators Directory"
      placeholder="Library subfolder with emulators"
      bind:value={emulatorDir}
    />
    <TextField
      name="Bios Files Directory"
      placeholder="Library subfolder with bios files"
      bind:value={biosDir}
    />
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel} disabled={$isFirstSetup}>Cancel</Button>
      <Button type="tonal" onclick={onConfirm} disabled={!canSave}>Save</Button>
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
</style>
