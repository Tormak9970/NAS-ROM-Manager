<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { SystemController } from "@controllers";
  import { scrollShadow } from "@directives";
  import { DatabaseSearch } from "@icons";
  import { Button, TextField } from "@interactables";
  import { igdbSearchPlatformOnSelect, igdbSearchPlatformTitle, sgdbSearchOnSelect, sgdbSearchTitle, showEditSystemModal, showSearchIGDBPlatformModal, showSearchSGDBModal, systemEditingId } from "@stores/Modals";
  import { systems } from "@stores/State";
  import type { IGDBMetadataPlatform, ParserPattern, System } from "@types";
  import { asyncEvery } from "@utils";
  import type { RgbaColor } from "svelte-awesome-color-picker";
  import TagColorInput from "./TagColorInput.svelte";
  import PatternsInput from "./parser-patterns/PatternsInput.svelte";

  const system = $systems[$systemEditingId!];

  let open = $state(true);

  let systemName = $state(system.name);
  let igdbId = $state(system.igdbPlatformId);
  let sgdbId = $state(system.sgdbId);
  let abbreviation = $state(system.abbreviation);
  let folder = $state(system.folder);

  const colors = system.tagConfig.backgroundColor.split(" ").map((color: string) => parseInt(color));
  let tagColor = $state<RgbaColor>({
    r: colors[0],
    g: colors[1],
    b: colors[2],
    a: 1
  });
  const tagConfigColor = $derived(`${tagColor.r} ${tagColor.g} ${tagColor.b}`);
  let patterns = $state<ParserPattern[]>(system.patterns);

  let canSave = $state(false);

  $effect(() => {
    const syncronousChecks = !!systemName &&
      !!igdbId &&
      !!sgdbId &&
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

    const updatedParser: System = {
      name: systemName,
      abbreviation: abbreviation,
      folder: folder,
      igdbPlatformId: igdbId,
      sgdbId: sgdbId,
      coverPath: system.coverPath,
      thumbPath: system.thumbPath,
      tagConfig: {
        backgroundColor: tagConfigColor,
        borderColor: tagConfigColor,
      },
      patterns: patterns,
    }

    $systems[abbreviation] = updatedParser;

    if (abbreviation !== system.abbreviation) {
      delete $systems[system.abbreviation];
    }

    $systems = { ...$systems };
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
    $systemEditingId = null;
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
  
  function openSGDBSearch() {
    $sgdbSearchTitle = systemName;
    $sgdbSearchOnSelect = (id: string) => {
      sgdbId = id;
    }
    $showSearchSGDBModal = true;
  }
</script>

<ModalBody
  headline="Edit System"
  open={open}
  oncloseend={() => { $showEditSystemModal = false }}
>
  <div class="content">
    <div class="scroll-container" use:scrollShadow={{ background: "--m3-scheme-surface-container" }}>
      <div class="fields">
        <TextField
          name="Name"
          bind:value={systemName}
          trailingIcon={DatabaseSearch}
          ontrailingClick={openIGDBSearch}
        />
        <TextField
          name="IGDB Id"
          bind:value={igdbId}
          trailingIcon={DatabaseSearch}
          ontrailingClick={openIGDBSearch}
        />
        <TextField
          name="SGDB Id"
          bind:value={sgdbId}
          trailingIcon={DatabaseSearch}
          ontrailingClick={openSGDBSearch}
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
      <Button type="tonal" onclick={onSave} disabled={!canSave}>Save</Button>
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

    overflow-y: scroll;
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
