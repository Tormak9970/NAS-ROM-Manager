<script lang="ts">
  import type { ROMMetadata } from "@types";

  type Props = {
    metadata: ROMMetadata
  }

  let { metadata }: Props = $props();
</script>

<div class="details metadata">
  <div class="text-info">
    <h2>Overview</h2>
    <div class="summary body-text">{metadata.metadata?.summary ?? "No overview was available"}</div>
    
  </div>
  <div class="table-info">

  </div>
</div>
<div class="similar-games">

</div>

<style>
  
</style>