<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  
  type Props = {
    extraOptions?: HTMLAttributes<HTMLDivElement>;
    children?: Snippet;
  }

  let { extraOptions = {}, children }: Props = $props();
</script>

<div class="m3-container" {...extraOptions}>
  {@render children?.()}
</div>

<style>
  :root {
    --m3-segmented-button-shape: var(--m3-util-rounding-full);
  }

  .m3-container {
    border: 0.0625rem solid rgb(var(--m3-scheme-outline));
    height: 2rem;
    border-radius: var(--m3-segmented-button-shape);
    overflow: hidden;

    display: inline-flex;
  }
</style>