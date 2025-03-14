<script lang="ts">
  import type { ROMMetadata } from "@types";
  import { formatDateNumber } from "@utils";
  import AgeRating from "@views/library/details/AgeRating.svelte";
  import TableRow from "./TableRow.svelte";

  type Props = {
    metadata: ROMMetadata
  }

  let { metadata }: Props = $props();

  let ageRatings = $derived(metadata.metadata?.metadata?.ageRatings ?? []);
</script>

<div class="details metadata">
  <div class="text-info">
    <div>
      <h2>Overview</h2>
      <div class="summary body-text">{metadata.metadata?.summary ?? "No overview was available"}</div>
    </div>
    
    <div class="companies">
      <div>
        <h3>Developer</h3>
        <div class="body-text">{metadata.metadata?.metadata?.developers?.join(',\n') || 'Unkown'}</div>
      </div>
      <div>
        <h3>Publisher</h3>
        <div class="body-text">{metadata.metadata?.metadata?.publishers?.join(',\n') || 'Unkown'}</div>
      </div>
    </div>
    
    <!-- Keywords -->

    <div>
      <h3>Supported Languages</h3>
      <div class="body-text">{metadata.metadata?.metadata?.languages?.join(', ') || 'Unkown'}</div>
    </div>

    <div>
      <h3>Ratings</h3>
      <div class="age-ratings">
        {#each ageRatings as ageRatings}
          <AgeRating rating={ageRatings} />
        {/each}
      </div>
    </div>
  </div>
  <div class="right">
    <div class="table-info">
      <div class="ratings">
  
      </div>
      <TableRow
        label="Release Date"
        value={metadata.metadata?.metadata?.firstReleaseDate ? formatDateNumber(metadata.metadata?.metadata?.firstReleaseDate) : 'Unkown'}
      />
      <!-- TODO: truncate after 4 entries -->
      <TableRow
        label="DLCs"
        value={metadata.metadata?.metadata?.dlcs?.join(',\n') || 'None'}
      />
      <!-- TODO: truncate after 4 entries -->
      <TableRow
        label="Expansions"
        value={metadata.metadata?.metadata?.expansions?.join(',\n') || 'None'}
      />
      <TableRow
        label="Franchise"
        value={metadata.metadata?.metadata?.franchises?.[0] ?? 'Unkown'}
      />
      <div class="links">
        TODO: Links
        <!-- TODO: SGDB -->
        <!-- TODO: IGDB -->
      </div>
    </div>
  </div>
</div>
<div class="similar-games">

</div>

<style>
  .metadata {
    width: 100%;

    display: flex;

    gap: 3rem;
  }

  .text-info {
    flex-grow: 1;

    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .companies {
    width: 100%;

    display: flex;
    gap: 4rem;
  }

  .table-info {
    border-radius: var(--m3-util-rounding-small);

    border: 2px solid rgb(var(--m3-scheme-outline));

    width: 20rem;
  }

  .age-ratings {
    display: flex;
    flex-wrap: wrap;

    gap: 1rem;
  }
</style>