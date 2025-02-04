<script lang="ts">
  import { Folder } from "@icons";
  import { createEventDispatcher } from "svelte";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";
  import TextField from "./TextField.svelte";

  export let extraWrapperOptions: HTMLAttributes<HTMLDivElement> = {};
  export let extraOptions: HTMLInputAttributes = {};
  export let name: string;

  export let disabled = false;
  export let error = false;
  
  const dispatch = createEventDispatcher();
  
  let value = "";

  function handleFilePrompt() {
    // TODO: open file dialog

    // TODO: trigger on:change with the file as the value.
  }
</script>

<TextField
  name={name}
  trailingIcon={Folder}
  disabled={disabled}
  error={error}
  readonly
  on:trailingClick={handleFilePrompt}
  extraOptions={extraOptions}
  extraWrapperOptions={extraWrapperOptions}
  bind:value
/>