<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  type Props = {
    extraOptions?: HTMLAttributes<HTMLDivElement>;
    percent: number;
  }

  let { extraOptions = {}, percent }: Props = $props();
</script>

<div class="m3-container" role="progressbar" {...extraOptions}>
  <div class="percent" style="width: {percent}%;"></div>
</div>

<style>
  :root {
    --m3-linear-progress-shape: var(--m3-util-rounding-full);
  }
  .m3-container {
    display: inline-flex;
    background-color: rgb(var(--m3-scheme-primary-container));
    height: 0.5rem;
    min-width: 8rem;
    width: 100%;
    overflow: hidden;
    border-radius: var(--m3-linear-progress-shape);
  }
  .percent {
    background-color: rgb(var(--m3-scheme-primary));
    transition: all 200ms;
    border-radius: var(--m3-linear-progress-shape);
  }

  .m3-container {
    print-color-adjust: exact;
    -webkit-print-color-adjust: exact;
  }
</style>