<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { ColorPicker } from "@interactables";
  import { Card } from "@layout";
  import { isLandscape } from "@stores/State";

  
  type Props = {
    label: string;
    description: string;
    icon?: IconifyIcon | undefined;
    iconSize?: string;
    color: string;
  }

  let {
    label,
    description,
    icon = undefined,
    iconSize = "1.5rem",
    color = $bindable(),
  }: Props = $props();
</script>

<div class="settings-entry">
  <Card type="transparent" extraOptions={{ style: "width: 100%;" }}>
    <div class="content">
      {#if $isLandscape}
        <div class="icon-container">
          {#if icon}
            <Icon icon={icon} height={iconSize} width={iconSize} />
          {/if}
        </div>
      {/if}
      <div class="info">
        <div class="font-label">{label}</div>
        <div class="font-body description">
          {description}
        </div>
      </div>
      <label class="color-container">
        <ColorPicker bind:hex={color} />
      </label>
    </div>
  </Card>
</div>

<style>
  .content {
    width: 100%; 
    display: flex;
    align-items: center;
  }

  .icon-container {
    height: 50px;
    width: 50px;
    border-radius: 50%;

    display: flex;
    align-items: center;
    justify-content: center;

    margin-right: 10px;
  }

  .icon-container :global(svg) {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .info {
    max-width: 50rem;
  }
  
  .description {
    color: rgb(var(--m3-scheme-outline));
  }

  .color-container {
    margin-left: auto;
  }
</style>