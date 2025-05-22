<script lang="ts">
  import { IGDB_WEBSITE_TYPES, type ROMMetadata } from "@types";
  import { formatDateNumber } from "@utils";
  import AgeRating from "@views/library/details/AgeRating.svelte";
  import RelatedGameTableRow from "@views/library/details/RelatedGameTableRow.svelte";
  import WebsiteLink from "@views/library/details/WebsiteLink.svelte";
  import TableRow from "./TableRow.svelte";

  type Props = {
    metadata: ROMMetadata;
    portrait: boolean;
  }

  let { metadata, portrait }: Props = $props();

  const ageRatings = $derived(metadata.metadata?.metadata?.ageRatings ?? []);
  const websites = $derived(metadata.metadata?.metadata?.websites ?? []);
</script>

<div class="details metadata" class:portrait>
  <div class="text-info">
    <div>
      <h2>Overview</h2>
      <div class="summary body-text">{metadata.metadata?.summary ?? "No overview was available"}</div>
    </div>
    
    <div class="companies" class:portrait>
      <div>
        <h3>Developer</h3>
        <div class="body-text">{metadata.metadata?.metadata?.developers?.join(',\n') || 'Unkown'}</div>
      </div>
      <div>
        <h3>Publisher</h3>
        <div class="body-text">{metadata.metadata?.metadata?.publishers?.join(',\n') || 'Unkown'}</div>
      </div>
    </div>

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
      <TableRow
        label="Release Date"
        value={metadata.metadata?.metadata?.firstReleaseDate ? formatDateNumber(metadata.metadata.metadata.firstReleaseDate) : 'Unkown'}
      />
      <RelatedGameTableRow
        label="DLCs"
        value={metadata.metadata?.metadata?.dlcs ?? []}
      />
      <RelatedGameTableRow
        label="Expansions"
        value={metadata.metadata?.metadata?.expansions ?? []}
      />
      <TableRow
        label="Franchise"
        value={metadata.metadata?.metadata?.franchises?.[0] ?? 'Unkown'}
      />
      <div class="links">
        <WebsiteLink type={IGDB_WEBSITE_TYPES.SGDB} url="https://www.steamgriddb.com/game/{metadata.sgdbId}" />
        <WebsiteLink type={IGDB_WEBSITE_TYPES.IGDB} url="https://www.igdb.com/games/{metadata.metadata?.slug}" />
        {#each websites as website}
          <WebsiteLink type={website.type} url={website.url} />
        {/each}
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

  .metadata.portrait {
    flex-direction: column;
    align-items: center;
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
  .companies.portrait {
    flex-direction: column;
    gap: 2rem;
  }

  .companies .body-text {
    white-space: pre;
  }

  .age-ratings {
    display: flex;
    flex-wrap: wrap;

    gap: 1rem;
  }

  .table-info {
    border-radius: var(--m3-util-rounding-small);

    border: 1px solid rgb(var(--m3-scheme-surface-container-highest));

    width: 20rem;
  }

  .links {
    width: calc(100% - 1rem);
    
    margin: 0.5rem;

    display: flex;
    flex-wrap: wrap;

    gap: 0.5rem;
  }
</style>