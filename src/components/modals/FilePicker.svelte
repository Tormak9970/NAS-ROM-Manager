<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { WebsocketController } from "@controllers";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { filePickerCancel, filePickerConfig, filePickerConfirm, showFilePickerModal } from "@stores/Modals";
  import { isRegEx, type FilePickerEntry } from "@types";

  let config = $filePickerConfig!;

  let open = $state(true);

  let loading = $state(true);
  let selected = $state<string | null>(null);
  let entries = $state<FilePickerEntry[]>([]);
  let currentPath = $state(config.startPath);

  // TODO: need to verify this works properly.
  const sortEntries = (a: FilePickerEntry, b: FilePickerEntry) => {
    if (a.isDir && b.isDir) {
      return a.name > b.name ? 1 : (a.name < b.name) ? -1 : 0;
    }

    if (!a.isDir && !b.isDir) {
      return a.name > b.name ? 1 : (a.name < b.name) ? -1 : 0;
    }

    if (a.isDir && !b.isDir) {
      return 1;
    } else {
      return -1;
    }
  }

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

      entries = filtered.sort(sortEntries);

      loading = false;
    })
  });

  function toggleSelectPath(path: string) {
    if (selected === path) {
      selected = null;
      return;
    }

    selected = path;
  }

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
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Loading...</div>
      </div>
    {:else}
      <div class="entries">
        {#each entries as entry (entry.path)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <!-- svelte-ignore event_directive_deprecated -->
          <div class="entry" class:selected={entry.path === selected} on:click={() => toggleSelectPath(entry.path)}>
            {entry.name}
          </div>
        {/each}
      </div>
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onConfirm} disabled={loading || !!selected}>Choose</Button>
  </div>
</ModalBody>

<style>
  .content {
    max-width: 300px;
  }

  .loading-container {
    width: 100%;

    display: flex;
    align-items: center;
    gap: 20px;

    margin: 0rem 1rem;
    margin-top: 1rem;
  }

  .current-path {

  }

  .entries {

  }

  .entry {

  }

  .selected {

  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
