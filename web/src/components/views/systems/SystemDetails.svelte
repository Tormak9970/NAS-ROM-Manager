<script lang="ts">
  import { romMetadata } from "@stores/State";
  import type { System } from "@types";
  import RomsGrid from "@views/RomsGrid.svelte";

  type Props = {
    system: System | undefined;
    romIds: string[];
    portrait: boolean;
  }

  let { system, romIds, portrait }: Props = $props();
  
  const romIdList = $derived(romIds.sort((a: string, b: string) => {
    const metaA = $romMetadata[a];
    const metaB = $romMetadata[b];

    if (metaA.isFavorite && !metaB.isFavorite) return -1;
    if (metaB.isFavorite && !metaA.isFavorite) return 1;

    return a.localeCompare(b);
  }));
</script>

<div class="details" class:portrait>
  <h2>ROMs</h2>
  {#if romIds?.length === 0}
    <div class="summary body-text">
      No ROMs were found for <b>{system?.name}</b>
    </div>
  {:else}
    <RomsGrid gridName={`${system?.abbreviation}-roms`} romIds={romIdList} />
  {/if}
</div>
<style>
  .details {
    width: 100%;

    display: flex;
    flex-direction: column;
  }

  .portrait h2 {
    padding-left: 0.5rem;
  }
</style>