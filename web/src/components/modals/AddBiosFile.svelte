<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, FileField, Select } from "@interactables";
  import { RestService } from "@services";
  import { addBiosFileSystem, showAddBiosFileModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { showInfoSnackbar, systems } from "@stores/State";

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
    
    const system = $addBiosFileSystem!;

    $uploadProgressConfig = {
      config: {
        system: system,
        file: file!,
        needsUnzip: false
      },
      onCancel: RestService.cancelBIOSUpload,
      upload: RestService.uploadBIOS,
      process: async (_, closeModal) => {
        $systems[system].biosFiles.push(file!.name);
        $systems = { ...$systems };

        $showInfoSnackbar({ message: "Upload complete" });
        
        closeModal();
      }
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
  headline="Add Bios File"
  open={open}
  onclose={() => {
    $showAddBiosFileModal = false;
    $addBiosFileSystem = null;
  }}
>
  <div class="content">
    <Select name="System" options={systemOptions} disabled bind:value={$addBiosFileSystem!} />
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
