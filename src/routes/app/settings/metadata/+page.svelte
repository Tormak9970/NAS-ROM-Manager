<script lang="ts">
  import { CloudDownload } from "@icons";
  import { saveMetadataAlongside } from "@stores/State";
  import { SettingsBody, ToggleSetting } from "@views/settings";
</script>

<svelte:head>
	<title>Metadata Settings - NRM</title>
  <meta name="description" content="View and modify the metadata settings of NRM." />
</svelte:head>

<SettingsBody title="Metadata">
  <ToggleSetting
    icon={CloudDownload}
    iconSize="1.5rem"
    label="Save ROM Metadata with ROMs"
    description="Saves ROM metadata in files with the ROMs."
    bind:checked={$saveMetadataAlongside}
  />
</SettingsBody>