<script lang="ts">
  import { Icon } from "@component-utils";
  import { Checkmark } from "@icons";
  import { selectedNewGrid } from "@stores/Modals";
  import type { SGDBImage } from "@types";
  import { scale } from "svelte/transition";
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
      <div class="overlay" class:selected transition:scale={{ start: 0, duration: 200 }}>
        <Icon icon={Checkmark} width="2.5rem" height="2.5rem" />
      </div>
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
    cursor: pointer;
  }

  .grid::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: .125rem solid rgb(var(--m3-scheme-primary));
    transition: all .2s;
    animation: clippath 3s linear infinite;
    border-radius: var(--m3-util-rounding-medium);
    opacity: 0;
    z-index: 2;
  }
  .grid.selected::after {
    opacity: 1;
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
    color: rgb(var(--m3-scheme-primary));
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

  @keyframes clippath {
    0%, 100% {
      -webkit-clip-path: inset(0 0 98% 0);
      clip-path: inset(0 0 98% 0);
    }
    25% {
      -webkit-clip-path: inset(0 98% 0 0);
      clip-path: inset(0 98% 0 0);
    }
    50% {
      -webkit-clip-path: inset(98% 0 0 0);
      clip-path: inset(98% 0 0 0);
    }
    75% {
      -webkit-clip-path: inset(0 0 0 98%);
      clip-path: inset(0 0 0 98%);
    }
  }
</style>