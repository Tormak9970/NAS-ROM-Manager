<script lang="ts">
  import type { HTMLAttributes, HTMLButtonAttributes } from "svelte/elements";

  export let extraOptions: HTMLAttributes<HTMLDivElement> & HTMLButtonAttributes = {};
  export let type: "elevated" | "filled" | "outlined" | "transparent";
</script>

<div class="card-wrapper type-{type}" {...extraOptions}>
  <slot name="header" />
  <div class="m3-container">
    <slot />
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
    padding: 1rem; /* protip: use margin: -1rem (adjust as needed) to make images stretch to the end */

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
    border: solid 0.0625rem rgb(var(--m3-scheme-outline));
  }

  .type-elevated {
    box-shadow:
      0px 3px 1px -2px rgb(var(--m3-scheme-shadow) / 0.2),
      0px 2px 2px 0px rgb(var(--m3-scheme-shadow) / 0.14),
      0px 1px 5px 0px rgb(var(--m3-scheme-shadow) / 0.12);
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