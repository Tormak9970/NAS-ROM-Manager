<script lang="ts">
  import { BackHandOutline, DonutLarge, GridViewOutline, InfoOutline, InterestsOutline } from "@icons";
  import { SettingsBody, SettingsNavButton } from "@views/settings";

</script>

<svelte:head>
	<title>Settings - NRM</title>
  <meta name="description" content="View and modify the settings of your NRM instance." />
</svelte:head>

<SettingsBody title="Settings">
  <SettingsNavButton
    icon={InterestsOutline}
    label="Appearance"
    route="/settings/appearance"
    description="Manage the app's appearance settings"
  />
  <SettingsNavButton
    icon={GridViewOutline}
    label="Layout"
    route="/settings/layout"
    description="Manage the app's layout and views"
  />
  <SettingsNavButton
    icon={DonutLarge}
    label="Metadata"
    route="/settings/metadata"
    description="Manage how rom metadata is saved"
  />
  <SettingsNavButton
    icon={BackHandOutline}
    label="Accessibility"
    route="/settings/accessibility"
    description="Manage the app's accessibility settings"
  />
  <SettingsNavButton
    icon={InfoOutline}
    label="App Info"
    route="/settings/app-info"
    description="View information about the app"
  />
</SettingsBody>