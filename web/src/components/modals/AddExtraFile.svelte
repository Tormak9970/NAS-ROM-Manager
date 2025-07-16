<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, FileField } from "@interactables";
  import { RestService, WebsocketService } from "@services";
  import { addExtraFileRomId, addExtraFileSystem, addExtraFileType, showAddExtraFileModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { library, romDLCs, romUpdates, showInfoSnackbar } from "@stores/State";
  import { ExtraFileType } from "@types";

  let open = $state(true);

  let file = $state<File | null>(null);

  const canUpload = $derived(!!file);

  /**
   * Function to run on confirmation.
   */
  async function onUpload(): Promise<void> {
    open = false;
    
    const fileName = file!.name;
    const system = $addExtraFileSystem!;
    const romId = $addExtraFileRomId!;
    const fileType = $addExtraFileType;
    const isDLC = fileType === ExtraFileType.DLC;

    $uploadProgressConfig = {
      config: {
        uploadFolder: $library[isDLC ? "dlcDir" : "updateDir"],
        romId: romId,
        system: system,
        file: file!,
        needsUnzip: false
      },
      process: async (_, closeModal) => {
        if (isDLC) {
          if (!!$romDLCs[romId]) {
            $romDLCs[romId].push(fileName);
          } else {
            $romDLCs[romId] = [fileName];
          }
          $romDLCs = { ...$romDLCs };
        } else {
          if (!!$romUpdates[romId]) {
            $romUpdates[romId].push(fileName);
          } else {
            $romUpdates[romId] = [fileName];
          }
          $romUpdates = { ...$romUpdates };
        }

        $showInfoSnackbar({ message: "Upload complete" });
        
        closeModal();
      },
      complete: async (data) => {
        const res = await RestService.uploadROMExtraComplete(data);
        await WebsocketService.addExtraFileToCache(fileType, romId, fileName);

        return res;
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
  headline={`Add ${$addExtraFileType === ExtraFileType.DLC ? "DLC" : "Update"} File`}
  open={open}
  onclose={() => {
    $showAddExtraFileModal = false;
    $addExtraFileType = ExtraFileType.DLC;
    $addExtraFileSystem = null;
    $addExtraFileRomId = null;
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
