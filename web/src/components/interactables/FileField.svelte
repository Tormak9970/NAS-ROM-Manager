<script lang="ts">
  import { Folder } from "@icons";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";
  import TextField from "./TextField.svelte";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLInputAttributes;
    name: string;
    fileExtensions?: string[];
    disabled?: boolean;
    placeholder?: string;
    onchange?: (file: File | undefined) => void;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    name,
    fileExtensions = [],
    disabled = false,
    placeholder = " ",
    onchange = () => {},
  }: Props = $props();

  // @ts-expect-error This will always be defined before its usage.
  let fileElement: HTMLInputElement = $state();
  
  let value = $state("");

  function handleFilePrompt(e: Event) {
    const files = (e.currentTarget as HTMLInputElement).files;

    if (files && files.length > 0) {
      value = files[0].name;

      onchange?.(files[0]);
    }
  }
</script>

<input
  type="file"
  id="fileElem"
  style="display:none"
  accept={fileExtensions.length > 0 ? fileExtensions.join(",") : undefined}
  onchange={handleFilePrompt}
  bind:this={fileElement}
/>
<TextField
  {name}
  trailingIcon={Folder}
  {placeholder}
  bind:value
  ontrailingClick={() => fileElement.click()}
  {extraOptions}
  {extraWrapperOptions}
  readonly
  {disabled}
/>