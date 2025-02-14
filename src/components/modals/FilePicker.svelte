<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { WebsocketController } from "@controllers";
  import { Button } from "@interactables";
  import { filePickerCancel, filePickerConfig, filePickerConfirm, showFilePickerModal } from "@stores/Modals";
  import { isRegEx, type FilePickerEntry } from "@types";

  let config = $filePickerConfig!;

  let open = $state(true);

  let loading = $state(true);
  let selected = $state<string | null>(null);
  let entries = $state<FilePickerEntry[]>([]);
  let currentPath = $state(config.startPath);

  $effect(() => {
    loading = true;
    WebsocketController.getFilePickerEntries(currentPath, config).then((results: FilePickerEntry[]) => {
      let filtered = results;

      if (config.filter) {
        const filter = config.filter;
        filtered = isRegEx(filter)
          ? filtered.filter(entry => filter.test(entry.path))
          : filtered.filter(filter);
      }

      entries = filtered;

      loading = false;
    })
  });


  // WebsocketController.getFilePickerEntries
  // Filter results with config.filter
  
  function onCloseEnd() {
    $showFilePickerModal = false;
    $filePickerConfig = null;
    $filePickerConfirm = async (paths: string[]) => {};
    $filePickerCancel = async () => {};
  }

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    await $filePickerConfirm([selected!]);
    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    await $filePickerCancel();
    open = false;
  }
</script>

<ModalBody headline={"Need to hide"} open={open} canClose={false} on:closeEnd={onCloseEnd}>
  <div class="content">
    <div class="current-path">

    </div>
    {#if loading}
      <div class="loading">

      </div>
    {:else}
      <div class="entries">

      </div>
      "Test"
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onConfirm} disabled={!!selected}>Choose</Button>
  </div>
</ModalBody>

<style>
  .content {
    max-width: 300px;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
