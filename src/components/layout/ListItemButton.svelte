<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";

  export let extraOptions: HTMLButtonAttributes = {};
  export let headline = "";
</script>

<button on:click class="m3-container" {...extraOptions}>
  {#if $$slots.leading}
    <div class="leading">
      <slot name="leading" />
    </div>
  {/if}
  <div class="body">
    <p class="headline m3-font-body-large">{headline}</p>
  </div>
</button>

<style>
  .m3-container {
    display: flex;
    padding: 0.5rem 1.5rem 0.5rem 1rem;
    align-items: center;
    gap: 1rem;
    text-align: inherit;
    background-color: transparent;
    border: none;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: background-color 200ms;
    height: 3.5rem;
  }
  .leading {
    display: contents;
    color: rgb(var(--m3-scheme-on-surface-variant));
  }
  .leading > :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
    flex-shrink: 0;
  }

  p {
    margin: 0;
  }
  .headline {
    color: rgb(var(--m3-scheme-on-surface));
  }

  @media (hover: hover) {
    .m3-container:hover {
      background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
    }
  }
  .m3-container:focus-visible,
  .m3-container:active {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }
</style>