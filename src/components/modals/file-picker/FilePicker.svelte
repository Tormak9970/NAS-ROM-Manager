<script lang="ts">
  import { Icon, ModalBody } from "@component-utils";
  import { WebsocketController } from "@controllers";
  import { BackArrow } from "@icons";
  import { Button } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { LoadingSpinner } from "@layout";
  import { filePickerCancel, filePickerConfig, filePickerConfirm, showFilePickerModal } from "@stores/Modals";
  import { isRegEx, type FilePickerEntry } from "@types";
  import Entry from "./Entry.svelte";

  let config = $filePickerConfig!;

  let open = $state(true);

  let loading = $state(true);
  let entries = $state<FilePickerEntry[]>([]);
  let lastPath = $state(config.startPath);
  let currentPath = $state(config.startPath);

  const sortEntries = (a: FilePickerEntry, b: FilePickerEntry) => {
    if (a.isDir && b.isDir) {
      return a.name < b.name ? -1 : (a.name > b.name) ? 1 : 0;
    }

    if (!a.isDir && !b.isDir) {
      return a.name < b.name ? -1 : (a.name > b.name) ? 1 : 0;
    }

    if (a.isDir && !b.isDir) {
      return -1;
    } else {
      return 1;
    }
  }

  $effect(() => {
    loading = true;
    WebsocketController.getFilePickerEntries(currentPath, config).then((results: FilePickerEntry[]) => {
      let filtered = results.map((entry: FilePickerEntry) => {
        return {
          name: entry.name,
          path: entry.path.replaceAll(/\\/g, "/"),
          isDir: entry.isDir
        }
      });

      if (config.filter) {
        const filter = config.filter;
        filtered = isRegEx(filter)
          ? filtered.filter(entry => filter.test(entry.path))
          : filtered.filter(filter);
      }

      entries = filtered.sort(sortEntries);

      loading = false;
    });
  });

  function select(path: string) {
    lastPath = currentPath;
    currentPath = path;
  }

  function goBack() {
    currentPath = lastPath;
  }
  
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
    await $filePickerConfirm([currentPath!]);
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

<ModalBody headless open={open} canClose={false} on:closeEnd={onCloseEnd}>
  <div class="content">
    <div class="header">
      <Button iconType="full" type="text" disabled={lastPath === currentPath} on:click={goBack}>
        <Icon icon={BackArrow} />
      </Button>
      <TextField
        name=""
        value={currentPath}
        readonly
        extraWrapperOptions={{
          style: "height: 2.5rem; width: 100%;"
        }}
      />
    </div>
    {#if loading}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Loading...</div>
      </div>
    {:else}
      <div class="entries">
        {#each entries as entry, i (entry.path)}
          <Entry
            entry={entry}
            index={i}
            on:select={() => select(entry.path)}
          />
        {/each}
      </div>
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onConfirm} disabled={loading || !currentPath}>Choose</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 100%;
  }

  .loading-container {
    width: 100%;

    display: flex;
    align-items: center;
    gap: 20px;

    margin: 0rem 1rem;
    margin-top: 2rem;
  }

  .header {
    width: calc(100% + 0.5rem);

    margin-left: -0.5rem;

    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .entries {
    width: 100%;
    height: 15rem;

    margin-top: 1rem;

    border-radius: var(--m3-util-rounding-extra-small);
    overflow: hidden;
    overflow-y: scroll;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
