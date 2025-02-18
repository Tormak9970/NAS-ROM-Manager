<script lang="ts">
  import { roms } from "@stores/State";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  let rom = $derived($roms[data.id]);
</script>

<svelte:head>
	<title>Library - NRM</title>
  <meta name="description" content="Your personal ROM library." />
</svelte:head>

<div id="rom-entry">
  
</div>

<style>
  #rom-entry {
    width: 100%;
    height: 100%;
  }
</style>