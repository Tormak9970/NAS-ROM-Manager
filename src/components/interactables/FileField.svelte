<script lang="ts">
  import { Folder } from "@icons";
  import { createEventDispatcher } from "svelte";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";
  import TextField from "./TextField.svelte";

  export let extraWrapperOptions: HTMLAttributes<HTMLDivElement> = {};
  export let extraOptions: HTMLInputAttributes = {};
  export let name: string;

  export let fileExtensions: string[] = [];

  export let disabled = false;

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
  accept={fileExtensions.length > 0 ? fileExtensions.join(",") : undefined}
  on:change={handleFilePrompt}
  bind:this={fileElement}
/>
<TextField
  name={name}
  trailingIcon={Folder}
  disabled={disabled}
  readonly
  on:trailingClick={() => fileElement.click()}
  extraOptions={extraOptions}
  extraWrapperOptions={extraWrapperOptions}
  bind:value
/>