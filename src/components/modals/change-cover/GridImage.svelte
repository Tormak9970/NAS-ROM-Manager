<script lang="ts">
  import { Icon } from "@component-utils";
  import { Landscape } from "@icons";
  import { GRID_LAYOUTS, IMAGE_FADE_OPTIONS } from "@utils";
  import Lazy from "svelte-lazy";

  type Props = {
    imagePath: string;
    altText: string;
    isVideo: boolean;
  }

  let { imagePath, altText, isVideo = false}: Props = $props();
  
  let layout = $derived(GRID_LAYOUTS.sgdbGrid);
  let widthWithBorder = $derived(layout.width - 2);
  let heightWithBorder = $derived(layout.height - 2);

  let showWarning = $state(false);

  /**
   * Function to run when the user starts hovering over a video.
   * @param e The associated MouseEvent.
   */
  function onEnter(e: Event): void {
    (e.target as HTMLVideoElement).play();
  }

  /**
   * Function to run when the user stops hovering over a video.
   * @param e The associated MouseEvent.
   */
  function onLeave(e: Event): void {
    (e.target as HTMLVideoElement).pause();
  }
</script>

<div
  class="grid-image"
  style:width="{widthWithBorder}px"
  style:height="{heightWithBorder}px"
>
  {#if !showWarning && imagePath}
    <Lazy height="{heightWithBorder}px" fadeOption={IMAGE_FADE_OPTIONS}>
      {#if isVideo}
        <video
          src="{imagePath}"
          muted
          loop
          autoplay={false}
          style="max-width: {widthWithBorder}px; max-height: {heightWithBorder}px; width: auto; height: auto;"
          onmouseover={onEnter}
          onfocus={onEnter}
          onmouseleave={onLeave}
          onblur={onLeave}
        ></video>
      {:else}
        <img
          src="{imagePath}"
          alt="{altText}"
          style="max-width: {widthWithBorder}px; max-height: {heightWithBorder}px; width: auto; height: auto;"
          draggable="false"
          onerror={() => showWarning = true}
        />
      {/if}
    </Lazy>
  {:else}
    <div class="placeholder">
      <Icon icon={Landscape} height="1.5rem" width="1.5rem" />
    </div>
  {/if}
</div>

<style>
  .grid-image {
    position: relative;
  }
  
  .placeholder {
    position: absolute;
    top: 0;
    left: 0;

    width: 100%;
    height: 100%;
    border-radius: var(--m3-util-rounding-medium);

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>