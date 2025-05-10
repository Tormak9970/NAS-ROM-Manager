<script lang="ts">
  import { Button, Select } from "@interactables";
  import { BottomSheet } from "@layout";
  import { showFormatsFilterSheet } from "@stores/Sheets";
  import { fileFormatsBySystem, searchFilters } from "@stores/State";
  import { search } from "@utils";
  
  let filterFormat = $state<string>($searchFilters.format ?? "");
  let filterSystem = $state<string>($searchFilters.system ?? "");
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

  let open = $state(true);

  function onSubmit() {
    search({
      ...$searchFilters,
      format: filterFormat,
      system: filterSystem
    });

    open = false;
  }

  function onClose() {
    $showFormatsFilterSheet = false;
  }
</script>

<BottomSheet padding="0rem 1rem 1rem 1rem" open={open} onclose={onClose}>
  <div class="label m3-font-title-large">Select a Format</div>
  <Select
    name="Format"
    hideLabel
    placeholder="Format"
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