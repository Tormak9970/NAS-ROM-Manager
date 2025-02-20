<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { Slider } from "@interactables";
  import { Card } from "@layout";
  import type { Spring } from "svelte/motion";

  export let label: string;
  export let description: string;
  export let icon: IconifyIcon | undefined = undefined;
  export let iconSize: string = "1.5rem";
  export let value: number;
  export let min = 0;
  export let max = 100;
  export let step = 1;

  let valueDisplayed: Spring<number>;
</script>

<Card type="transparent" extraOptions={{ style: "width: 100%;" }}>
  <div class="content">
    <div class="icon-container">
      {#if icon}
        <Icon icon={icon} height={iconSize} width={iconSize} />
      {/if}
    </div>
    <div class="info">
      <div class="font-label">{label}</div>
      <div class="font-body description">
        {description}
      </div>
      <div class="slider-container">
        <div style="width: 90%;">
          <Slider min={min} max={max} step={step} bind:value={value} bind:valueDisplayed={valueDisplayed} />
        </div>
        <div style="width: 10%;">{$valueDisplayed?.toFixed(0)}</div>
      </div>
    </div>
  </div>
</Card>

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