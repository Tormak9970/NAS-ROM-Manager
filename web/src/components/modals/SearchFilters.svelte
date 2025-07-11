<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, DateField, Select } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { showSearchFiltersModal } from "@stores/Modals";
  import { fileFormatsBySystem, metadataSearchFilters, systems } from "@stores/State";
  import type { SearchQuery } from "@types";
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

  let systemFormatLookup: Record<string, string> = Object.entries($fileFormatsBySystem).reduce((lookup: Record<string, string>, [system, formats]) => {
    for (const format of formats) {
      lookup[format] = system;
    }

    return lookup;
  }, {});

  let fileFormatOptions: SelectItem[] = $derived.by(() => {
    const availableOptions = filterSystem !== "" ? $fileFormatsBySystem[filterSystem] : Object.values($fileFormatsBySystem).flat();

    return availableOptions.sort().map((format: string) => {
      return { label: format, value: format };
    });
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
    const filters: SearchQuery = {};

    if (textQuery !== "") {
      filters.textQuery = textQuery;
    }
    if (filterSystem !== "") {
      filters.system = filterSystem;
    }
    if (filterGenre !== "") {
      filters.genre = filterGenre;
    }
    if (filterPublisher !== "") {
      filters.publisher = filterPublisher;
    }
    if (filterDeveloper !== "") {
      filters.developer = filterDeveloper;
    }
    if (filterFormat !== "") {
      filters.format = filterFormat;
    }
    if (startReleaseDate !== "") {
      filters.startReleaseDate = startReleaseDate;
    }
    if (endReleaseDate !== "") {
      filters.endReleaseDate = endReleaseDate;
    }

    search(filters);

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
  onclose={onCloseEnd}
  maxWidth="40rem"
>
  <div class="content">
    <div class="search-row">
      <TextField
        name="Search Query"
        placeholder="Ex: Zelda"
        bind:value={textQuery}
        extraWrapperOptions={{
          style: "width: 100%;"
        }}
      />
    </div>
    <div class="search-row">
      <Select
        name="System"
        placeholder="Select a system"
        options={systemOptions}
        disabled={systemOptions.length === 1}
        bind:value={filterSystem}
        clearable
      />
      <Select
        name="File Format"
        placeholder="Select a file format"
        options={fileFormatOptions}
        disabled={fileFormatOptions.length === 1}
        bind:value={filterFormat}
        onSelect={(option) => {
          if (option) {
            filterSystem = systemFormatLookup[option.value];
          }
        }}
        clearable
      />
    </div>
    <div class="search-row">
      <Select
        name="Genre"
        placeholder="Game genre"
        options={genreOptions}
        disabled={genreOptions.length <= 1}
        bind:value={filterGenre}
        clearable
      />
      <Select
        name="Developer"
        placeholder="Game developer"
        options={developerOptions}
        disabled={developerOptions.length <= 1}
        bind:value={filterDeveloper}
        clearable
      />
      <Select
        name="Publisher"
        placeholder="Game publisher"
        options={publisherOptions}
        disabled={publisherOptions.length <= 1}
        bind:value={filterPublisher}
        clearable
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
