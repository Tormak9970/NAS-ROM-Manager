<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { LoadingSpinner } from "@layout";
  import { loadingModalMessage, showLoadingModal } from "@stores/Modals";

  let open = $state(true);

  function closeEnd() {
    $showLoadingModal = false;
  }
</script>

<ModalBody
  open={open}
  canClose={false}
  on:closeEnd={closeEnd}
>
  <div class="content">
    <LoadingSpinner /> <div class="font-headline-small">{$loadingModalMessage}</div>
  </div>
</ModalBody>

<style>
  .content {
    width: 100%;

    display: flex;
    align-items: center;
    gap: 20px;

    margin: 0rem 1rem;
    margin-top: 1rem;
  }
</style>
