<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, DateField, Select } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { showSearchFiltersModal } from "@stores/Modals";
  import { metadataSearchFilters, romFileFormats, systems } from "@stores/State";
  import { search } from "@utils";

  let open = $state(true);

  let textQuery = $state("");
  let filterSystem = $state<string>("");
  let startReleaseDate = $state<string>("");
  let endReleaseDate = $state<string>("");
  let filterGenre = $state<string>("");
  let filterPublisher = $state<string>("");
  let filterDeveloper = $state<string>("");
  let filterFormat = $state<string>("");

  let systemOptions: SelectItem[] = Object.entries($systems).sort().map(([key, value]) => {
    return { label: key, value: value.abbreviation };
  });

  let fileFormatOptions: SelectItem[] = [...$romFileFormats].sort().map((format: string) => {
    return { label: format, value: format };
  });

  let genreOptions: SelectItem[] = Array.from($metadataSearchFilters.genres.values()).sort().map((value: string) => {
    return { label: value, value: value };
  });
  let developerOptions: SelectItem[] = Array.from($metadataSearchFilters.developers.values()).sort().map((value) => {
    return { label: value, value: value };
  });
  let publisherOptions: SelectItem[] = Array.from($metadataSearchFilters.publishers.values()).sort().map((value) => {
    return { label: value, value: value };
  });

  
  function onCloseEnd() {
    $showSearchFiltersModal = false;
  }

  /**
   * Function to run on confirmation.
   */
  async function onSearch(): Promise<void> {
    search({
      textQuery: textQuery,
      system: filterSystem !== "" ? filterSystem : undefined,
      genre: filterGenre !== "" ? filterGenre : undefined,
      publisher: filterPublisher !== "" ? filterPublisher : undefined,
      developer: filterDeveloper !== "" ? filterDeveloper : undefined,
      format: filterFormat !== "" ? filterFormat : undefined,
      startReleaseDate: startReleaseDate,
      endReleaseDate: endReleaseDate,
    });

    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody
  headline={"Search Filters"}
  open={open}
  oncloseend={onCloseEnd}
  maxWidth="40rem"
>
  <div class="content">
    <div class="search-row">
      <TextField
        name="Search Query"
        bind:value={textQuery}
        extraWrapperOptions={{
          style: "width: 100%;"
        }}
      />
    </div>
    <div class="search-row">
      <Select
        name="System"
        options={systemOptions}
        disabled={systemOptions.length === 1}
        bind:value={filterSystem}
      />
      <Select
        name="File Format"
        options={fileFormatOptions}
        disabled={fileFormatOptions.length === 1}
        bind:value={filterFormat}
      />
    </div>
    <div class="search-row">
      <Select
        name="Genre"
        options={genreOptions}
        disabled={genreOptions.length <= 1}
        bind:value={filterGenre}
      />
      <Select
        name="Developer"
        options={developerOptions}
        disabled={developerOptions.length <= 1}
        bind:value={filterDeveloper}
      />
      <Select
        name="Publisher"
        options={publisherOptions}
        disabled={publisherOptions.length <= 1}
        bind:value={filterPublisher}
      />
    </div>
    <div class="row-title m3-font-title-small">RELEASE DATE RANGE</div>
    <div class="search-row">
      <DateField
        name="Start Date"
        bind:date={startReleaseDate}
      />
      <DateField
        name="End Date"
        bind:date={endReleaseDate}
      />
    </div>
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onSearch}>Search</Button>
    </div>
  {/snippet}
</ModalBody>

<style>
  .content {
    width: 100%;
  }

  .row-title {
    font-weight: bold;
  }

  .search-row {
    width: 100%;

    display: flex;
    gap: 0.5rem;

    margin-bottom: 1rem;
  }
</style>
