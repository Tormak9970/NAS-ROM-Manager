<script lang="ts">
	import { page } from '$app/state';

  const error = $derived(page.error!);
</script>

<svelte:head>
	<title>Error {error.code}</title>
</svelte:head>

<div id="error-container">
  <div class="error m3-font-headline-large">
    <div class="code m3-font-headline-small">{error.code}</div>
    <div class="border"></div>
    <div>{error.message}</div>
  </div>
</div>

<style>
  #error-container {
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .error {
    display: flex;
    align-items: center;

    gap: 0.75rem;
  }

  .code {
    color: rgb(var(--m3-scheme-outline));
  }

  .border {
    height: 2.5rem;
    width: 1px;

    background-color: rgb(var(--m3-scheme-outline));
  }
</style>