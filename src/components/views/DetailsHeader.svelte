<script lang="ts">
  import { Icon } from "@component-utils";
  import { BackArrow } from "@icons";
  import { Button } from "@interactables";
  import { GRID_LAYOUTS } from "@utils";
  import Capsule from "@views/Capsule.svelte";
  import SystemTag from "@views/SystemTag.svelte";
  import type { Snippet } from "svelte";

  type Props = {
    portrait: boolean;
    title: string;
    system: string;
    capsuleSrc: string;
    controls: Snippet;
    headerMetadata: Snippet;
  }

  let {
    portrait,
    title,
    system,
    capsuleSrc,
    controls,
    headerMetadata
  }: Props = $props();
</script>

<div class="header" class:portrait>
  {#if portrait}
    <div class="back-button">
      <Button iconType="full" type="text" size="2.75rem" iconSize="1.75rem" onclick={() => window.history.back()}>
        <Icon icon={BackArrow} />
      </Button>
    </div>
  {/if}
  <div class="capsule" style="height: {GRID_LAYOUTS.portrait.height * 1.2}px;">
    <Capsule src={capsuleSrc} />
  </div>
  <div class="info" class:portrait>
    <SystemTag system={system} />
    <div class="title m3-font-headline-{portrait ? "small" : "medium"}">
      {title}
    </div>
    <div class="header-metadata" class:portrait>
      {@render headerMetadata()}
    </div>
  </div>
  <div class="controls" class:portrait style:--m3-button-shape="var(--m3-util-rounding-small)">
    {@render controls()}
  </div>
</div>

<style>
  .header {
    width: 100%;

    display: flex;
    align-items: flex-end;

    position: relative;
    z-index: 2;

    gap: 1rem;

    pointer-events: none;
  }

  .header > * {
    pointer-events: all;
  }

  .header.portrait {
    flex-direction: column;
    align-items: center;
  }

  .back-button {
    position: absolute;

    left: 1rem;
    top: 0.75rem;
  }

  .portrait .title {
    text-align: center;
  }

  .capsule {
    position: relative;

    aspect-ratio: 2 / 3;

    margin-top: 0.5rem;
  }

  .info {
    display: flex;
    flex-direction: column;

    align-items: flex-start;
  }

  .info.portrait {
    align-items: center;
  }

  .header-metadata {
    font-size: 0.9rem;

    color: rgb(var(--m3-scheme-on-surface-variant));

    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  .header-metadata.portrait {
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }
  .header-metadata > :global(.first-row) {
    display: flex;
    gap: 0.5rem;
  }

  .controls {
    width: fit-content;

    display: flex;

    gap: 0.5rem;

    margin-left: auto;
  }
  .controls.portrait {
    margin: 0;
  }
</style>