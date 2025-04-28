<script lang="ts">
  import { selectedNewGrid } from "@stores/Modals";
  import type { SGDBImage } from "@types";
  import GridImage from "./GridImage.svelte";

  type Props = {
    grid: SGDBImage;
    gridType: string
  }

  let { grid, gridType }: Props = $props();

  let imagePath = grid.thumb.toString();

  const selected = $derived($selectedNewGrid?.id === grid.id);

  function handleClick() {
    if ($selectedNewGrid?.id === grid.id) {
      $selectedNewGrid = null;
    } else {
      $selectedNewGrid = grid;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="grid" class:selected onclick={handleClick}>
  <div class="overlay-container">
    {#if selected}
      <div class="overlay" class:selected>
        <!-- When selected, scale in background to cover entire portrait, and bounce in the icon -->
        <!-- When decelected, scale out background and icon -->
      </div>
      <!-- <div class="icon-container" in:bounce>
        
      </div> -->
    {/if}
  </div>

  <GridImage
    imagePath={imagePath}
    altText="{grid.author.name}'s Grid image"
    gridType={gridType}
    isVideo={grid.isAnimated}
  />
</div>

<style>
  .grid {
    position: relative;

    border: 1px solid rgb(var(--m3-scheme-surface-container-highest));
    border-radius: var(--m3-util-rounding-medium);
    overflow: hidden;

    scale: 1;

    transition: transform 0.2s, border 0.2s;
  }

  .grid:hover {
    transform: scale(1.075);
    border: 1px solid rgb(var(--m3-scheme-outline));
    cursor: pointer;
  }

  .grid.selected {
    border: 1px solid rgb(var(--m3-scheme-primary));
  }
  
  .overlay {
    display: none;

    position: absolute;
    top: 0;
    left: 0;

    width: 100%;
    height: 100%;
    border-radius: 4px;

    background-color: rgb(var(--m3-scheme-outline) / 0.7);

    justify-content: center;
    align-items: center;

    z-index: 1;
  }

  .overlay.selected {
    display: flex;
  }

  .selected :global(svg) {
    animation: icon-select 0.5s cubic-bezier(0.64, 0.57, 0.67, 1.53);
    color: rgb(var(--m3-scheme-on-secondary-container));
  }

  @keyframes icon-select {
    0% {
      scale: 1
    }
    25% {
      scale: 1.07
    }
    50% {
      scale: 0.93
    }
    75% {
      scale: 1.07
    }
    100% {
      scale: 1
    }
  }
</style>