<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button } from "@interactables";
  import { controlledModalCancel, controlledModalCancelText, controlledModalConfirm, controlledModalConfirmText, controlledModalIsWarning, controlledModalMessage, controlledModalTitle, showControlledModal } from "@stores/Modals";

  let open = true;

  function onCloseEnd() {
    $showControlledModal = false;
    $controlledModalTitle = "";
    $controlledModalMessage = "";
    $controlledModalConfirmText = "";
    $controlledModalConfirm = async () => {};
    $controlledModalCancelText = "";
    $controlledModalCancel = async () => {};
    $controlledModalIsWarning = false;
  }

  /**
   * Function to run on confirmation.
   */
  async function onConfirm(): Promise<void> {
    await $controlledModalConfirm();
    open = false;
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    await $controlledModalCancel();
    open = false;
  }
</script>

<ModalBody
  headline={$controlledModalTitle}
  open={open}
  canClose={false}
  oncloseend={onCloseEnd}
>
  <div class="content">
    {$controlledModalMessage}
  </div>
  {#snippet buttons()}
    <div>
      {#if $controlledModalCancelText !== ""}
        <Button type="tonal" onclick={onCancel}>{$controlledModalCancelText}</Button>
      {/if}
      {#if $controlledModalConfirmText !== ""}
        <Button type="tonal" warning={$controlledModalIsWarning} onclick={onConfirm}>{$controlledModalConfirmText}</Button>
      {/if}
    </div>
  {/snippet}
</ModalBody>

<style>
  .content {
    width: 100%;
  }
</style>
