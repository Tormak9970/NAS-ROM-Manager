<script lang="ts">
  import { BugReport, EditDocument, InfoOutline, License, Person, Star } from "@icons";
  import { showChangelogModal } from "@stores/Modals";
  import { ButtonSetting, SettingsBody, TextSetting } from "@views/settings";

</script>

<svelte:head>
	<title>App Info Settings - NRM</title>
  <meta name="description" content="View and modify the navigation settings of NRM." />
</svelte:head>

<SettingsBody title="App Info">
  <TextSetting label="Version" description={NRM_FRONTEND_VERSION} icon={InfoOutline} />
  <TextSetting label="Developer" description="Travis Lane" icon={Person} />
  <ButtonSetting label="View Changelog" description="View the changes made in each release" icon={EditDocument} onclick={() => $showChangelogModal = true} />
  <ButtonSetting label="GitHub" description="View the app's source and star it!" icon={Star} onclick={() => open("https://github.com/Tormak9970/NAS-ROM-Manager", "_blank")} />
  <ButtonSetting label="Report Bugs" description="Ran into an issue? Let me know!" icon={BugReport} onclick={() => open("https://github.com/Tormak9970/NAS-ROM-Manager/issues/new/choose", "_blank")} />
  <ButtonSetting label="Licensing" description="GPL v3.0" icon={License} onclick={() => open("https://github.com/Tormak9970/NAS-ROM-Manager/blob/main/LICENSE", "_blank")} />
</SettingsBody>