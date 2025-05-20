<script lang="ts">
  import { stopPropagation } from "@utils";
  import type { Snippet } from "svelte";
  import type { HTMLAttributes, HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    extraOptions?: HTMLAttributes<HTMLDivElement> & HTMLButtonAttributes;
    type: "elevated" | "filled" | "outlined" | "transparent";
    highlight?: boolean;
    children?: Snippet;
    onclick?: () => void;
  }

  let {
    extraOptions = {},
    type,
    highlight = false,
    children,
    onclick = () => {},
  }: Props = $props();
</script>

<button
  onclick={stopPropagation(onclick)}
  class="m3-container type-{type}"
  {...extraOptions}
>
  <div class="layer" class:highlight></div>
  {@render children?.()}
</button>

<style>
  :root {
    --m3-card-shape: var(--m3-util-rounding-medium);
  }
  .m3-container {
    display: flex;
    flex-direction: column;
    position: relative;
    padding: 1rem; /* protip: use margin: -1rem (adjust as needed) to make images stretch to the end */
    border: none;
    border-radius: var(--m3-card-shape);
    background-color: rgb(var(--m3-scheme-surface));
    color: rgb(var(--m3-scheme-on-surface));
    transition: background-color 0.2s, box-shadow 0.2s;
  }
  .layer {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    transition: background-color 0.2s;
    pointer-events: none;
  }
  
  .type-transparent {
    background-color: transparent;
  }

  .type-elevated {
    background-color: rgb(var(--m3-scheme-surface-container-low));
  }
  .type-filled {
    background-color: rgb(var(--m3-scheme-surface-container-highest));
  }
  .type-outlined {
    background-color: rgb(var(--m3-scheme-surface-container));
    border: 0.0625rem solid rgb(var(--m3-scheme-outline) / 0.12);
    --text-color: var(--m3-scheme-on-background);
  }

  .type-elevated {
    box-shadow: var(--m3-util-elevation-1);
  }

  button {
    text-align: inherit;
    font: inherit;
    letter-spacing: inherit;
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
  }
  @media (hover: hover) {
    button:hover {
      box-shadow: var(--m3-util-elevation-1);
    }
    button.type-elevated:hover {
      box-shadow: var(--m3-util-elevation-2);
    }
    button:hover > .layer {
      background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
    }
  }
  button:is(:focus-visible, :active) > .layer {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }
  
  .highlight.layer,
  button:hover > .highlight.layer {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }

  @media print, (forced-colors: active) {
    .layer {
      display: none;
    }
    .type-filled {
      outline: solid 0.125rem;
    }
  }
  @media (forced-colors: active) {
    .type-elevated {
      outline: solid 0.125rem;
    }
  }
</style>