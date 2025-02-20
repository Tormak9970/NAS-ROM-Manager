<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { Toggle } from "@interactables";
  import { Card } from "@layout";
  import { isLandscape } from "@stores/State";

  export let label: string;
  export let description: string;
  export let icon: IconifyIcon | undefined = undefined;
  export let iconSize: string = "1.5rem";
  export let checked: boolean;
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
      <label class="toggle-container">
        <Toggle bind:checked={checked} />
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

  .toggle-container {
    margin-left: auto;
    align-self: center;
  }
</style>