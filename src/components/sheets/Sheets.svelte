<script lang="ts">
  import { showSGDBFiltersSheet } from "@stores/Sheets";
  import SgdbFilters from "./sgdb-filters/SGDBFilters.svelte";

</script>

{#if $showSGDBFiltersSheet}
  <SgdbFilters />
{/if}