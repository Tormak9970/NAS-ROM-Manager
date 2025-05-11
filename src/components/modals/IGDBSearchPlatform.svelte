<script lang="ts">
  import { ModalBody } from "@component-utils";
  import GameTitleEntry from "@component-utils/GameTitleEntry.svelte";
  import { IGDBController } from "@controllers";
  import { Button } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { LoadingSpinner } from "@layout";
  import { igdbSearchPlatformOnSelect, igdbSearchPlatformTitle, showSearchIGDBRomModal } from "@stores/Modals";
  import { type IGDBMetadataPlatform } from "@types";
  import { debounce } from "@utils";

  let open = $state(true);

  let loading = $state(true);
  let entries = $state<IGDBMetadataPlatform[]>([]);

  let platformTitle = $state($igdbSearchPlatformTitle!);
  let selectedPlatform = $state<IGDBMetadataPlatform | null>(null);
  
  const debouncedSearch = debounce((gameTitle: string) => {
    IGDBController.searchForPlatform(gameTitle).then((results: IGDBMetadataPlatform[]) => {
      entries = results;
      loading = false;
    });
  }, 500);

  $effect(() => {
    loading = true;
    debouncedSearch(platformTitle);
  });

  function select(platform: IGDBMetadataPlatform) {
    selectedPlatform = platform;
  }
  
  function onCloseEnd() {
    $showSearchIGDBRomModal = false;
    $igdbSearchPlatformTitle = null;
    $igdbSearchPlatformOnSelect = () => {};
  }

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    $igdbSearchPlatformOnSelect(selectedPlatform);
    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline={"IGDB Search Results"} open={open} onclose={onCloseEnd}>
  <div class="content">
    <TextField
      name="Search Query"
      placeholder="Search for a system..."
      bind:value={platformTitle}
      extraWrapperOptions={{
        style: "width: 100%;"
      }}
    />
    {#if loading}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Loading...</div>
      </div>
    {:else}
      <div class="entries styled-scrollbar">
        {#each entries as entry, i (entry.igdbId)}
          <GameTitleEntry
            name={entry.name}
            index={i}
            selected={selectedPlatform?.igdbId === entry.igdbId}
            onSelect={() => select(entry)}
          />
        {:else}
          <div class="message-container">
            <div class="message">No results found.</div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onConfirm} disabled={loading || !selectedPlatform}>Choose</Button>
    </div>
  {/snippet}
</ModalBody>

<style>
  .content {
    width: 100%;
  }

  .loading-container {
    margin: 0rem 1rem;
    margin-top: 2rem;
  }

  .entries {
    width: 100%;
    height: 15rem;

    margin-top: 1rem;

    background-color: rgb(var(--m3-scheme-surface-container-lowest));
    border-radius: var(--m3-util-rounding-extra-small);

    overflow: hidden;
    overflow-y: auto;
  }

  .entries > :global(:last-child) {
    border-bottom: none;
  }

  .message-container {
    width: 100%;
    height: 100%;

    background-color: rgb(var(--m3-scheme-surface-container-lowest));

    display: flex;
    justify-content: center;
  }

  .message {
    margin-top: 2rem;
  }
</style>
