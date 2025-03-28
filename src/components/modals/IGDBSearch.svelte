<script lang="ts">
  import { ModalBody } from "@component-utils";
  import GameTitleEntry from "@component-utils/GameTitleEntry.svelte";
  import { IGDBController } from "@controllers";
  import { Button } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { LoadingSpinner } from "@layout";
  import { igdbSearchOnSelect, igdbSearchPlatformId, igdbSearchTitle, showSearchIGDBModal } from "@stores/Modals";
  import { type IGDBSearchResult } from "@types";
  import { debounce } from "@utils";

  let open = $state(true);

  let loading = $state(true);
  let entries = $state<IGDBSearchResult[]>([]);

  let gameTitle = $state($igdbSearchTitle!);
  let selectedId = $state("");
  
  const debouncedSearch = debounce((gameTitle: string, platformId: string) => {
    IGDBController.searchForGame(gameTitle, platformId).then((results: IGDBSearchResult[]) => {
      entries = results;
      loading = false;
    });
  }, 500);

  $effect(() => {
    loading = true;
    debouncedSearch(gameTitle, $igdbSearchPlatformId!);
  });

  function select(id: number) {
    selectedId = id.toString();
  }
  
  function onCloseEnd() {
    $showSearchIGDBModal = false;
    $igdbSearchTitle = null;
    $igdbSearchPlatformId = null;
    $igdbSearchOnSelect = () => {};
  }

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    $igdbSearchOnSelect(selectedId);
    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline={"IGDB Search Results"} open={open} on:closeEnd={onCloseEnd}>
  <div class="content">
    <TextField
      name="Search Query"
      bind:value={gameTitle}
      extraWrapperOptions={{
        style: "width: 100%;"
      }}
    />
    {#if loading}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Loading...</div>
      </div>
    {:else}
      <div class="entries">
        {#each entries as entry, i (entry.igdbId)}
          <GameTitleEntry
            name={entry.name}
            index={i}
            selected={selectedId === entry.igdbId.toString()}
            onSelect={() => select(entry.igdbId)}
          />
        {:else}
          <div class="message-container">
            <div class="message">No results found.</div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
  <div slot="buttons">
    <Button type="tonal" on:click={onCancel}>Cancel</Button>
    <Button type="tonal" on:click={onConfirm} disabled={loading || !selectedId}>Choose</Button>
  </div>
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
    overflow-y: scroll;
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
