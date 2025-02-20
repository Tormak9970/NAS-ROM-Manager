<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { Card } from "@layout";

  export let label: string;
  export let description: string;
  export let icon: IconifyIcon | undefined = undefined;
  export let iconSize: string = "1.5rem";
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
    </div>
  </div>
</Card>

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
</style>