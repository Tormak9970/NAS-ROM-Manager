<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { Select } from "@interactables";
  import { Card } from "@layout";
  import { isLandscape } from "@stores/State";

  type Props = {
    label: string;
    description: string;
    icon?: IconifyIcon | undefined;
    iconSize?: string;
    options: SelectItem[];
    value: string;
  }

  let {
    label,
    description,
    icon = undefined,
    iconSize = "1.5rem",
    options,
    value = $bindable(),
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
      <div class="action-container">
        <Select
          name={label}
          options={options}
          bind:value={value}
          hideLabel
        />
      </div>
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

  .action-container {
    margin-left: auto;
    width: 10rem;
  }
</style>