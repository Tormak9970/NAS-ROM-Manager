<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, FileField, Select } from "@interactables";
  import { RestService } from "@services";
  import { replaceBiosFilePath, replaceBiosFileSystem, showReplaceBiosFileModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { library, showInfoSnackbar, systems } from "@stores/State";

  let open = $state(true);
  
  let systemOptions: SelectItem[] = Object.entries($systems).sort().map(([key, value]) => {
    return { label: key, value: value.abbreviation };
  });

  let file = $state<File | null>(null);

  const canUpload = $derived(!!file);

  /**
   * Function to run on confirmation.
   */
  async function onUpload(): Promise<void> {
    open = false;
    
    const system = $replaceBiosFileSystem!;

    $uploadProgressConfig = {
      config: {
        uploadFolder: $library.biosDir,
        system: system,
        file: file!,
        needsUnzip: false,
        path: $replaceBiosFilePath!
      },
      process: async (_, closeModal) => {
        $showInfoSnackbar({ message: "Replaced file" });
        closeModal();
      },
      complete: RestService.uploadBIOSComplete,
      isReplace: true,
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
  headline="Replace Bios File"
  open={open}
  onclose={() => {
    $showReplaceBiosFileModal = false;
    $replaceBiosFileSystem = null;
  }}
>
  <div class="content">
    <Select name="System" options={systemOptions} disabled bind:value={$replaceBiosFileSystem!} />
    <FileField name="File" placeholder="Choose a file" onchange={(value) => file = value!} />
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
</style>
