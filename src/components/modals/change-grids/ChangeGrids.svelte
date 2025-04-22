<script lang="ts">
  import { Icon } from "@component-utils";
  import ModalBody from "@component-utils/ModalBody.svelte";
  import { RestController } from "@controllers";
  import { Tune } from "@icons";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { changeGridsId, changeGridsOnSelect, changeGridsSearchId, changeGridsType, selectedNewGrid, showChangeGridsModal } from "@stores/Modals";
  import { showSGDBFiltersSheet } from "@stores/Sheets";
  import GridResults from "./GridResults.svelte";

  let open = $state(true);
  
  let saving = $state(false);

  const canSave = $derived($selectedNewGrid !== null);

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    saving = true;

    if ($changeGridsType === "Capsule") {
      const [fullCapsule, thumbCapsule] = await RestController.cacheCapsule(
        $selectedNewGrid!.url.toString(),
        $selectedNewGrid!.thumb.toString(),
        $changeGridsId!.replace(/[/\\?%*:|"<> ]/g, '_')
      )

      $changeGridsOnSelect(fullCapsule, thumbCapsule);
    } else {
      const hero = await RestController.cacheHero(
        $selectedNewGrid!.url.toString(),
        $changeGridsId!.replace(/[/\\?%*:|"<> ]/g, '_')
      )

      $changeGridsOnSelect(undefined, undefined, hero);
    }

    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }

  function closeEnd() {
    $showChangeGridsModal = false;
    $changeGridsType = "Capsule";
    $changeGridsSearchId = null;
    $selectedNewGrid = null;
    $changeGridsId = null;
    $changeGridsOnSelect = () => {};
  }
</script>

<ModalBody
  headline={`Change ${$changeGridsType === "Capsule" ? "Cover" : "Banner"} Art`}
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
        <LoadingSpinner /> <div class="font-headline-small">Applying Grid...</div>
      </div>
    {:else}
      <GridResults sgdbId={$changeGridsSearchId!} />
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
