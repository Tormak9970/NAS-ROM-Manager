<script lang="ts">
  import { Icon } from "@component-utils";
  import { RomController } from "@controllers";
  import { contextMenu } from "@directives";
  import { Download, FavoriteOff, FavoriteOn, Landscape } from "@icons";
  import { Button } from "@interactables";
  import { getRomMenuItems } from "@menus";
  import { libraryGridType, romMetadata, roms } from "@stores/State";
  import type { ROMMetadata } from "@types";
  import { formatFileSize, goToROM, GRID_LAYOUTS } from "@utils";
  import SystemTag from "@views/SystemTag.svelte";
  import Tag from "@views/Tag.svelte";
  import { fade } from "svelte/transition";
  import Cover from "../Cover.svelte";

  type RomProps = {
    romId: string;
  }

  let { romId }: RomProps = $props();

  const rom = $derived($roms[romId]);
  const metadata: ROMMetadata = $derived($romMetadata[romId]);

  const layout = $derived(GRID_LAYOUTS[$libraryGridType]);
  const isFavorite = $derived(metadata.isFavorite);

  const menuItems = $derived(getRomMenuItems(romId, metadata?.igdbId));
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="rom"
  style:width="{layout.width - 2}px"
  style:height="{layout.height - 2}px"
  onclick={() => goToROM(romId)}
  in:fade={{ duration: 200 }}
  use:contextMenu={{ items: menuItems }}
>
  {#if metadata.thumbPath === "No Grids"}
    <div class="placeholder">
      <Icon icon={Landscape} height="1.5rem" width="1.5rem" />
    </div>
  {/if}
  <Cover romId={romId} />
  <div class="overlay">
    <div class="rom-info">
      <div class="title">{rom.title}</div>
      <div class="file-info">
        <div style:font-size=".9rem">{formatFileSize(rom.size)}</div>
        <Tag backgroundColor="var(--m3-scheme-tertiary-container)">.{rom.format}</Tag>
      </div>
    </div>
    <div style:--m3-button-shape="var(--m3-util-rounding-small)">
      <Button
        type="filled"
        iconType="left"
        size="2rem"
        extraOptions={{
          style: "width: 100%;"
        }}
        on:click={() => RomController.download(romId)}
      >
        <Icon icon={Download} />
        Download
      </Button>
    </div>
  </div>
  <div class="system">
    <SystemTag system={rom.system} />
  </div>
  <div class="favorite" class:visible={isFavorite}>
    <Button iconType="full" type="text" size="2rem" iconSize="1.25rem" on:click={() => RomController.toggleFavorite(romId)}>
      <Icon icon={isFavorite ? FavoriteOn : FavoriteOff} />
    </Button>
  </div>
</div>

<style>
  .rom {
    position: relative;

    border: 1px solid rgb(var(--m3-scheme-surface-container-highest));
    border-radius: var(--m3-util-rounding-medium);
    overflow: hidden;

    scale: 1;

    transition: transform 0.2s, border 0.2s;
  }

  .rom:hover {
    transform: scale(1.04);
    border: 1px solid rgb(var(--m3-scheme-outline));
    cursor: pointer;
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

  .overlay {
    position: absolute;
    top: 0;
    left: 0;

    width: calc(100% - 0.8rem);
    height: calc(100% - 0.8rem);
    padding: 0.4rem;

    opacity: 0;

    background: linear-gradient(rgb(var(--m3-scheme-surface-container-highest) / 0.4) 0%, rgb(var(--m3-scheme-surface-container-highest) / 0.9) 100%);
    
    transition: opacity 0.15s;

    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 0.25rem;
  }

  .rom:hover .overlay {
    opacity: 1;
  }

  .system {
    position: absolute;
    top: 0.3rem;
    left: 0.4rem;
  }
  
  .favorite {
    position: absolute;
    top: 0.1rem;
    right: 0.1rem;

    opacity: 0;
    transition: opacity 0.15s;
  }

  .favorite.visible {
    --m3-scheme-primary: 255 49 49;

    opacity: 1;
  }

  .rom:hover .favorite {
    opacity: 1;
  }

  .rom-info {
    color: white;
  }

  .file-info {
    width: 100%;

    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .title {
    font-size: 1.4rem;
    font-weight: bold;

    display: -webkit-box;
    -webkit-box-orient: vertical;

    line-clamp: 3;
    -webkit-line-clamp: 3;
    -moz-line-clamp: 3;

    text-overflow: ellipsis;
    overflow: hidden;
  }
</style>