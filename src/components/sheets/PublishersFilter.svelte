<script lang="ts">
  import { Button, Select } from "@interactables";
  import { BottomSheet } from "@layout";
  import { showPublishersFilterSheet } from "@stores/Sheets";
  import { metadataSearchFilters, searchFilters } from "@stores/State";
  import { search } from "@utils";
  
  let filterPublisher = $state<string>($searchFilters.publisher ?? "");
  let publisherOptions: SelectItem[] = Array.from($metadataSearchFilters.publishers.values()).sort().map((value: string) => {
    return { label: value, value: value };
  });

  let open = $state(true);

  function onSubmit() {
    search({
      ...$searchFilters,
      publisher: filterPublisher
    });

    open = false;
  }

  function onClose() {
    $showPublishersFilterSheet = false;
  }
</script>

<BottomSheet padding="0rem 1rem 1rem 1rem" open={open} onclose={onClose}>
  <div class="label m3-font-title-large">Select a Publisher</div>
  <Select
    name="Publisher"
    hideLabel
    placeholder="Publisher"
    options={publisherOptions}
    disabled={publisherOptions.length === 1}
    bind:value={filterPublisher}
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