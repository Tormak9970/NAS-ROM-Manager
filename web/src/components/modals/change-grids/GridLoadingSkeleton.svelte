<script lang="ts">
  import { GRID_LAYOUTS } from "@utils";

  type Props = {
    gridType?: string;
  }

  let {
    gridType = "portrait"
  }: Props = $props();
  
  const layout = $derived(GRID_LAYOUTS[gridType]);
</script>

<div
  class="grid-skeleton"
  style:width="{layout.width - 2}px"
  style:height="{layout.height - 2}px"
>
  <div class="shine"></div>
</div>

<style>
  .grid-skeleton {
    position: relative;

    border: 1px solid rgb(var(--m3-scheme-surface-container-highest));
    border-radius: var(--m3-util-rounding-medium);
    overflow: hidden;

    scale: 1;

    transition: transform 0.2s, border 0.2s;
  }

  .shine {
    position: absolute;

    height: 100%;
    width: 100%;

    left: -100%;

    background-image: linear-gradient(
      to left,
      rgb(var(--m3-scheme-surface-container-high) / 0.05),
      rgb(var(--m3-scheme-surface-container-high) / .2),
      rgb(var(--m3-scheme-surface-container-high) / .5),
      rgb(var(--m3-scheme-surface-container-high) / .2),
      rgb(var(--m3-scheme-surface-container-high) / 0.05)
    );
    animation: loading 1.5s infinite;

    z-index: 45;
  }

  @keyframes loading {
    0%{
      left: -100%;
    }
    100%{
      left: 100%;
    }
  }
</style>