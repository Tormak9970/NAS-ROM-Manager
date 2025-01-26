<script lang="ts">
  import { goto } from "$app/navigation";
  import { LoadingSpinner } from "@layout";
  import { loadedSettings, landingPage } from "@stores/State";

  $effect(() => {
    if ($loadedSettings) {
      goto(`/${$landingPage}`);
    }
  });
</script>

<svelte:head>
	<title>Loading...</title>
</svelte:head>

<div id="loading">
  <LoadingSpinner extraOptions={{ style: "width: 5rem; height: 5rem;"}} />
</div>

<style>
  #loading {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>