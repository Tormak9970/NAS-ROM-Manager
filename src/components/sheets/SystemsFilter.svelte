<script lang="ts">
  import { Button, Select } from "@interactables";
  import { BottomSheet } from "@layout";
  import { showSystemsFilterSheet } from "@stores/Sheets";
  import { searchFilters, systems } from "@stores/State";
  import { search } from "@utils";
  
  let filterSystem = $state<string>($searchFilters.system ?? "");
  let systemOptions: SelectItem[] = Object.entries($systems).sort().map(([key, value]) => {
    return { label: key, value: value.abbreviation };
  });

  let open = $state(true);

  function onSubmit() {
    search({
      ...$searchFilters,
      system: filterSystem
    });

    open = false;
  }

  function onClose() {
    $showSystemsFilterSheet = false;
  }
</script>

<BottomSheet padding="0rem 1rem 1rem 1rem" open={open} onclose={onClose}>
  <div class="label m3-font-title-large">Select a System</div>
  <Select
    name="System"
    hideLabel
    placeholder="System"
    options={systemOptions}
    disabled={systemOptions.length === 1}
    bind:value={filterSystem}
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