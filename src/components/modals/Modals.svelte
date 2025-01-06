<script>
  import { showControlledModal } from "@stores/Modals";
  import Controlled from "./Controlled.svelte";
</script>

{#if $showControlledModal}
  <Controlled />
{/if}