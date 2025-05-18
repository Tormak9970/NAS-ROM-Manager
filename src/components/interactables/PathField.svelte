<script lang="ts">
  import { Folder } from "@icons";
  import TextField from "@interactables/TextField.svelte";
  import { DialogService } from "@services";
  import { FileSelectionType, type FilePickerFilter } from "@types";
  import { isValidLinuxPath, isValidWindowsPath } from "@utils";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLInputAttributes;
    type: FileSelectionType;
    filter?: FilePickerFilter;
    extensions?: string[];
    name: string;
    disabled?: boolean;
    placeholder?: string;
    value?: string;
    onchange?: (e: Event) => void;
    oninput?: (e: Event) => void;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    type,
    filter = () => true,
    extensions = [],
    name,
    disabled = false,
    placeholder = " ",
    value = $bindable("/"),
    onchange = () => {},
    oninput = () => {},
  }: Props = $props();

  const validatePath = async (value: string) => isValidWindowsPath(value) || isValidLinuxPath(value);

  async function getPathFromDialog() {
    const paths = await DialogService.openFilePicker({
      select: type,
      startPath: value,
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
  {name}
  trailingIcon={Folder}
  {disabled}
  validate={validatePath}
  {placeholder}
  bind:value
  {onchange}
  {oninput}
  ontrailingClick={getPathFromDialog}
  {extraOptions}
  {extraWrapperOptions}
/>