<script lang="ts">
  import { showDevelopersFilterSheet, showFormatsFilterSheet, showGenresFilterSheet, showPublishersFilterSheet, showReleaseDateFilterSheet, showSGDBFiltersSheet, showSystemsFilterSheet } from "@stores/Sheets";
  import DevelopersFilter from "./DevelopersFilter.svelte";
  import FormatsFilter from "./FormatsFilter.svelte";
  import GenresFilter from "./GenresFilter.svelte";
  import PublishersFilter from "./PublishersFilter.svelte";
  import ReleaseDateFilter from "./ReleaseDateFilter.svelte";
  import SgdbFilters from "./sgdb-filters/SGDBFilters.svelte";
  import SystemsFilter from "./SystemsFilter.svelte";

</script>

{#if $showSGDBFiltersSheet}
  <SgdbFilters />
{/if}

{#if $showSystemsFilterSheet}
  <SystemsFilter />
{/if}

{#if $showFormatsFilterSheet}
  <FormatsFilter />
{/if}

{#if $showGenresFilterSheet}
  <GenresFilter />
{/if}

{#if $showDevelopersFilterSheet}
  <DevelopersFilter />
{/if}

{#if $showPublishersFilterSheet}
  <PublishersFilter />
{/if}

{#if $showReleaseDateFilterSheet}
  <ReleaseDateFilter />
{/if}
