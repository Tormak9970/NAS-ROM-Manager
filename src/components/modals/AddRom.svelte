<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, Checkbox, FileField, Select } from "@interactables";
  import { addRomLibrary, addRomSystem, showAddRomModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { libraries, systems } from "@stores/State";
  import { systemToParser } from "@utils";

  let open = $state(true);
  
  let libraryOptions: SelectItem[] = Object.keys($libraries).sort().map((key) => {
    return { label: key, value: key };
  });

  let systemOptions: SelectItem[] = Object.keys($systems).sort().map((key) => {
    return { label: key, value: systemToParser(key) };
  });

  if ($addRomLibrary === "") {
    $addRomLibrary = libraryOptions[0].value;
  }
  
  if ($addRomSystem === "") {
    $addRomSystem = systemOptions[0].value;
  }

  let file = $state<File | null>(null);
  
  let isZip = $derived(file && file.name.endsWith(".zip"));
  let needsUnzip = $state(false);
  let okStructure = $state(false);

  let canUpload = $derived(!!file && (!isZip || okStructure));

  /**
   * Function to run on confirmation.
   */
  async function onUpload(): Promise<void> {
    open = false;

    $uploadProgressConfig = {
      library: $addRomLibrary,
      system: $addRomSystem,
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
    <Select name="Library" options={libraryOptions} disabled={libraryOptions.length === 1} bind:value={$addRomLibrary} />
    <Select name="System" options={systemOptions} disabled={systemOptions.length === 1} bind:value={$addRomSystem} />
    <FileField name="File" on:change={(e) => file = e.detail.value} />
    {#if file && isZip}
      <label>
        <div class="m3-font-title-medium">Unzip after upload:</div>
        <Checkbox bind:checked={needsUnzip} />
      </label>
      <label>
        <div class="m3-font-title-medium">Zip has <a href="https://github.com/Tormak9970/NAS-ROM-Manager?tab=readme-ov-file#single-root-folder" rel="noopener noreferrer" target="_blank">one root folder</a>:</div>
        <Checkbox bind:checked={okStructure} />
      </label>
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
