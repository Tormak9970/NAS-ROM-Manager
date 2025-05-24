<script lang="ts">
  import { Icon } from "@component-utils";
  import { EmergencyHome } from "@icons";
  import { Card } from "@layout";
  import { landingPage } from "@stores/State";
  import { BackendErrorType } from "@types";
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>An Error Occured</title>
</svelte:head>

<div id="error">
  <Card type="elevated">
    <div id="headline"></div>
    <div class="container">
      <Icon icon={EmergencyHome} width="4rem" height="4rem" />
      <div class="info">
        <div>{data.message}</div>
        <div>{data.fix}</div>
        <div class="notice">
          {#if data.type === BackendErrorType.PANIC.toString()}
            This error is not recoverable and will require a navigation to the Home Page or a container restart.
          {:else}
            Go <a href={$landingPage ?? "/"}>Home</a>
          {/if}
        </div>
      </div>
    </div>
  </Card>
</div>

<style>
  #error {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  #headline {
    width: calc(100% + 2rem);
    margin: -1rem -1rem 0.5rem -1rem;
    height: 0.5rem;
    background-color: rgb(var(--m3-scheme-error));
  }

  .container {
    display: flex;
    align-items: center;

    gap: 2rem;

    padding: 1rem;
    padding-right: 1.5rem;

    border-radius: var(--m3-util-rounding-medium);

    font-size: 1.2rem;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .info > :first-child {
    font-weight: bold;
    font-size: 1.5rem;
  }

  .notice {
    margin-top: 1.25rem;
    margin-bottom: -0.75rem;
    font-size: 1rem;
  }

  a, a:visited {
    font-weight: bold;
    text-decoration: underline;
  }

  #error :global(svg) {
    color: rgb(var(--m3-scheme-error));
  }

  #error :global(.type-elevated) {
    background-color: rgb(var(--m3-scheme-surface-container));
    box-shadow: var(--m3-util-elevation-2);
  }
</style>