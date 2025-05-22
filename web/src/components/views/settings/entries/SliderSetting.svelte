<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { Slider } from "@interactables";
  import { Card } from "@layout";
  import { isLandscape } from "@stores/State";
  
  type Props = {
    label: string;
    description: string;
    icon?: IconifyIcon | undefined;
    iconSize?: string;
    value: number;
    min?: number;
    max?: number;
    step?: number;
  }

  let {
    label,
    description,
    icon = undefined,
    iconSize = "1.5rem",
    value = $bindable(),
    min = 0,
    max = 100,
    step = 1,
  }: Props = $props();

  let slider: Slider | undefined = $state();
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
        <div class="slider-container">
          <div style="width: 90%;">
            <Slider min={min} max={max} step={step} bind:value={value} bind:this={slider} />
          </div>
          <div style="width: 10%;">{slider?.valueDisplayed?.current.toFixed(0)}</div>
        </div>
      </div>
    </div>
  </Card>
</div>

<style>
  .content {
    width: 100%; 
    display: flex;
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

  .slider-container {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 15px;
  }
</style>