<script lang="ts">
  import { Icon, ModalBody } from "@component-utils";
  import { Tune } from "@icons";
  import { Button } from "@interactables";
  import { changeCoverId, showChangeCoverModal } from "@stores/Modals";
  import { showSGDBFiltersSheet } from "@stores/Sheets";
  import { romMetadata, roms } from "@stores/State";

  let open = $state(true);

  let rom = $roms[$changeCoverId!];
  let metadata = $romMetadata[$changeCoverId!];

  let coverPath = $state(metadata.coverPath);
  let thumbPath = $state(metadata.thumbPath);

  let canSave = $derived(!!coverPath && !!thumbPath);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    $romMetadata[$changeCoverId!].coverPath = coverPath;
    $romMetadata[$changeCoverId!].thumbPath = thumbPath;

    $romMetadata = { ...$romMetadata };
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }

  function closeEnd() {
    $showChangeCoverModal = false;
    $changeCoverId = null;
  }
</script>

<ModalBody
  headline={"Change Cover Art"}
  open={open}
  on:closeEnd={closeEnd}
  extraOptions={{
    style: "min-width: 300px; max-width: 32rem; width: 100vw;"
  }}
>
  <div slot="headline-action">
    <Button type="text" iconType="full" on:click={() => $showSGDBFiltersSheet = true}>
      <Icon icon={Tune} />
    </Button>
  </div>
  <div class="content">
    <!-- TODO: render grids here -->
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onSave} disabled={!canSave}>Save</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 100%;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
