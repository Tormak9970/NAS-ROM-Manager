<script lang="ts">
  import { MotionBlur } from "@icons";
  import { reducedMotion } from "@stores/State";
  import { SettingsBody, ToggleSetting } from "@views/settings";

</script>

<svelte:head>
	<title>Accessibility Settings - NRM</title>
  <meta name="description" content="View and modify the accessibility settings of NRM." />
</svelte:head>

<SettingsBody title="Accessibility">
  <ToggleSetting
    icon={MotionBlur}
    iconSize="2rem"
    label="Reduced Motion"
    description="Reduces hover effects and transitions throughout the app"
    bind:checked={$reducedMotion}
  />
</SettingsBody>