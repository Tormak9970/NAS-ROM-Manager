<script lang="ts">
  import { goto } from "$app/navigation";
  import { LoadingSpinner } from "@layout";
  import { landingPage, loadedLibrary, loadedSettings } from "@stores/State";
  import type { PageData } from './$types';
  
  let { data }: { data: PageData } = $props();

  $effect(() => {
    if ($loadedSettings) {
      goto(`/loading?message=${encodeURIComponent("Loading Library...")}`);
    }

    if ($loadedLibrary) {
      goto(`/${$landingPage}`);
    }
  });
</script>

<svelte:head>
	<title>{data.message ?? "Loading..."}</title>
</svelte:head>

<div id="loading">
  <LoadingSpinner extraOptions={{ style: "width: 5rem; height: 5rem;"}} />
  <div class="m3-font-title-large text">{data.message ?? "Loading..."}</div>
</div>

<style>
  #loading {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;

    gap: 0.5rem;
  }

  .text {
    font-size: 1.25rem;
  }
</style>