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

  let fileElement: HTMLInputElement;
  
  const dispatch = createEventDispatcher();
  
  let value = "";

  function handleFilePrompt(e: Event) {
    const files = (e.currentTarget as HTMLInputElement).files;

    if (files && files.length > 0) {
      value = files[0].name;

      dispatch("change", {
        value: files[0]
      });
    }
  }
</script>

<input
  type="file"
  id="fileElem"
  style="display:none"
  on:change={handleFilePrompt}
  bind:this={fileElement}
/>
<TextField
  name={name}
  trailingIcon={Folder}
  disabled={disabled}
  error={error}
  readonly
  on:trailingClick={() => fileElement.click()}
  extraOptions={extraOptions}
  extraWrapperOptions={extraWrapperOptions}
  bind:value
/>