<script lang="ts">
  import type { ROMMetadata } from "@types";
  import { formatDateNumber } from "@utils";
  import TableRow from "./TableRow.svelte";

  type Props = {
    metadata: ROMMetadata
  }

  let { metadata }: Props = $props();
</script>

<div class="details metadata">
  <div class="text-info">
    <h2>Overview</h2>
    <div class="summary body-text">{metadata.metadata?.summary ?? "No overview was available"}</div>
    
    <h2>Story Summary</h2>
    <div class="summary body-text">{metadata.metadata?.summary ?? "No overview was available"}</div>
    
    <!-- Keywords -->
  </div>
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
    <TableRow
      label="Publisher"
      value={metadata.metadata?.metadata?.publishers?.join(',\n') || 'Unkown'}
    />
    <TableRow
      label="Developer"
      value={metadata.metadata?.metadata?.developers?.join(',\n') || 'Unkown'}
    />
    <!-- TODO: truncate after 4 entries -->
    <TableRow
      label="Supported Languages"
      value={metadata.metadata?.metadata?.languages?.join(',\n') || 'Unkown'}
    />
    <div class="links">
      <!-- TODO: SGDB -->
      <!-- TODO: IGDB -->
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

  }

  .table-info {
    border-radius: var(--m3-util-rounding-small);

    border: 2px solid rgb(var(--m3-scheme-outline));

    width: 20rem;
  }
</style>