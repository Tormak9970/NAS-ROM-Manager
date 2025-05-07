<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, DateField, Select } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { showSearchFiltersModal } from "@stores/Modals";
  import { metadataSearchFilters, systems } from "@stores/State";

  let open = $state(true);

  let textQuery = $state("");
  let filterSystem = $state<string>("");
  let startReleaseDate = $state<string>("");
  let endReleaseDate = $state<string>("");
  let filterGenre = $state<string>("");
  let filterPublisher = $state<string>("");
  let filterDeveloper = $state<string>("");
  let filterFormat = $state<string>("");
  let startSize = $state<string>("");
  let endSize = $state<string>("");

  let systemOptions: SelectItem[] = Object.entries($systems).sort().map(([key, value]) => {
    return { label: key, value: value.abbreviation };
  });

  let genreOptions: SelectItem[] = Object.values($metadataSearchFilters.genres).sort().map((value: string) => {
    return { label: value, value: value };
  });
  let developerOptions: SelectItem[] = Object.values($metadataSearchFilters.developers).sort().map((value) => {
    return { label: value, value: value };
  });
  let publisherOptions: SelectItem[] = Object.values($metadataSearchFilters.publishers).sort().map((value) => {
    return { label: value, value: value };
  });

  
  function onCloseEnd() {
    $showSearchFiltersModal = false;
  }

  /**
   * Function to run on confirmation.
   */
  async function onSearch(): Promise<void> {
    // navigate to search and structure query params

    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
</script>

<ModalBody headline={"Search Filters"} open={open} oncloseend={onCloseEnd}>
  <div class="content">
    <TextField
      name="Search Query"
      bind:value={textQuery}
      extraWrapperOptions={{
        style: "width: 100%;"
      }}
    />
    <div class="search-row">
      <!-- Systems -->
      <Select
        name="Systems"
        options={systemOptions}
        disabled={systemOptions.length === 1}
        bind:value={filterSystem}
      />
    
      <!-- formats -->
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
    <div class="search-row">
      <!-- start-date -->
        <DateField
          name="Start Date"
        />
      <!-- end-date -->
    </div>
    <div class="search-row">
      <!-- start-size -->
      <!-- end-size -->
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

  .search-row {
    width: 100%;

    display: flex;
    gap: 0.5rem;
  }
</style>
