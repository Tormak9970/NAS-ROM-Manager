<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    extraOptions?: HTMLButtonAttributes;
    type: "rail" | "bar";
    selected: boolean;
    icon: IconifyIcon;
    children?: Snippet;
    onclick?: (e: any) => void;
  }

  let { extraOptions = {}, type, selected, icon, onclick, children }: Props = $props();
</script>

<button
  {onclick}
  class="m3-container type-{type}"
  class:selected
  {...extraOptions}
>
  <div class="icon-space">
    <Icon {icon} />
  </div>
  {#if type === "rail"}
    <p class="m3-font-label-medium font-body-medium">
      {@render children?.()}
    </p>
  {/if}
</button>

<style>
  .m3-container {
    display: flex;
    flex-direction: column;
    
    gap: 0.25rem;
    padding: 0;
    border: none;
    text-align: center;

    background-color: transparent;
    --text-color: var(--m3-scheme-on-surface-variant);
    color: rgb(var(--text-color));
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    
    position: relative;
  }
  
  .type-bar {
    height: 2rem;
  }

  .type-rail {
    flex-direction: row;
    align-items: center;
    padding: 0.25rem;
  }

  .m3-container::before {
    position: absolute;
    display: block;
    content: " ";
    background-color: rgb(var(--m3-scheme-secondary-container));
    z-index: 1;

    opacity: 0;
    inset: 0 50%;
    width: 0;
    border-radius: 2rem;
    transition:
      opacity 200ms,
      background-color 200ms,
      inset 0ms 200ms,
      width 0ms 200ms;
  }

  .selected {
    --text-color: var(--m3-scheme-on-surface);
  }
  
  .m3-container:hover.m3-container::before,
  .selected.m3-container::before {
    opacity: 1;
    inset: 0 0;
    width: 100%;
    transition:
      width 400ms cubic-bezier(0.356, 0.701, 0, 1.004),
      inset 400ms cubic-bezier(0.356, 0.701, 0, 1.004);
  }
  
  
  .m3-container:hover.m3-container::before {
    background-color: rgb(var(--text-color) / 0.08);
  }
  .selected:hover.m3-container::before {
    background-color: rgb(var(--m3-scheme-secondary-container) / 0.9);
  }

  .m3-container:focus-visible.m3-container::before,
  .m3-container:active.m3-container::before {
    background-color: rgb(var(--text-color) / 0.12);
  }
  
  .selected:focus-visible.m3-container::before,
  .selected:active.m3-container::before {
    background-color: rgb(var(--m3-scheme-secondary-container) / 0.9);
  }
  
  p {
    position: relative;
    z-index: 2;
    margin: 0;
    transition: color 200ms;
  }
  
  .icon-space {
    display: flex;
    flex: none;
    align-self: center;
    justify-content: center;
    align-items: center;

    width: 4rem;
    height: 2rem;
  }
  .type-rail > .icon-space {
    width: 3.5rem;
  }
  .icon-space > :global(svg) {
    position: relative;
    z-index: 2;
    width: 1.5rem;
    height: 1.5rem;
    transition: 
      color 200ms;
  }
  .selected > .icon-space > :global(svg) {
    color: rgb(var(--m3-scheme-on-secondary-container));
  }

  @keyframes icon-select {
    0% {
      scale: 1
    }
    25% {
      scale: 1.07
    }
    50% {
      scale: 0.93
    }
    75% {
      scale: 1.07
    }
    100% {
      scale: 1
    }
  }
</style>