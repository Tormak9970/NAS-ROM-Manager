<script lang="ts">
    import { DialogController } from "@controllers";
    import { Help } from "@icons";
    import TextField from "@interactables/TextField.svelte";
    import { isValidLinuxPath, isValidWindowsPath } from "@utils";
    import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";

  export let extraWrapperOptions: HTMLAttributes<HTMLDivElement> = {};
  export let extraOptions: HTMLInputAttributes = {};
  export let name: string;

  export let disabled = false;
  export let value = "";

  const validatePath = (value: string) => isValidWindowsPath(value) || isValidLinuxPath(value);

  function showPathHelp() {
    DialogController.message("Valid Paths", "A valid file path should only use forward slashes. It should have no backslashes.", "Close");
  }
</script>

<TextField
  name={name}
  trailingIcon={Help}
  disabled={disabled}
  validate={validatePath}
  bind:value
  on:change
  on:input
  on:trailingClick={showPathHelp}
  extraOptions={extraOptions}
  extraWrapperOptions={extraWrapperOptions}
/>