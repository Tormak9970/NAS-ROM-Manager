<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { ColorPicker } from "@interactables";
  import { Card } from "@layout";
  import { isLandscape } from "@stores/State";

  export let label: string;
  export let description: string;
  export let icon: IconifyIcon | undefined = undefined;
  export let iconSize: string = "1.5rem";
  export let color: string;
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
      <!-- svelte-ignore a11y-label-has-associated-control -->
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