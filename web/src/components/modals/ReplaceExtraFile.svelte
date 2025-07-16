<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, FileField } from "@interactables";
  import { RestService } from "@services";
  import { replaceExtraFilePath, replaceExtraFileRomId, replaceExtraFileSystem, replaceExtraFileType, showReplaceExtraFileModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { library, showInfoSnackbar } from "@stores/State";
  import { ExtraFileType } from "@types";

  let open = $state(true);

  let file = $state<File | null>(null);

  const canUpload = $derived(!!file);

  /**
   * Function to run on confirmation.
   */
  async function onUpload(): Promise<void> {
    open = false;
    
    const system = $replaceExtraFileSystem!;
    const romId = $replaceExtraFileRomId!;
    const isDLC = $replaceExtraFileType === ExtraFileType.DLC;

    $uploadProgressConfig = {
      config: {
        uploadFolder: $library[isDLC ? "dlcDir" : "updateDir"],
        romId: romId,
        system: system,
        file: file!,
        needsUnzip: false,
        path: $replaceExtraFilePath!
      },
      process: async (_, closeModal) => {
        $showInfoSnackbar({ message: "Replaced file" });
        closeModal();
      },
      complete: RestService.uploadROMExtraComplete,
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
  headline={`Replace ${$replaceExtraFileType === ExtraFileType.DLC ? "DLC" : "Update"} File`}
  open={open}
  onclose={() => {
    $showReplaceExtraFileModal = false;
    $replaceExtraFileType = ExtraFileType.DLC;
    $replaceExtraFileSystem = null;
    $replaceExtraFileRomId = null;
    $replaceExtraFilePath = null;
  }}
>
  <div class="content">
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
