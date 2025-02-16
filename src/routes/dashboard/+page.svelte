<script lang="ts">
  import { Card } from "@layout";
  import { libraries, roms, romsBySystem, systems } from "@stores/State";

  let libraryCount = $derived(Object.keys($libraries).length);
  let romCount = $derived(Object.keys($roms).length);
  let systemCount = $derived(Object.keys($systems).length);

  let firstSystem = $derived(Object.keys($romsBySystem)[0]);
  let biggestPlatform = $derived(Object.keys($romsBySystem).reduce((biggest: string, system: string) => {
    return $romsBySystem[system].length > $romsBySystem[biggest].length ? system : biggest;
  }, firstSystem));
</script>

<svelte:head>
	<title>Dashboard</title>
  <meta name="description" content="Your personal NRM Dashboard" />
</svelte:head>

<div id="dashboard">
  <Card type="elevated">
    <div slot="header" class="card-header">Your Collection</div>
    <div class="body">
      <ul>
        <li># Libraries: {libraryCount}</li>
        <li># ROMs: {romCount}</li>
        <li># Systems: {systemCount}</li>
        <li># Emulators: {0}</li>
        <!-- TODO: seperate -->
        <li>Biggest Platform: {romCount}</li>
      </ul>
    </div>
  </Card>
  <Card type="elevated">
    <div slot="header" class="card-header">Systems</div>
    <div class="body">
      <ul>
        <li># Systems: {systemCount}</li>
        <!-- TODO: make this a tag -->
        <li>Biggest System: {biggestPlatform}</li>
        <!-- TODO: make all system tags render here. -->
        <li>Systems: {romCount}</li>
      </ul>
    </div>
  </Card>
  <Card type="elevated">
    <div slot="header" class="card-header">Server Info</div>
    <div class="body">
      <ul>
        <li>Frontend Version: {systemCount}</li>
        <li>Backend Version: {romCount}</li>
        <li>Build Date: {romCount}</li>
      </ul>
    </div>
  </Card>
</div>

<style>
  #dashboard {
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  #dashboard :global(.type-elevated) {
    box-shadow: none;
  }
</style>