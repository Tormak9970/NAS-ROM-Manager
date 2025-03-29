<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { RestController } from "@controllers";
  import { Button, ProgressIndicator } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { downloadProgressRom, showDownloadProgressModal } from "@stores/Modals";
  import { showInfoSnackbar } from "@stores/State";
  import { formatFileSize } from "@utils";
  import { onMount } from "svelte";

  let open = $state(true);
  
  let prepping = $state(true);
  let downloadProgress = $state(0);
  let fileSize = $state(1);

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    RestController.cancelDownload();
    open = false;
  }

  function closeEnd() {
    $showDownloadProgressModal = false;
    $downloadProgressRom = null;
  }

  onMount(() => {
    RestController.downloadRom(
      $downloadProgressRom!,
      (size: number) => {
        fileSize = size;
        prepping = false;
      },
      (progress: number) => {
        downloadProgress = progress;
      },
      (finished: boolean) => {
        if (finished) $showInfoSnackbar({ message: "Download complete" });
        open = false;
      }
    );
  });
</script>

<ModalBody
  headline="Download Progress"
  open={open}
  canClose={false}
  oncloseend={closeEnd}
  extraOptions={{ style: "margin-bottom: 0rem" }}
>
  <div class="content">
    {#if prepping}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Loading ROM Metadata...</div>
      </div>
    {:else}
      <div class="download-container">
        <ProgressIndicator percent={downloadProgress / fileSize * 100} />
        <div class="info">{formatFileSize(downloadProgress)} / {formatFileSize(fileSize)}</div>
      </div>
    {/if}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="text" on:click={onCancel} warning>Cancel</Button>
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

  .download-container {
    width: 100%;

    display: flex;
    flex-direction: column;
    gap: 0.5rem;


    margin-top: 1rem;
  }
</style>
