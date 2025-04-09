<script lang="ts">
    import { Icon } from "@component-utils";
    import { WebsocketController } from "@controllers";
    import { Close } from "@icons";
    import { Button, Select, TextField } from "@interactables";
    import { Accordion } from "@layout";
    import type { ParserPattern } from "@types";
    import { isValidRegex } from "@utils";

  type Props = {
    label: string;
    pattern: ParserPattern;
    onDelete: () => void;
  }

  let {
    label,
    pattern = $bindable(),
    onDelete,
  }: Props = $props();

  let options: SelectItem[] = [
    { label: "Single File", value: "single-file" },
    { label: "Folder", value: "folder" }
  ];
  
  let downloadOptionValue = $state("single-file");

  let parentFolder = $state("");

  $effect(() => {
    pattern.downloadStrategy = downloadOptionValue === "single-file" ? {
      type: "single-file"
    } : {
      type: "folder",
      parent: parentFolder
    }
  })
</script>

<Accordion label={label}>
  {#snippet header()}
    <Button type="tonal" warning iconType="full" size="2rem" iconSize="1.25rem" onclick={onDelete}>
      <Icon icon={Close} />
    </Button>
  {/snippet}
  <div class="pattern">
    <TextField
      name="Glob"
      extraWrapperOptions={{ style: "width: 100%" }}
      validate={WebsocketController.isValidGlob}
      bind:value={pattern.glob}
    />
    <div class="footnote">
      An outline of the glob syntax can be found <a href="https://github.com/olson-sean-k/wax?tab=readme-ov-file#patterns" target="_blank" rel="noreferrer noopenner">here</a>.
    </div>

    <TextField
      name="Regex"
      extraWrapperOptions={{ style: "width: 100%" }}
      validate={async (regex: string) => isValidRegex(regex)}
      bind:value={pattern.regex}
    />
    <div class="footnote">
      To test and learn about RegEx, check out <a href="https://regex101.com/" target="_blank" rel="noreferrer noopenner">https://regex101.com/</a>.
    </div>

    <Select
      name="Download Strategy"
      options={options}
      bind:value={downloadOptionValue}
    />
    <div class="footnote" style:margin-top="0.25rem">
      How roms on this system will be downloaded. "Single File" is for single file roms, "Folder" is for roms that are folders with data.
    </div>

    {#if downloadOptionValue === "folder"}
      <TextField
        name="Parent Folder"
        extraWrapperOptions={{ style: "width: 100%" }}
        validate={async (value: string) => value !== ""}
        bind:value={parentFolder}
      />
      <div class="footnote">
        This is the parent folder of the folder that will be zipped when downloading. Its usually the system's folder.
      </div>
    {/if}
    
    <div class="examples">
      For examples, take a look at the <a href="https://github.com/Tormak9970/NAS-ROM-Manager/tree/main/parsers" target="_blank" rel="noreferrer noopenner">default parsers</a>.
    </div>
  </div>
</Accordion>

<style>
  .footnote {
    margin-bottom: 1rem;
  }

  .examples {
    margin-top: 0.25rem;
  }
</style>