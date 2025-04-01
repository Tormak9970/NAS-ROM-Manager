<script lang="ts">
  import { systems } from "@stores/State";
  import SystemTag from "@views/SystemTag.svelte";

  const systemList = $derived(Object.keys($systems));
</script>

<div class="system-tag-cloud">
  {#each systemList as system}
    <SystemTag system={system} />
  {/each}
</div>

<style>
  .system-tag-cloud {
    width: 100%;

    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }
</style>