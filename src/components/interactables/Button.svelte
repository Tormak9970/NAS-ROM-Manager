<script lang="ts">
  import { stopPropagation } from "@utils";
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";
  
  type Props = {
    extraOptions?: HTMLButtonAttributes;
    iconType?: "none" | "left" | "full";
    type: "elevated" | "filled" | "tonal" | "outlined" | "text";
    size?: string;
    iconSize?: string;
    disabled?: boolean;
    warning?: boolean;
    children?: Snippet;
    onclick?: () => void;
  }

  let {
    extraOptions = {},
    iconType = "none",
    type,
    size = "2.5rem",
    iconSize = "1.5rem",
    disabled = false,
    warning = false,
    children,
    onclick = () => {}
  }: Props = $props();

  // @ts-expect-error This will always be defined before its usage.
  let innerButton: HTMLButtonElement = $state();

  export function getButtonElement(): HTMLButtonElement {
    return innerButton;
  }
</script>

<button
  bind:this={innerButton}
  onclick={stopPropagation(onclick)}
  {disabled}
  class="m3-container m3-font-label-large font-label {type} icon-{iconType}"
  style:--size={size}
  style:--icon-size={iconSize}
  class:warning
  {...extraOptions}
>
  <div class="layer"></div>
  {@render children?.()}
</button>

<style>
  :root {
    --m3-button-shape: var(--m3-util-rounding-full);
  }

  .m3-container {
    border: none;
    height: var(--size);
    padding: 0 1.5rem;
    border-radius: var(--m3-button-shape);
    color: rgb(var(--text-color));
    transition: background-color 0.2s ease-out, color 0.2s ease-out, box-shadow 0.2s ease-out;

    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    position: relative;
    overflow: hidden;
  }
  .layer {
    position: absolute;
    inset: 0;
    transition: background-color 0.2s ease-out;
  }

  .m3-container > :global(*) {
    flex-shrink: 0;
  }
  .icon-left {
    padding-left: 1rem;
    gap: 0.5rem;
  }
  .icon-left > :global(svg) {
    width: 1.125rem;
    height: 1.125rem;
  }
  .icon-full {
    width: var(--size);
    padding: 0;
  }
  .icon-full :global(svg) {
    width: var(--icon-size);
    height: var(--icon-size);
  }

  .m3-container:disabled {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
    cursor: auto;
  }

  .m3-container:enabled.elevated {
    background-color: rgb(var(--m3-scheme-surface-container-low));
    --text-color: var(--m3-scheme-primary);
    box-shadow: var(--m3-util-elevation-1);
  }

  .m3-container:enabled.filled {
    background-color: rgb(var(--m3-scheme-primary));
    --text-color: var(--m3-scheme-on-primary);
  }

  .m3-container:enabled.tonal {
    background-color: rgb(var(--m3-scheme-secondary-container));
    --text-color: var(--m3-scheme-on-secondary-container);
  }
  
  .m3-container:enabled.tonal.warning {
    background-color: rgb(var(--m3-scheme-tertiary-container));
    --text-color: var(--m3-scheme-on-tertiary-container);
  }

  .m3-container.outlined {
    background-color: transparent;
    border: 0.0625rem solid rgb(var(--m3-scheme-on-surface) / 0.12);
  }
  .m3-container:enabled.outlined {
    background-color: rgb(var(--m3-scheme-surface-container));
    border: 0.0625rem solid rgb(var(--m3-scheme-outline) / 0.12);
    --text-color: var(--m3-scheme-on-background);
  }

  .m3-container.text {
    background-color: transparent;
    padding: 0 0.75rem;
    --text-color: var(--m3-scheme-primary);
  }
  .m3-container.text.icon-left {
    padding-right: 1rem;
  }

  .m3-container {
    -webkit-tap-highlight-color: transparent;
  }
  @media (hover: hover) {
    .m3-container:enabled:hover > .layer {
      background-color: rgb(var(--text-color) / 0.08);
    }
    .m3-container:enabled.elevated:hover {
      box-shadow: var(--m3-util-elevation-2);
    }
    .m3-container:enabled.filled:hover {
      box-shadow: var(--m3-util-elevation-1);
    }
    .m3-container:enabled.tonal:hover {
      box-shadow: var(--m3-util-elevation-1);
    }
  }
  .m3-container:enabled:focus-visible > .layer,
  .m3-container:enabled:active > .layer {
    background-color: rgb(var(--text-color) / 0.12);
  }
</style>