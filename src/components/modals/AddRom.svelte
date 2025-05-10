<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, Checkbox, FileField, Select } from "@interactables";
  import { addRomSystem, showAddRomModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { systems } from "@stores/State";

  let open = $state(true);

  let systemOptions: SelectItem[] = Object.entries($systems).sort().map(([key, value]) => {
    return { label: key, value: value.abbreviation };
  });
  
  if ($addRomSystem === "") {
    $addRomSystem = systemOptions[0].value;
  }

  let file = $state<File | null>(null);
  
  const isZip = $derived(file && file.name.endsWith(".zip"));
  let needsUnzip = $state(false);
  let okStructure = $state(false);

  const canUpload = $derived(!!file && (!isZip || okStructure));

  /**
   * Function to run on confirmation.
   */
  async function onUpload(): Promise<void> {
    open = false;

    $uploadProgressConfig = {
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

<ModalBody
  headline="Add ROM"
  open={open}
  oncloseend={() => { $showAddRomModal = false }}
>
  <div class="content">
    <Select name="System" options={systemOptions} disabled={systemOptions.length === 1} bind:value={$addRomSystem} />
    <FileField name="File" onchange={(value) => file = value!} />
    {#if file && isZip}
      <label>
        <div class="m3-font-title-medium">Unzip after upload:</div>
        <Checkbox bind:checked={needsUnzip} />
      </label>
      {#if needsUnzip}
        <label>
          <div class="m3-font-title-medium">Zip has <a href="https://github.com/Tormak9970/NAS-ROM-Manager?tab=readme-ov-file#single-root-folder" rel="noopener noreferrer" target="_blank">one root folder</a>:</div>
          <Checkbox bind:checked={okStructure} />
        </label>
      {/if}
    {/if}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onUpload} disabled={!canUpload}>Upload</Button>
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
