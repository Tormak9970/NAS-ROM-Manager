<script lang="ts">
  import { Icon, ModalBody } from "@component-utils";
  import { RestController, RomController } from "@controllers";
  import { Cached, DatabaseSearch } from "@icons";
  import { Button, Checkbox, TextField } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { changeGridsId, changeGridsOnSelect, changeGridsSearchId, changeGridsType, editIsPostUpload, igdbSearchRomOnSelect, igdbSearchRomPlatformId, igdbSearchRomTitle, romEditingId, sgdbSearchOnSelect, sgdbSearchTitle, showChangeGridsModal, showEditRomModal, showSearchIGDBRomModal, showSearchSGDBModal } from "@stores/Modals";
  import { romMetadata, roms, systems } from "@stores/State";

  let open = $state(true);
  let saving = $state(false);

  let rom = $roms[$romEditingId!];
  let system = $systems[rom.system];
  let metadata = $romMetadata[$romEditingId!];

  let title = $state(metadata.title);
  let sgdbId = $state(metadata.sgdbId);
  let igdbId = $state(metadata.igdbId);
  let fullCapsulePath = $state(metadata.fullCapsulePath);
  let thumbCapsulePath = $state(metadata.thumbCapsulePath);
  let heroPath = $state(metadata.heroPath);
  let isFavorite = $state(metadata.isFavorite);
  let igdbMetadata = $state(metadata.metadata);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    saving = true;
    
    if (fullCapsulePath !== system.fullCapsulePath) {
      const [fullCached, thumbCached] = await RestController.cacheCapsule(
        fullCapsulePath,
        thumbCapsulePath,
        $romEditingId!.replace(/[/\\?%*:|"<> ]/g, '_')
      )

      fullCapsulePath = fullCached;
      thumbCapsulePath = thumbCached;
    }
    
    if (heroPath !== system.heroPath) {
      const heroCached = await RestController.cacheHero(
        heroPath!,
        $romEditingId!.replace(/[/\\?%*:|"<> ]/g, '_')
      );
      
      heroPath = heroCached;
    }
    
    $romMetadata[$romEditingId!] = {
      title: title,
      fullCapsulePath: fullCapsulePath,
      thumbCapsulePath: thumbCapsulePath,
      heroPath: heroPath,
      sgdbId: sgdbId,
      igdbId: igdbId,
      metadata: igdbMetadata,
      isFavorite: isFavorite
    }

    $romMetadata = { ...$romMetadata };

    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }

  function closeEnd() {
    $showEditRomModal = false;
    $editIsPostUpload = false;
    $romEditingId = null;
  }

  async function refreshMetadata() {
    igdbMetadata = await RomController.refreshMetadata(igdbId);
  }

  function openSGDBSearch() {
    $sgdbSearchTitle = title ?? rom.title;
    $sgdbSearchOnSelect = (id: string) => {
      sgdbId = id;
    }
    $showSearchSGDBModal = true;
  }

  function openIGDBSearch() {
    $igdbSearchRomTitle = title ?? rom.title;
    $igdbSearchRomOnSelect = (id: string) => {
      igdbId = id;
    }
    $igdbSearchRomPlatformId = system.igdbPlatformId;
    $showSearchIGDBRomModal = true;
  }
  
  function editCapsule() {
    $changeGridsSearchId = $romMetadata[$romEditingId!].sgdbId;
    $changeGridsType = "Capsule";
    $changeGridsOnSelect = (fullCapsule?: string, thumbCapsule?: string) => {
      fullCapsulePath = fullCapsule!;
      thumbCapsulePath = thumbCapsule!;
    };
    $changeGridsId = $romEditingId!;
    $showChangeGridsModal = true;
  }

  function editHero() {
    $changeGridsSearchId = $romMetadata[$romEditingId!].sgdbId;
    $changeGridsType = "Hero";
    $changeGridsOnSelect = (fullCapsule?: string, thumbCapsule?: string, hero?: string) => {
      heroPath = hero!;
    };
    $changeGridsId = $romEditingId!;
    $showChangeGridsModal = true;
  }
</script>

<ModalBody
  headline={$editIsPostUpload ? "Confirm Details" : "Edit ROM"}
  open={open}
  canClose={!$editIsPostUpload}
  onclose={closeEnd}
>
  <div class="content">
    {#if saving}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Saving Changes...</div>
      </div>
    {:else}
      <TextField name="Title" bind:value={title} />
      <TextField
        name="SGDB Id"
        bind:value={sgdbId}
        trailingIcon={DatabaseSearch}
        ontrailingClick={openSGDBSearch}
      />
      <TextField
        name="IGDB Id"
        bind:value={igdbId}
        trailingIcon={DatabaseSearch}
        ontrailingClick={openIGDBSearch}
      />
      <label>
        <div class="m3-font-title-medium">Favorite:</div>
        <Checkbox bind:checked={isFavorite} />
      </label>
      <div class="actions" style:--m3-button-shape="var(--m3-util-rounding-small)">
        <Button type="filled" extraOptions={{ style: "flex-grow: 1" }} onclick={editCapsule}>
          Edit Cover
        </Button>
        <Button type="filled" extraOptions={{ style: "flex-grow: 1" }} onclick={editHero}>
          Edit Banner
        </Button>
      </div>
      <div class="actions" style:--m3-button-shape="var(--m3-util-rounding-small)">
        <Button type="filled" extraOptions={{ style: "flex-grow: 1" }} iconType="left" onclick={refreshMetadata}>
          <Icon icon={Cached} />
          Metadata
        </Button>
      </div>
    {/if}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={onCancel}>Cancel</Button>
      <Button type="tonal" onclick={onSave} disabled={saving}>Save</Button>
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

  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .actions {
    width: 100%;

    display: flex;
    justify-content: space-between;

    gap: 0.5rem;
  }

  .actions > :global(button) {
    width: calc(50% - 0.25rem);
  }
</style>
