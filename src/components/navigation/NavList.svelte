<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  type Props = {
    extraOptions?: HTMLAttributes<HTMLElement>;
    type: "rail" | "bar";
    children: Snippet;
  }

  let { extraOptions = {}, type, children }: Props = $props();
</script>

<nav class="m3-container type-{type}" {...extraOptions}>
  {@render children()}
</nav>

<style>
  .m3-container {
    display: flex;
    position: relative;
    flex-grow: 1;
    background-color: rgb(var(--m3-scheme-surface-container));
    padding: 0.75rem 0.5rem 1rem 0.5rem;
    justify-content: space-evenly;
    gap: 0.5rem;
  }

  .type-rail {
    background-color: transparent;
    flex-direction: column;
    padding: 0;
    justify-content: flex-start;
    gap: 0.5rem;
  }

  .type-bar {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-around;
  }
</style>