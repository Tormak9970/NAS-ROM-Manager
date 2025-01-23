<script>
  import { showAddLibraryModal, showControlledModal } from "@stores/Modals";
  import Controlled from "./Controlled.svelte";
  import AddLibrary from "./AddLibrary.svelte";
</script>

{#if $showControlledModal}
  <Controlled />
{/if}

{#if $showAddLibraryModal}
  <AddLibrary />
{/if}