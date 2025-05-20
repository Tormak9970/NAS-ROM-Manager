<script lang="ts">
  import { Icon } from "@component-utils";
  import { scrollShadow } from "@directives";
  import { Celebration, ForwardArrow } from "@icons";
  import { Button } from "@interactables";
  import { Card } from "@layout";
  import { showUpdateModal } from "@stores/Modals";
  import { emulators, isLandscape, roms, romsBySystem, systems } from "@stores/State";
  import { updateManifest } from "@stores/Update";
  import Statistic from "@views/dashboard/Statistic.svelte";
  import StorageIndicator from "@views/dashboard/StorageIndicator.svelte";
  import SystemTagCloud from "@views/dashboard/SystemTagCloud.svelte";
  import SystemTag from "@views/SystemTag.svelte";

  const emulatorCount = $derived(Object.keys($emulators).length);
  const romCount = $derived(Object.keys($roms).length);
  const systemCount = $derived(Object.keys($systems).length);

  const firstSystem = $derived(Object.keys($romsBySystem)[0]);
  const biggestPlatform = $derived(Object.keys($romsBySystem).reduce((biggest: string, system: string) => {
    return $romsBySystem[system].length > $romsBySystem[biggest].length ? system : biggest;
  }, firstSystem));

  const cardOptions = {
    style: "width: calc(100% - 2rem); max-width: 25rem;"
  }
</script>

<svelte:head>
	<title>Dashboard - NRM</title>
  <meta name="description" content="Your personal NRM Dashboard" />
</svelte:head>

<div id="dashboard" use:scrollShadow={{ background: "--m3-scheme-background" }}>
  <div class="content">
    <Card type="elevated" extraOptions={cardOptions}>
      <div class="card-header m3-font-title-medium">Your Collection</div>
      <div class="body">
        <ul>
          <Statistic label="Emulators">{emulatorCount}</Statistic>
          <Statistic label="Systems">{systemCount}</Statistic>
          <Statistic label="ROMs">{romCount}</Statistic>
        </ul>
      </div>
    </Card>
    <Card type="elevated" extraOptions={cardOptions}>
      <div class="card-header m3-font-title-medium">Systems</div>
      <div class="body">
        <ul>
          <Statistic label="Biggest System">
            <SystemTag system={biggestPlatform} />
          </Statistic>
          <Statistic label="Systems"></Statistic>
          <SystemTagCloud />
        </ul>
      </div>
    </Card>
    <Card type="elevated" extraOptions={cardOptions}>
      <div class="card-header m3-font-title-medium">Server Info</div>
      <div class="body">
        <ul>
          <Statistic label="Frontend Version">{NRM_FRONTEND_VERSION}</Statistic>
          <Statistic label="Backend Version">{import.meta.env.NRM_BACKEND_VERSION}</Statistic>
          <Statistic label="Build Date">{import.meta.env.NRM_BUILD_DATE}</Statistic>
        </ul>
      </div>
    </Card>
    {#if $updateManifest?.available && !$isLandscape}
      <Card type="outlined" extraOptions={cardOptions}>
        <div class="card-header m3-font-title-medium">Update Available!</div>
        <div class="body">
          <ul>
            <Statistic label="Version">
              <div class="version">
                <div>v{$updateManifest?.currentVersion}</div>
                <Icon icon={ForwardArrow} />
                <div>{$updateManifest?.version}</div>
              </div>
            </Statistic>
            
          </ul>
          <div class="update-container" style:--m3-button-shape="var(--m3-util-rounding-small)">
            <Button
              iconType="left"
              type="tonal"
              extraOptions={{ style: "width: 100%" }}
              onclick={() => $showUpdateModal = true}
            >
              <Icon icon={Celebration} />
              View Details
            </Button>
          </div>
        </div>
      </Card>
    {/if}
    <StorageIndicator extraOptions={cardOptions} />
    <div style="width: 100%; height: 0.5rem;"></div>
  </div>
</div>

<style>
  #dashboard {
    width: 100%;
    height: 100%;

    overflow-y: scroll;
    overflow-x: hidden;
  }

  .content {
    width: 100%;
    height: fit-content;

    display: flex;
    justify-content: center;
    flex-wrap: wrap;

    gap: 1.5rem;
    
    margin-top: 1rem;

    --m3-font-title-medium-size: 1.25rem;
  }

  #dashboard :global(.type-elevated) {
    box-shadow: none;
  }

  .card-header {
    font-weight: bold;
  }

  .body {
    width: 100%;

    margin-bottom: -1rem;
  }

  ul {
    width: 100%;

    list-style: none;

    margin-top: 0.5rem;

    padding-left: 0;
  }

  .version {
    display: flex;
    align-items: center;

    gap: 0.25rem;
  }

  .update-container {
    margin-bottom: 1rem;
  }
</style>