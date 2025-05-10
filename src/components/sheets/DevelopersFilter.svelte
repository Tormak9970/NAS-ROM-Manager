<script lang="ts">
  import { Button, Select } from "@interactables";
  import { BottomSheet } from "@layout";
  import { showDevelopersFilterSheet } from "@stores/Sheets";
  import { metadataSearchFilters, searchFilters } from "@stores/State";
  import { search } from "@utils";
  
  let filterDeveloper = $state<string>($searchFilters.developer ?? "");
  let developerOptions: SelectItem[] = Array.from($metadataSearchFilters.developers.values()).sort().map((value: string) => {
    return { label: value, value: value };
  });

  let open = $state(true);

  function onSubmit() {
    search({
      ...$searchFilters,
      developer: filterDeveloper
    });

    open = false;
  }

  function onClose() {
    $showDevelopersFilterSheet = false;
  }
</script>

<BottomSheet padding="0rem 1rem 1rem 1rem" open={open} onclose={onClose}>
  <div class="label m3-font-title-large">Select a Developer</div>
  <Select
    name="Developer"
    hideLabel
    placeholder="Developer"
    options={developerOptions}
    disabled={developerOptions.length === 1}
    bind:value={filterDeveloper}
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