<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { ApiController } from "@controllers/ApiController";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { downloadProgressRom, showDownloadProgressModal } from "@stores/Modals";
  import { formatFileSize } from "@utils";
  import { onMount } from "svelte";

  let open = $state(true);
  
  let prepping = $state(true);
  let downloadProgress = $state(0);

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }

  function closeEnd() {
    $showDownloadProgressModal = false;
    $downloadProgressRom = null;
  }

  onMount(() => {
    ApiController.downloadRom(
      $downloadProgressRom!,
      (fileSize: number) => {
        console.log("file size is:", formatFileSize(fileSize));
      },
      (progress: number) => {

      },
      () => {

      }
    );
  });
</script>

<ModalBody headline="Download Progress" open={open} canClose={false} on:closeEnd={closeEnd}>
  <div class="content">
    {#if prepping}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Loading ROM Metadata...</div>
      </div>
    {:else}
      
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 350px;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .loading-container {
    width: 100%;

    display: flex;
    align-items: center;
    gap: 20px;

    margin: 0rem 1rem;
    margin-top: 1rem;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
