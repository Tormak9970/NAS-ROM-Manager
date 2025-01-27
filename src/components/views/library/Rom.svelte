<script lang="ts">
  import { Icon } from "@component-utils";
  import { Download } from "@icons";
  import { Button } from "@interactables";
  import { SYSTEM_TAGS_CONFIG } from "@models";
  import { libraryGridType } from "@stores/State";
  import type { ROM } from "@types";
  import { formatFileSize, GRID_LAYOUTS } from "@utils";
  import RomTag from "./RomTag.svelte";

  type RomProps = {
    rom: ROM;
  }

  let { rom }: RomProps = $props();

  let system = $derived(rom.system.toLowerCase());

  let cover = "https://cdn2.steamgriddb.com/thumb/3c64afe806cd466dd1ffecbe3e2e8cce.jpg";

  let layout = $derived(GRID_LAYOUTS[$libraryGridType]);

  const DEFAULT_COLOR = "var(--m3-scheme-tertiary-container)";
  let systemTagConfig = $derived(SYSTEM_TAGS_CONFIG[system]);

  function download() {

  }

  function openEditModal() {

  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="rom"
  style:width="{layout.width - 2}px"
  style:height="{layout.height - 2}px"
  style:--cover-url='url("{cover}")'
  onclick={openEditModal}
>
  <div class="cover"></div>
  <div class="overlay">
    <div class="rom-info">
      <div class="title">{rom.title}</div>
      <div class="date">{rom.addDate}</div>
      <div class="file-info">
        <div class="size">{formatFileSize(rom.size)}</div>
        <RomTag backgroundColor="var(--m3-scheme-tertiary-container)">.{rom.format}</RomTag>
      </div>
    </div>
    <div style:--m3-button-shape="var(--m3-util-rounding-small)">
      <Button
        type="filled"
        iconType="left"
        size="2.25rem"
        extraOptions={{
          style: "width: 100%;"
        }}
        on:click={download}
      >
        <Icon icon={Download} />
        Download
      </Button>
    </div>
  </div>
  <div class="system">
    <RomTag
      backgroundColor={systemTagConfig?.backgroundColor ?? DEFAULT_COLOR}
      borderColor={systemTagConfig?.borderColor ?? DEFAULT_COLOR}
      isUppercase
    >
      <b>{rom.system}</b>
    </RomTag>
  </div>
</div>

<style>
  .rom {
    position: relative;

    border: 1px solid rgb(var(--m3-scheme-surface-container-highest));
    border-radius: var(--m3-util-rounding-medium);
    overflow: hidden;

    scale: 1;

    transition: scale 0.2s, border 0.2s;
  }

  .rom:hover {
    scale: 1.05;
    border: 1px solid rgb(var(--m3-scheme-outline));
  }

  .cover {
    position: absolute;
    top: 0;
    left: 0;

    width: 100%;
    height: 100%;
    border-radius: var(--m3-util-rounding-medium);

    background-image: var(--cover-url);
    background-repeat: no-repeat;
    background-size: contain;
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