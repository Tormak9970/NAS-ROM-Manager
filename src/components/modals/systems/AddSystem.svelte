<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { DatabaseSearch } from "@icons";
  import { Button, TextField } from "@interactables";
  import { igdbSearchPlatformOnSelect, igdbSearchPlatformTitle, showAddSystemModal, showSearchIGDBPlatformModal } from "@stores/Modals";
  import type { IGDBMetadataPlatform, ParserPattern } from "@types";
  import type { RgbaColor } from "svelte-awesome-color-picker";
  import TagColorInput from "./TagColorInput.svelte";
  import PatternsInput from "./parser-patterns/PatternsInput.svelte";

  let open = $state(true);

  let title = $state("");
  let igdbId = $state("");
  let abbreviation = $state("");
  let folder = $state("");
  let tagColor = $state<RgbaColor>({
    r: 228,
    g: 0,
    b: 15,
    a: 1
  });
  const tagConfigColor = $derived(`${tagColor.r} ${tagColor.g} ${tagColor.b}`);
  let patterns = $state<ParserPattern[]>([
    {
      glob: "",
      regex: "",
      downloadStrategy: {
        type: "single-file"
      }
    }
  ]);

  const canSave = $derived(!!title && !!igdbId && !!abbreviation && !!folder && patterns.length > 0);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    open = false;

    const newParser = {
      name: title,
      abbreviation: abbreviation,
      igdbPlatformId: igdbId,
      folder: folder,
      tagConfig: {
        backgroundColor: tagConfigColor,
        borderColor: tagConfigColor,
      },
      patterns: patterns,
    }

    // TODO: save to backend
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
  
  function openIGDBSearch() {
    $igdbSearchPlatformTitle = title;
    $igdbSearchPlatformOnSelect = (platform: IGDBMetadataPlatform | null) => {
      if (platform) {
        title = platform.name;
        igdbId = platform.igdbId;
        abbreviation = platform.abbreviation;
      }
    }
    $showSearchIGDBPlatformModal = true;
  }
</script>

<ModalBody
  headline="Add System"
  open={open}
  canClose={false} 
  oncloseend={() => { $showAddSystemModal = false }}
>
  <div class="content">
    <TextField
      name="Name"
      bind:value={title}
      trailingIcon={DatabaseSearch}
      ontrailingClick={openIGDBSearch}
    />
    <!-- TODO: error if there is a platform with this name already -->
    <TextField
      name="IGDB Id"
      bind:value={igdbId}
    />
    <TextField
      name="Abbreviation"
      bind:value={abbreviation}
    />
    <TextField
      name="Folder"
      bind:value={folder}
    />
    <div class="footnote">
      Using the folder name listed on the <a href="https://emudeck.github.io/cheat-sheet/" target="_blank" rel="noreferrer noopenner">EmuDeck Wiki</a> is strongly recommended.
    </div>
    <TagColorInput bind:tagColor={tagColor} />
    <PatternsInput bind:patterns={patterns} />
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onSave} disabled={!canSave}>Create</Button>
    </div>
  {/snippet}
</ModalBody>

<style>
  .content {
    width: 100%;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
