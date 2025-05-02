<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { SystemController } from "@controllers";
  import { scrollShadow } from "@directives";
  import { DatabaseSearch } from "@icons";
  import { Button, TextField } from "@interactables";
  import { igdbSearchPlatformOnSelect, igdbSearchPlatformTitle, showAddSystemModal, showSearchIGDBPlatformModal } from "@stores/Modals";
  import { systems } from "@stores/State";
  import type { IGDBMetadataPlatform, ParserPattern, System } from "@types";
  import { asyncEvery } from "@utils";
  import type { RgbaColor } from "svelte-awesome-color-picker";
  import TagColorInput from "./TagColorInput.svelte";
  import PatternsInput from "./parser-patterns/PatternsInput.svelte";

  let systemsList = Object.values($systems);

  let open = $state(true);

  let systemName = $state("");
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
      glob: "*.(?i){nsp,xci,nca}",
      regex: ".+\\/(?<title>.+)\\.(?:nsp|xci|nca)$",
      downloadStrategy: {
        type: "single-file"
      }
    }
  ]);

  let canSave = $state(false);

  $effect(() => {
    const syncronousChecks = !!systemName &&
      !systemsList.some((system) => system.name === systemName) &&
      !!igdbId &&
      !!abbreviation &&
      !!folder &&
      patterns.length > 0;

    asyncEvery(patterns, SystemController.validateParserPattern).then((isPatternsValid: boolean) => {
      canSave = syncronousChecks && isPatternsValid;
    });
  });

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    open = false;

    const newParser: System = {
      name: systemName,
      abbreviation: abbreviation,
      folder: folder,
      igdbPlatformId: igdbId,
      sgdbId: "",
      fullCapsulePath: "",
      thumbCapsulePath: "",
      heroPath: "",
      tagConfig: {
        backgroundColor: tagConfigColor,
        borderColor: tagConfigColor,
      },
      patterns: patterns,
    }

    $systems[abbreviation] = newParser;
    $systems = { ...$systems };
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }
  
  function openIGDBSearch() {
    $igdbSearchPlatformTitle = systemName;
    $igdbSearchPlatformOnSelect = (platform: IGDBMetadataPlatform | null) => {
      if (platform) {
        systemName = platform.name;
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
  oncloseend={() => { $showAddSystemModal = false }}
>
  <div class="content">
    <div class="scroll-container styled-scrollbar" use:scrollShadow={{ background: "--m3-scheme-surface-container" }}>
      <div class="fields">
        <TextField
          name="Name"
          validate={async (value: string) => !systemsList.some((system) => system.name === value)}
          bind:value={systemName}
          trailingIcon={DatabaseSearch}
          ontrailingClick={openIGDBSearch}
        />
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
    </div>
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
    height: 100%;
  }

  .scroll-container {
    height: 100%;
    width: 100%;

    overflow-y: auto;
    overflow-x: hidden;
  }

  .fields {
    width: 100%;
    
    margin-top: 0.5rem;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
