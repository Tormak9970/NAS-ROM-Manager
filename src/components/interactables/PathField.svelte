<script lang="ts">
  import { DialogController } from "@controllers";
  import { Folder } from "@icons";
  import TextField from "@interactables/TextField.svelte";
  import { FileSelectionType, type FilePickerFilter } from "@types";
  import { isValidLinuxPath, isValidWindowsPath } from "@utils";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";

  export let extraWrapperOptions: HTMLAttributes<HTMLDivElement> = {};
  export let extraOptions: HTMLInputAttributes = {};
  export let type: FileSelectionType;
  export let startPath = "";
  export let filter: FilePickerFilter = () => true;
  export let extensions: string[] = [];
  export let name: string;

  export let disabled = false;
  export let value = "";

  const validatePath = (value: string) => isValidWindowsPath(value) || isValidLinuxPath(value);

  async function getPathFromDialog() {
    const paths = await DialogController.openFilePicker({
      select: type,
      startPath: startPath,
      showFiles: type === FileSelectionType.FILE,
      showHiddenFiles: type === FileSelectionType.FILE,
      filter: filter,
      extensions: extensions,
      max: 1
    });

    if (paths[0]) {
      value = paths[0];
    }
  }
</script>

<TextField
  name={name}
  trailingIcon={Folder}
  disabled={disabled}
  validate={validatePath}
  bind:value
  on:change
  on:input
  on:trailingClick={getPathFromDialog}
  extraOptions={extraOptions}
  extraWrapperOptions={extraWrapperOptions}
/>