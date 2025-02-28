<script lang="ts">
  import { Toggle } from "@interactables";
  import { dbFilters } from "@stores/State";
  import type { DBFilter } from "@types";
  import { toUpperCaseSplit } from "@utils";

  type Props = {
    section: keyof DBFilter;
  }

  let { section }: Props = $props();

  let label = $derived(section === "oneoftag" ? "Tags" : toUpperCaseSplit(section));

  /**
   * Creates a function to update the specified filter.
   * @param section The section of the filter to update.
   * @param filter The filter to update.
   * @returns A function to update the filter.
   */
  function updateFilters(section: keyof DBFilter, filter: string): (e: any) => void {
    return (e: any) => {
      const value = e.detail.value;
      const filters = $dbFilters;

      filters[section][filter] = value;

      $dbFilters = { ...filters };
    }
  }
</script>

<div class="filter-section">
  <div class="section-label">{label}</div>
  <div class="filters">
    {#each Object.keys($dbFilters[section]) as filter}
      <label class="toggle-container m3-font-title-medium">
        {toUpperCaseSplit(filter)}
        <Toggle
          checked={$dbFilters[section][filter]}
          on:change={updateFilters(section, filter)}
        />
      </label>
    {/each}
  </div>
</div>

<style>
  .filter-section {
    width: 100%;
  }

  .section-label {
    color: rgb(var(--m3-scheme-primary));
  }

  .filters {
    width: 100%;

    display: flex;
    flex-direction: column;

    gap: 0.25rem;
  }

  .toggle-container {
    width: 100%;

    display: flex;
    align-items: center;
    justify-content: space-between;

    font-size: 1.15rem;
  }
</style>