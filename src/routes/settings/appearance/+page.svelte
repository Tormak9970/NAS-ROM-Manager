<script lang="ts">
  import { BrightnessMedium, DarkMode, Palette as Theme } from "@icons";
  import { palette, themePrimaryColor, useOledPalette } from "@stores/State";
  import { ColorPresets, ColorSetting, SettingsBody, ToggleSetting } from "@views/settings";

  let auto = $state($palette === "Auto");
  let darkMode = $state($palette === "Dark");

  $effect(() => {
    if (auto) {
      $palette = "Auto";
    } else if (darkMode) {
      $palette = "Dark";
    } else {
      $palette = "Light";
    }
  });
</script>

<svelte:head>
	<title>Appearance Settings - NRM</title>
  <meta name="description" content="View and modify the appearance settings of NRM." />
</svelte:head>

<SettingsBody title="Appearance">
  <ColorSetting icon={Theme} label="App Theme" description="Customize the color palette of the app" bind:color={$themePrimaryColor} />
  <ColorPresets />
  <ToggleSetting icon={BrightnessMedium} label="Use System Palette" description="Determine the theme's palette from your system" bind:checked={auto} />
  {#if !auto}
    <ToggleSetting icon={DarkMode} label="Use Dark Mode" description="Use the theme's dark mode palette" bind:checked={darkMode} />
  {/if}
  <ToggleSetting label="OLED Dark" description="Use an all black theme for the app" bind:checked={$useOledPalette} />
</SettingsBody>