<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, Checkbox, FileField, Select, Toggle } from "@interactables";
  import { showAddRomModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { libraries, systems } from "@stores/State";

  let open = $state(true);
  
  let libraryOptions: SelectItem[] = Object.keys($libraries).map((key) => {
    return { label: key, value: key };
  });

  let systemOptions: SelectItem[] = Object.keys($systems).map((key) => {
    return { label: key, value: key };
  });

  let library = $state(libraryOptions[0].value);
  let system = $state(systemOptions[0].value);
  let file = $state<File | null>(null);
  
  let isZip = $derived(file && file.name.endsWith(".zip"));
  let needsUnzip = $state(false);
  let okStructure = $state(false);

  let canUpload = $derived(!!library && !!system && !!file && (!isZip || okStructure));

  /**
   * Function to run on confirmation.
   */
  async function onUpload(): Promise<void> {
    open = false;

    $uploadProgressConfig = {
      libraryPath: $libraries[library].path,
      system: system,
      file: file!,
      needsUnzip: needsUnzip
    }
    $showUploadProgressModal = true;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline="Add ROM" open={open} canClose={false} on:closeEnd={() => { $showAddRomModal = false }}>
  <div class="content">
    <Select name="Library" options={libraryOptions} disabled={libraryOptions.length === 1} bind:value={library} />
    <Select name="System" options={systemOptions} disabled={systemOptions.length === 1} bind:value={system} />
    <FileField name="File" />
    {#if file && isZip}
      <label>
        <div class="m3-font-title-medium">Unzip After Upload:</div>
        <Toggle bind:checked={needsUnzip} />
      </label>
      <label>
        <div class="m3-font-title-medium">This zip has <a href="">one root folder</a>:</div>
        <Checkbox bind:checked={okStructure} />
      </label>
      <!-- TODO: checkbox verifying it has a single root -->
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onUpload} disabled={!canUpload}>Upload</Button>
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
