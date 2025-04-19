<script lang="ts">
  import { Icon } from "@component-utils";
  import ModalBody from "@component-utils/ModalBody.svelte";
  import { RestController } from "@controllers";
  import { Tune } from "@icons";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { changeCoverId, changeCoverOnSelect, changeCoverSearchId, selectedNewCoverGrid, showChangeCoverModal } from "@stores/Modals";
  import { showSGDBFiltersSheet } from "@stores/Sheets";
  import GridResults from "./GridResults.svelte";

  let open = $state(true);
  
  let saving = $state(false);

  const canSave = $derived($selectedNewCoverGrid !== null);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    saving = true;

    const [cover, thumb] = await RestController.cacheCover(
      $selectedNewCoverGrid!.url.toString(),
      $selectedNewCoverGrid!.thumb.toString(),
      $changeCoverId!.replace(/[/\\?%*:|"<> ]/g, '_')
    )

    $changeCoverOnSelect(cover, thumb);

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
    $changeCoverSearchId = null;
    $selectedNewCoverGrid = null;
    $changeCoverId = null;
    $changeCoverOnSelect = () => {};
  }
</script>

<ModalBody
  headline={"Change Cover Art"}
  open={open}
  oncloseend={closeEnd}
  maxWidth="50rem"
  extraWrapperOptions={{
    style: "width: calc(100% - 2rem);"
  }}
  extraOptions={{
    style: "width: 100%;"
  }}
>
  {#snippet headlineAction()}
    <div>
      <Button type="text" iconType="full" onclick={() => $showSGDBFiltersSheet = true}>
        <Icon icon={Tune} />
      </Button>
    </div>
  {/snippet}
  <div class="content">
    {#if saving}
      <div class="loading-container">
        <LoadingSpinner /> <div class="font-headline-small">Applying Cover...</div>
      </div>
    {:else}
      <GridResults sgdbId={$changeCoverSearchId!} />
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

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
