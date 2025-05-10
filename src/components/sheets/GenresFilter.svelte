<script lang="ts">
  import { Button, Select } from "@interactables";
  import { BottomSheet } from "@layout";
  import { showGenresFilterSheet } from "@stores/Sheets";
  import { metadataSearchFilters, searchFilters } from "@stores/State";
  import { search } from "@utils";
  
  let filterGenre = $state<string>($searchFilters.genre ?? "");
  let genreOptions: SelectItem[] = Array.from($metadataSearchFilters.genres.values()).sort().map((value: string) => {
    return { label: value, value: value };
  });

  let open = $state(true);

  function onSubmit() {
    search({
      ...$searchFilters,
      genre: filterGenre
    });

    open = false;
  }

  function onClose() {
    $showGenresFilterSheet = false;
  }
</script>

<BottomSheet padding="0rem 1rem 1rem 1rem" open={open} onclose={onClose}>
  <div class="label m3-font-title-large">Select a Genre</div>
  <Select
    name="Genre"
    hideLabel
    placeholder="Genre"
    options={genreOptions}
    disabled={genreOptions.length === 1}
    bind:value={filterGenre}
    clearable
  />
  <div class="actions">
    <Button type="outlined" onclick={() => open = false}>Cancel</Button>
    <Button type="tonal" onclick={onSubmit}>Search</Button>
  </div>
</BottomSheet>

<style>
  .label {
    margin-bottom: 1rem;
    text-align: center;

    padding: 1rem 0;
  }

  .actions {
    margin-top: 1.5rem;
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
  }
</style>