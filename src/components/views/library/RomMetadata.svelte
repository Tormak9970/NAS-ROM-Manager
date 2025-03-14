<script lang="ts">
  import type { ROMMetadata } from "@types";

  type Props = {
    metadata: ROMMetadata
  }

  let { metadata }: Props = $props();
</script>

<div class="metadata">
  <div class="text-info">
    <h1 class="m3-font-headline-large">Overview</h1>
    <div class="summary body-text">{metadata.metadata?.summary ?? "No overview was available"}</div>
    
  </div>
  <div class="table-info">

  </div>
</div>
<div class="similar-games">

</div>

<style>
  .font-label-large {
    font-size: 1.1rem;
    line-height: 1.25rem;
    letter-spacing: 0.00625rem;
    font-weight: 500;
  }

  /* TODO: move this to app.css once finalized */
  .body-text {
    /* TODO: taller line spacing */

    color: rgb(var(--m3-scheme-on-surface-variant));
    
    /* TODO: larger font size */
  }
</style>