<script lang="ts">
  import { ModalBody } from "@component-utils";
  import GameTitleEntry from "@component-utils/GameTitleEntry.svelte";
  import { SGDBController } from "@controllers";
  import { Button } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { LoadingSpinner } from "@layout";
  import { sgdbSearchOnSelect, sgdbSearchTitle, showSearchSGDBModal } from "@stores/Modals";
  import { type SGDBGame } from "@types";
  import { debounce } from "@utils";

  let open = $state(true);

  let loading = $state(true);
  let entries = $state<SGDBGame[]>([]);

  let gameTitle = $state($sgdbSearchTitle!);
  let selectedId = $state("");

  const debouncedSearch = debounce(SGDBController.searchForGame, 500) as (title: string) => Promise<SGDBGame[]>;

  $effect(() => {
    loading = true;
    debouncedSearch(gameTitle).then((results: SGDBGame[]) => {
      entries = results;
      loading = false;
    });
  });

  function select(id: number) {
    selectedId = id.toString();
  }
  
  function onCloseEnd() {
    $showSearchSGDBModal = false;
    $sgdbSearchTitle = null;
    $sgdbSearchOnSelect = () => {};
  }

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    $sgdbSearchOnSelect(selectedId);
    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline={"SGDB Search Results"} open={open} on:closeEnd={onCloseEnd}>
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
        {#each entries as entry, i (entry.id)}
          <GameTitleEntry
            name={entry.name}
            index={i}
            selected={selectedId === entry.id.toString()}
            onSelect={() => select(entry.id)}
          />
        {/each}
      </div>
    {/if}
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onConfirm} disabled={loading || !selectedId}>Choose</Button>
  </div>
</ModalBody>

<style>
  .content {
    max-width: 400px;
  }

  .loading-container {
    width: 100%;

    display: flex;
    align-items: center;
    gap: 20px;

    margin: 0rem 1rem;
    margin-top: 2rem;
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

    margin: -0.5rem 0;
  }
</style>
