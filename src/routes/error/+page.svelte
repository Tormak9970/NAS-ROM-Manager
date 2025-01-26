<script lang="ts">
  import type { PageData } from './$types';
  import { Icon } from "@component-utils";
  import { EmergencyHome } from "@icons";
  import { landingPage } from "@stores/State";

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>An Error Occured</title>
</svelte:head>

<div id="error">
  <div class="container">
    <Icon icon={EmergencyHome} width="5rem" height="5rem" />
    <div class="info">
      <div>{data.message}</div>
      <div>{data.fix}</div>
      <div style:margin-top="0.5rem">
        Go <a href={$landingPage ?? "/"}>Home</a>
      </div>
    </div>
  </div>
</div>

<style>
  #error {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;
  }

  .container {
    display: flex;
    align-items: center;

    gap: 2rem;

    background-color: rgb(var(--m3-scheme-error));

    padding: 1rem;
    padding-right: 1.5rem;

    border-radius: var(--m3-util-rounding-medium);

    font-size: 1.2rem;

    color: rgb(var(--m3-scheme-on-error));
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

  a, a:visited {
    font-weight: bold;
    color: rgb(var(--m3-scheme-on-error));
  }

  a:hover {
    color: rgb(var(--m3-scheme-on-error) / 0.8);
  }
</style>