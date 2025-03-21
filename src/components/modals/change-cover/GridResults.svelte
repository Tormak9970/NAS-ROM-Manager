<script lang="ts">
  import { SGDBController } from "@controllers";
  import { scrollShadow } from "@directives";
  import { InfiniteScroll } from "@layout";
  import { dbFilters, hasMorePagesCache } from "@stores/State";
  import type { SGDBImage } from "@types";
  import { debounce, filterGrids, GRID_LAYOUTS } from "@utils";
  import { onMount } from "svelte";
  import Grid from "./Grid.svelte";
  import GridLoadingSkeleton from "./GridLoadingSkeleton.svelte";

  type Props = {
    sgdbId: string;
  }

  let { sgdbId }: Props = $props();

  let layout = $derived(GRID_LAYOUTS.sgdbGrid);
  
  let gridsContainer: HTMLDivElement | undefined = $state(undefined);

  let isLoading = $state(true);
  let grids: SGDBImage[] = $state([]);

  let hasMore = $derived(($hasMorePagesCache && $hasMorePagesCache[sgdbId]) ?? true);

  /**
   * Handles loading new grids when the user scrolls to the bottom.
   */
  async function handleLoadOnScroll() {
    const unfilteredGrids = await SGDBController.getCoversForGame(sgdbId);
    grids = filterGrids(unfilteredGrids, $dbFilters);
  }

  async function handleResize(isOverflowing: boolean) {
    if (gridsContainer && !isOverflowing) {
      handleLoadOnScroll();
    }
  }
  const debouncedResize = debounce(handleResize, 500);
  
  onMount(() => {
    handleLoadOnScroll().then(() => {
      isLoading = false;
    });
  });
</script>

<svelte:document on:resize={debouncedResize} />

<div class="scroll-container" use:scrollShadow>
  <div class="grid-results">
    {#if isLoading}
      <div
        class="game-grid"
        style="--img-width: {layout.width}px; --img-height: {layout.height}px; --gap: {layout.gap}px;">
        {#each new Array(100) as _}
          <GridLoadingSkeleton />
        {/each}
      </div>
    {:else}
      {#if grids.length > 0}
        <div bind:this={gridsContainer} class="game-grid" style="--img-width: {layout.width}px; --img-height: {layout.height}px; --gap: {layout.gap}px;">
          {#each grids as grid (grid.id)}
            <Grid grid={grid} />
          {/each}
        </div>
      {:else}
        <div class="message">
          No results were found with your filters.
        </div>
      {/if}
    {/if}
  </div>
  <InfiniteScroll
    hasMore={hasMore}
    threshold={100}
    on:loadMore={handleLoadOnScroll}
  />
</div>

<style>
  .game-grid {
    width: 100%;

    padding: 0.5rem 0;

    display: grid;
    
    grid-template-columns: repeat(auto-fit, var(--img-width));
    row-gap: var(--gap);
    column-gap: var(--gap);
    grid-auto-flow: row;
    grid-auto-rows: var(--img-height);

    justify-content: center;
  }
  
  .scroll-container {
    height: 100%;
    width: 100%;

    overflow-y: scroll;
    overflow-x: hidden;
  }

  .message {
    width: 100%;
    text-align: center;
    opacity: 0.5;
    padding-top: 40px;
  }
</style>