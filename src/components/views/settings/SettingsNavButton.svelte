<script lang="ts">
  import { goto } from "$app/navigation";
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { CardClickable } from "@layout";

  export let icon: IconifyIcon;
  export let label: string;
  export let route: string;
  export let description: string;
</script>

<div class="settings-entry">
  <CardClickable type="elevated" on:click={() => goto(route)} extraOptions={{ style: "width: 100%; display: flex; position: relative; padding: 10px;" }}>
    <div class="content">
      <div class="icon-container">
        <Icon icon={icon} height="1.5rem" width="1.5rem" />
      </div>
      <div class="info">
        <div class="label">{label}</div>
        <div class="font-body description">
          {description}
        </div>
      </div>
    </div>
  </CardClickable>
</div>

<style>
  .content {
    width: 100%; 
    display: flex;
    align-items: center;
  }
  
  .settings-entry :global(.type-elevated) {
    background-color: rgb(var(--m3-scheme-surface-container));
    box-shadow: none;
  }

  .icon-container {
    height: 3.5rem;
    width: 3.5rem;
    border-radius: var(--m3-util-rounding-medium);

    display: flex;
    align-items: center;
    justify-content: center;

    color: rgb(var(--m3-scheme-primary));
    background-color: rgb(var(--m3-scheme-background));

    margin-right: 15px;
  }

  .icon-container :global(svg) {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .info {
    width: calc(100% - 75px);
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .label {
    font-size: 1.15rem;
    font-weight: bold;
    color: rgb(var(--m3-scheme-primary));
  }
</style>