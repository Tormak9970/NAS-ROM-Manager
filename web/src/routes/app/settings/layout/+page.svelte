<script lang="ts">
  import { routes } from "$lib/routes";
  import { Route } from "@icons";
  import { isLandscapeRoutes, showEditRouteOrderModal } from "@stores/Modals";
  import { landingPage } from "@stores/State";
  import { ButtonSetting, SelectSetting, SettingsBody } from "@views/settings";

  let pages = Object.entries(routes).map(([key, value]) => {
    return {
      label: key,
      value: value.path.substring(1)
    };
  });
</script>

<svelte:head>
	<title>Layout Settings - NRM</title>
  <meta name="description" content="View and modify the layout settings of NRM." />
</svelte:head>

<SettingsBody title="Layout">
  <SelectSetting
    label="Landing Page"
    description="The first page opened"
    icon={Route}
    options={pages}
    bind:value={$landingPage}
  />
  <ButtonSetting
    label="Edit Desktop Routes"
    description="Edit the order of desktop navigation menu"
    onclick={() => {
      $isLandscapeRoutes = true;
      $showEditRouteOrderModal = true;
    }}
  />
  <ButtonSetting
    label="Edit Mobile Routes"
    description="Edit the order of mobile navigation menu"
    onclick={() => {
      $isLandscapeRoutes = false;
      $showEditRouteOrderModal = true;
    }}
  />
</SettingsBody>