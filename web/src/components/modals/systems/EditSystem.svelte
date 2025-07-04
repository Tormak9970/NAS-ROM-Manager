<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { scrollShadow } from "@directives";
  import { DatabaseSearch } from "@icons";
  import { Button, TextField } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { RestService, SystemService } from "@services";
  import { changeGridsId, changeGridsOnSelect, changeGridsSearchId, changeGridsType, igdbSearchPlatformOnSelect, igdbSearchPlatformTitle, sgdbSearchOnSelect, sgdbSearchTitle, showChangeGridsModal, showEditSystemModal, showSearchIGDBPlatformModal, showSearchSGDBModal, systemEditingId } from "@stores/Modals";
  import { systems } from "@stores/State";
  import type { IGDBMetadataPlatform, ParserPattern, System } from "@types";
  import { asyncEvery } from "@utils";
  import type { RgbaColor } from "svelte-awesome-color-picker";
  import TagColorInput from "./TagColorInput.svelte";
  import PatternsInput from "./parser-patterns/PatternsInput.svelte";

  const system = $systems[$systemEditingId!];

  let open = $state(true);
  let saving = $state(false);

  let systemName = $state(system.name);
  let igdbId = $state(system.igdbPlatformId);
  let sgdbId = $state(system.sgdbId);
  let abbreviation = $state(system.abbreviation);
  let fullCapsulePath = $state(system.fullCapsulePath);
  let thumbCapsulePath = $state(system.thumbCapsulePath);
  let heroPath = $state(system.heroPath);
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

    asyncEvery(patterns, SystemService.validateParserPattern).then((isPatternsValid: boolean) => {
      canSave = syncronousChecks && isPatternsValid;
    });
  });

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    saving = true;

    if (sgdbId !== "" && sgdbId !== "None") {
      if (fullCapsulePath !== system.fullCapsulePath) {
        const [fullCached, thumbCached] = await RestService.cacheCapsule(
          fullCapsulePath,
          thumbCapsulePath,
          $systemEditingId!.replace(/[/\\?%*:|"<> ]/g, '_')
        )

        fullCapsulePath = fullCached;
        thumbCapsulePath = thumbCached;
      }
      
      if (heroPath !== system.heroPath) {
        const heroCached = await RestService.cacheHero(
          heroPath!,
          $systemEditingId!.replace(/[/\\?%*:|"<> ]/g, '_')
        );
        
        heroPath = heroCached;
      }
    }

    const updatedParser: System = {
      name: systemName,
      abbreviation: abbreviation,
      folder: folder,
      igdbPlatformId: igdbId,
      sgdbId: sgdbId,
      fullCapsulePath: fullCapsulePath,
      thumbCapsulePath: thumbCapsulePath,
      heroPath: heroPath,
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
    
    open = false;
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

  function editCapsule() {
    $changeGridsSearchId = $systems[$systemEditingId!].sgdbId;
    $changeGridsType = "Capsule";
    $changeGridsOnSelect = (fullCapsule?: string, thumbCapsule?: string) => {
      fullCapsulePath = fullCapsule!;
      thumbCapsulePath = thumbCapsule!;
    };
    $changeGridsId = abbreviation;
    $showChangeGridsModal = true;
  }

  function editHero() {
    $changeGridsSearchId = $systems[$systemEditingId!].sgdbId;
    $changeGridsType = "Hero";
    $changeGridsOnSelect = (fullCapsule?: string, thumbCapsule?: string, hero?: string) => {
      heroPath = hero!;
    };
    $changeGridsId = abbreviation;
    $showChangeGridsModal = true;
  }
</script>

<ModalBody
  headline="Edit System"
  open={open}
  onclose={() => { $showEditSystemModal = false }}
>
  <div class="content">
    {#if saving}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Saving Changes...</div>
      </div>
    {:else}
      <div class="scroll-container styled-scrollbar" use:scrollShadow={{ background: "--m3-scheme-surface-container" }}>
        <div class="fields">
          <TextField
            name="Name"
            placeholder="System name"
            bind:value={systemName}
            trailingIcon={DatabaseSearch}
            ontrailingClick={openIGDBSearch}
          />
          <TextField
            name="IGDB Id"
            placeholder="System IGDB ID"
            bind:value={igdbId}
            trailingIcon={DatabaseSearch}
            ontrailingClick={openIGDBSearch}
          />
          <TextField
            name="SGDB Id"
            placeholder="System SGDB ID"
            bind:value={sgdbId}
            trailingIcon={DatabaseSearch}
            ontrailingClick={openSGDBSearch}
          />
          <div class="actions" style:--m3-button-shape="var(--m3-util-rounding-small)">
            <Button type="filled" extraOptions={{ style: "flex-grow: 1" }} onclick={editCapsule}>
              Edit Cover
            </Button>
            <Button type="filled" extraOptions={{ style: "flex-grow: 1" }} onclick={editHero}>
              Edit Banner
            </Button>
          </div>
          <TextField
            name="Abbreviation"
            placeholder="System abbreviation (ex: GBA)"
            bind:value={abbreviation}
          />
          <TextField
            name="Folder"
            placeholder="System folder"
            bind:value={folder}
          />
          <div class="footnote">
            Using the folder name listed on the <a href="https://emudeck.github.io/cheat-sheet/" target="_blank" rel="noreferrer noopenner">EmuDeck Wiki</a> is strongly recommended.
          </div>
          <TagColorInput bind:tagColor={tagColor} />
          <PatternsInput systemFolder={folder} bind:patterns={patterns} />
        </div>
      </div>
    {/if}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onSave} disabled={!canSave || saving}>Save</Button>
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

  .actions {
    width: 100%;

    display: flex;
    justify-content: space-between;

    gap: 0.5rem;

    margin-bottom: 0.25rem;
  }

  .actions > :global(button) {
    width: calc(50% - 0.25rem);
  }
</style>
