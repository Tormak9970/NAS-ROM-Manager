<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { RestController, SGDBController, WebsocketController } from "@controllers";
  import { Button, ProgressIndicator } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { editIsPostUpload, romEditingId, showEditRomModal, showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { romMetadata, roms, romsBySystem, showInfoSnackbar, showWarningSnackbar } from "@stores/State";
  import { formatFileSize, hash64 } from "@utils";
  import { onMount } from "svelte";

  let open = $state(true);
  
  let step = $state<"prep" | "upload" | "processing">("prep");
  let uploadProgress = $state(0);
  let fileSize = $uploadProgressConfig!.file.size;

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    await RestController.cancelUpload();
    open = false;
  }

  function closeEnd() {
    $showUploadProgressModal = false;
    $uploadProgressConfig = null;
  }

  async function processRom(success: boolean, romPath: string) {
    if (!success) {
      $showWarningSnackbar({ message: "Upload failed with unkown error" });
      onCancel();
      return;
    }
    
    const { system } = $uploadProgressConfig!;

    step = "processing";

    const rom = await WebsocketController.parseAddedRom(system, romPath);
    const id = hash64(rom.path);
    
    if (!$romsBySystem[rom.system].includes(id)) {
      $roms[id] = rom;
      $romsBySystem[rom.system].push(id);

      $romMetadata[id] = {
        title: rom.title,
        fullCapsulePath: "",
        thumbCapsulePath: "",
        sgdbId: "",
        igdbId: "",
        heroPath: "",
        metadata: null,
        isFavorite: false,
      }

      if ($romMetadata[id].sgdbId === "") {
        $romMetadata[id].sgdbId = await SGDBController.chooseSteamGridGameId(id, $romMetadata[id].title);
      }

      $roms = { ...$roms };
      $romMetadata = { ...$romMetadata };
      $romsBySystem = { ...$romsBySystem };
    }

    $showInfoSnackbar({ message: "Upload complete" });
    onCancel();
    $showEditRomModal = true;
    $editIsPostUpload = true;
    $romEditingId = id;
  }

  onMount(() => {
    RestController.uploadRom(
      $uploadProgressConfig!,
      () => step = "upload",
      (progress: number) => uploadProgress = progress,
      processRom,
    );
  });
</script>

<ModalBody
  headline="Upload Progress"
  open={open}
  canClose={false}
  oncloseend={closeEnd}
  extraOptions={{ style: "margin-bottom: 0rem" }}
>
  <div class="content">
    {#if step === "prep"}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Setting Upload Metadata...</div>
      </div>
    {:else if step === "upload"}
      <div class="upload-container">
        <ProgressIndicator percent={uploadProgress / fileSize * 100} />
        <div class="info">{formatFileSize(uploadProgress)} / {formatFileSize(fileSize)}</div>
      </div>
    {:else}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Processing ROM...</div>
      </div>
    {/if}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel} warning>Cancel</Button>
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

  .loading-container {
    margin: 0rem 1rem;
    margin-top: 1rem;
  }

  .upload-container {
    width: 100%;

    display: flex;
    flex-direction: column;
    gap: 0.5rem;


    margin-top: 1rem;
  }
</style>
