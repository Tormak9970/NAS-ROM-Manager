<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes, HTMLButtonAttributes } from "svelte/elements";

  type Props = {
    extraOptions?: HTMLAttributes<HTMLDivElement> & HTMLButtonAttributes;
    type: "elevated" | "filled" | "outlined" | "transparent";
    padding?: string;
    header?: Snippet;
    children?: Snippet;
  }

  let {
    extraOptions = {},
    type,
    padding = "1rem",
    header,
    children,
  }: Props = $props();
</script>

<div class="card-wrapper type-{type}" {...extraOptions}>
  {@render header?.()}
  <!-- ! protip: use margin: -1rem (adjust as needed) to make images stretch to the end -->
  <div class="m3-container" style:padding={padding}>
    {@render children?.()}
  </div>
</div>

<style>
  :root {
    --m3-card-shape: var(--m3-util-rounding-medium);
  }

  .card-wrapper {
    display: flex;
    flex-direction: column;
    
    position: relative;

    border: none;
    border-radius: var(--m3-card-shape);

    background-color: rgb(var(--m3-scheme-surface));
    color: rgb(var(--m3-scheme-on-surface));

    overflow: hidden;
  }

  .m3-container {
    display: flex;
    flex-direction: column;

    position: relative;

    transition: all 200ms;
  }
  .layer {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    transition: all 200ms;
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