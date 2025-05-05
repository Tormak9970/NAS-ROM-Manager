<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button } from "@interactables";
  import TextField from "@interactables/TextField.svelte";
  import { showSearchFiltersModal } from "@stores/Modals";
  import { type System } from "@types";

  let open = $state(true);

  let textQuery = $state("");
  let systems = $state<System[]>([]);
  let startReleaseDate = $state<string>("");
  let endReleaseDate = $state<string>("");
  let genres = $state<string[]>([]);
  let publishers = $state<string[]>([]);
  let developers = $state<string[]>([]);
  let formats = $state<string[]>([]);
  let startSize = $state<string>("");
  let endSize = $state<string>("");

  
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
