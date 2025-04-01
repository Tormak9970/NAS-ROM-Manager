<script lang="ts">
  import { selectedNewCoverGrid } from "@stores/Modals";
  import type { SGDBImage } from "@types";
  import GridImage from "./GridImage.svelte";

  type Props = {
    grid: SGDBImage;
  }

  let { grid }: Props = $props();

  let imagePath = grid.thumb.toString();

  const selected = $derived($selectedNewCoverGrid?.id === grid.id);

  function handleClick() {
    if ($selectedNewCoverGrid?.id === grid.id) {
      $selectedNewCoverGrid = null;
    } else {
      $selectedNewCoverGrid = grid;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="grid" class:selected onclick={handleClick}>
  <div class="loading-overlay" class:selected>
  </div>

  <GridImage
    imagePath={imagePath}
    altText="{grid.author.name}'s Grid image"
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
  
  .loading-overlay {
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

  .loading-overlay.selected {
    display: flex;
  }
</style>