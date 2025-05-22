<script lang="ts">
  import { AssignmentOutline, BackHandOutline, GridViewOutline, InfoOutline, InterestsOutline } from "@icons";
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
    route="/app/settings/appearance"
    description="Manage the app's appearance settings"
  />
  <SettingsNavButton
    icon={GridViewOutline}
    label="Layout"
    route="/app/settings/layout"
    description="Manage the app's layout and views"
  />
  <!-- <SettingsNavButton
    icon={DonutLarge}
    label="Metadata"
    route="/app/settings/metadata"
    description="Manage how rom metadata is saved"
  /> -->
  <SettingsNavButton
    icon={BackHandOutline}
    label="Accessibility"
    route="/app/settings/accessibility"
    description="Manage the app's accessibility settings"
  />
  <SettingsNavButton
    icon={AssignmentOutline}
    label="Documentation"
    route="https://github.com/Tormak9970/NAS-ROM-Manager"
    isExternal
    description="View the app's documentation"
  />
  <SettingsNavButton
    icon={InfoOutline}
    label="App Info"
    route="/app/settings/app-info"
    description="View information about the app"
  />
</SettingsBody>