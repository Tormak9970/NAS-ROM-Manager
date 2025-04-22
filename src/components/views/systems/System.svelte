<script lang="ts">
  import { Icon } from "@component-utils";
  import { contextMenu } from "@directives";
  import { Landscape } from "@icons";
  import { getSystemMenuItems } from "@menus";
  import { libraryGridType, romsBySystem, systems } from "@stores/State";
  import { goToSystem, GRID_LAYOUTS } from "@utils";
  import SystemTag from "@views/SystemTag.svelte";
  import Tag from "@views/Tag.svelte";
  import { fade } from "svelte/transition";
  import Capsule from "../Capsule.svelte";

  type Props = {
    abbreviation: string;
  }

  let { abbreviation }: Props = $props();

  const system = $derived($systems[abbreviation]);
  const layout = $derived(GRID_LAYOUTS[$libraryGridType]);
  const menuItems = $derived(getSystemMenuItems(abbreviation));
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="system"
  style:width="{layout.width - 2}px"
  style:height="{layout.height - 2}px"
  onclick={() => goToSystem(abbreviation)}
  in:fade={{ duration: 200 }}
  use:contextMenu={{ items: menuItems }}
>
  {#if system.thumbCapsulePath === "No Grids"}
    <div class="placeholder">
      <Icon icon={Landscape} height="1.5rem" width="1.5rem" />
    </div>
  {/if}
  <Capsule thumbCapsulePath={system.thumbCapsulePath} />
  <div class="system-tag">
    <SystemTag system={abbreviation} />
  </div>
  <div class="rom-count">
    <Tag backgroundColor="var(--m3-scheme-tertiary-container)">{$romsBySystem[abbreviation].length}</Tag>
  </div>
</div>

<style>
  .system {
    position: relative;

    border: 1px solid rgb(var(--m3-scheme-surface-container-highest));
    border-radius: var(--m3-util-rounding-medium);
    overflow: hidden;

    scale: 1;

    transition: transform 0.2s, border 0.2s;
  }

  .system:hover {
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

  .system-tag {
    position: absolute;
    top: 0.3rem;
    left: 0.4rem;
  }
  
  .rom-count {
    position: absolute;
    top: 0.3rem;
    right: 0.4rem;
  }
</style>