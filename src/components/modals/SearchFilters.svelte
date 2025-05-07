<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, DateField, Select } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { showSearchFiltersModal } from "@stores/Modals";
  import { systems } from "@stores/State";

  let open = $state(true);

  let textQuery = $state("");
  let filterSystem = $state<string>("");
  let startReleaseDate = $state<string>("");
  let endReleaseDate = $state<string>("");
  let filterGenres = $state<string[]>([]);
  let filterPublishers = $state<string[]>([]);
  let filterDevelopers = $state<string[]>([]);
  let filterFormats = $state<string[]>([]);
  let startSize = $state<string>("");
  let endSize = $state<string>("");

  let systemOptions: SelectItem[] = Object.entries($systems).sort().map(([key, value]) => {
    return { label: key, value: value.abbreviation };
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
      <!-- genres -->
      <!-- Developers -->
      <!-- Publishers -->
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
</style>
