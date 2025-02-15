<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { AppController } from "@controllers";
  import { Button, PathField, TextField, Toggle } from "@interactables";
  import { showAddLibraryModal } from "@stores/Modals";
  import { FileSelectionType, type Library } from "@types";

  let open = $state(true);

  let libraryName = $state("");
  let libraryPath = $state("/");
  let useDefaultParsers = $state(true);
  let parsersPath = $state("/");

  let canSave = $derived(!!libraryName && !!libraryPath && (useDefaultParsers || !!parsersPath));

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    open = false;

    const newLibrary: Library = {
      name: libraryName,
      path: libraryPath,
      useProvidedParsers: useDefaultParsers,
      parsersPath: useDefaultParsers ? "" : parsersPath,
      romCustomizations: []
    }

    AppController.addLibrary(newLibrary);
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline="Add Library" open={open} canClose={false} on:closeEnd={() => { $showAddLibraryModal = false }}>
  <div class="content">
    <TextField name="Library Name" bind:value={libraryName} />
    <PathField name="Library Path" type={FileSelectionType.FOLDER} bind:value={libraryPath} />
    <label class="toggle-label">
      <div class="m3-font-title-medium">Use Default Parsers:</div>
      <Toggle bind:checked={useDefaultParsers} />
    </label>
    {#if !useDefaultParsers}
      <PathField name="Parser Directory" type={FileSelectionType.FOLDER} bind:value={parsersPath} />
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
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

  .toggle-label {
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
