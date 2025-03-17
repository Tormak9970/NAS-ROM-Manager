<script lang="ts">
  import { LoadingSpinner } from "@layout";
  import { loadedLibrary } from "@stores/State";
  import type { LoadGuardProps } from "@types";
  import { onDestroy, onMount } from "svelte";
  import type { Unsubscriber } from "svelte/store";

  let { message, onLoad, children }: LoadGuardProps = $props();
  
  let libraryLoadUnsub: Unsubscriber;

  onMount(() => {
    libraryLoadUnsub = loadedLibrary.subscribe(async (loaded) => {
      if (loaded) {
        await onLoad?.();
      }
    });
  });

  onDestroy(() => {
    if (libraryLoadUnsub) libraryLoadUnsub();
  });
</script>

{#if !$loadedLibrary}
  <div class="loading-container">
    <LoadingSpinner /> <div class="font-headline-small">{message ?? "Loading Loading..."}</div>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .loading-container {
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;
  }
</style>