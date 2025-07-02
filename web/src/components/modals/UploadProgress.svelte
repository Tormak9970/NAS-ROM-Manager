<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, ProgressIndicator } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { UploadService } from "@services";
  import { showUploadProgressModal, uploadProgressConfig } from "@stores/Modals";
  import { showWarningSnackbar } from "@stores/State";
  import { formatFileSize } from "@utils";
  import { onMount } from "svelte";

  let open = $state(true);
  
  let step = $state<"prep" | "upload" | "processing">("prep");
  let uploadProgress = $state(0);
  let fileSize = $uploadProgressConfig!.config.file.size;

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    await UploadService.cancelUpload();
    open = false;
  }

  function closeEnd() {
    $showUploadProgressModal = false;
    $uploadProgressConfig = null;
  }

  async function processUpload(success: boolean, filePath: string) {
    if (!success) {
      $showWarningSnackbar({ message: "Upload failed with unkown error" });
      onCancel();
      return;
    }

    step = "processing";

    await $uploadProgressConfig!.process(filePath, () => open = false);
  }

  onMount(() => {
    UploadService.upload(
      $uploadProgressConfig!.config,
      () => step = "upload",
      (progress: number) => uploadProgress = progress,
      $uploadProgressConfig!.complete,
      processUpload,
      $uploadProgressConfig!.isReplace
    ).then((isCanceled: boolean) => {
      if (isCanceled) {
        open = false;
      }
    });
  });
</script>

<ModalBody
  headline="Upload Progress"
  open={open}
  canClose={false}
  onclose={closeEnd}
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
        <LoadingSpinner /> <div class="font-headline-small">Processing Upload...</div>
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
