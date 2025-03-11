<script lang="ts">
  import { scrollShadow } from "@directives";
  import { Card } from "@layout";
  import { emulators, roms, romsBySystem, systems } from "@stores/State";
  import Statistic from "@views/dashboard/Statistic.svelte";
  import StorageIndicator from "@views/dashboard/StorageIndicator.svelte";
  import SystemTagCloud from "@views/dashboard/SystemTagCloud.svelte";
  import SystemTag from "@views/SystemTag.svelte";

  let emulatorCount = $derived(Object.keys($emulators).length);
  let romCount = $derived(Object.keys($roms).length);
  let systemCount = $derived(Object.keys($systems).length);

  let firstSystem = $derived(Object.keys($romsBySystem)[0]);
  let biggestPlatform = $derived(Object.keys($romsBySystem).reduce((biggest: string, system: string) => {
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
</style>