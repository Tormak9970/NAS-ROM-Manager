<script lang="ts">
  import { Icon } from "@component-utils";
  import ModalBody from "@component-utils/ModalBody.svelte";
  import { RestController } from "@controllers";
  import { Tune } from "@icons";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { changeCoverId, selectedNewCoverGrid, showChangeCoverModal } from "@stores/Modals";
  import { showSGDBFiltersSheet } from "@stores/Sheets";
  import { romMetadata } from "@stores/State";
  import GridResults from "./GridResults.svelte";

  let open = $state(true);
  
  let saving = $state(false);

  let metadata = $romMetadata[$changeCoverId!];
  let canSave = $derived($selectedNewCoverGrid !== null);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    saving = true;

    const [cover, thumb] = await RestController.cacheCover(
      $selectedNewCoverGrid!.url.toString(),
      $selectedNewCoverGrid!.thumb.toString(),
      $changeCoverId!
    )

    $romMetadata[$changeCoverId!].coverPath = cover;
    $romMetadata[$changeCoverId!].thumbPath = thumb;

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
    $showChangeCoverModal = false;
    $changeCoverId = null;
    $selectedNewCoverGrid = null;
  }
</script>

<ModalBody
  headline={"Change Cover Art"}
  open={open}
  on:closeEnd={closeEnd}
  extraWrapperOptions={{
    style: "width: calc(100% - 2rem); max-width: 40rem;"
  }}
  extraOptions={{
    style: "width: 100%;"
  }}
>
  <div slot="headline-action">
    <Button type="text" iconType="full" on:click={() => $showSGDBFiltersSheet = true}>
      <Icon icon={Tune} />
    </Button>
  </div>
  <div class="content">
    {#if saving}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Applying Cover...</div>
      </div>
    {:else}
      <GridResults sgdbId={metadata.sgdbId} />
    {/if}
  </div>
  <div slot="buttons">
    <Button type="tonal" on:click={onCancel}>Cancel</Button>
    <Button type="tonal" on:click={onSave} disabled={!canSave}>Save</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
