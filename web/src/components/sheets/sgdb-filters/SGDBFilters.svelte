<script lang="ts">
  import { scrollShadow } from "@directives";
  import { SideSheet } from "@layout";
  import { changeGridsType } from "@stores/Modals";
  import { showSGDBFiltersSheet } from "@stores/Sheets";
  import { dbFilters } from "@stores/State";
  import type { DBFilter } from "@types";
  import FilterSection from "./FilterSection.svelte";
  
  const sections = $derived(Object.keys($dbFilters[$changeGridsType]) as (keyof DBFilter)[]);
</script>

<SideSheet width={300} onclose={() => $showSGDBFiltersSheet = false}>
  <div class="label m3-font-title-large">SGDB Filters</div>
  <div class="filters-wrapper">
    <div class="filters styled-scrollbar" use:scrollShadow={{ background: "--m3-scheme-surface-container-low" }}>
      <div class="filters-inner">
        {#each sections as section}
          <FilterSection section={section} />
        {/each}
      </div>
    </div>
  </div>
</SideSheet>

<style>
  .label {
    margin-bottom: 1rem;
  }

  .filters-wrapper {
    width: 100%;
    height: calc(100% - 50px);
    
    position: relative;
  }

  .filters {
    width: 100%;
    height: 100%;

    overflow-x: hidden;
    overflow-y: auto;
  }

  .filters-inner {
    width: calc(100% - 0.5rem);
    
    display: flex;
    flex-direction: column;

    gap: 1.25rem;
  }
</style>