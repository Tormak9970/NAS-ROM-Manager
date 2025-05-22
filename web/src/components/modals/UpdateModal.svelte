<script lang="ts">
  import { Icon, ModalBody } from "@component-utils";
  import { Celebration, ForwardArrow } from "@icons";
  import { Button } from "@interactables";
  import { showUpdateModal } from "@stores/Modals";
  import { updateManifest } from "@stores/Update";
  import MarkdownIt from "markdown-it";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  
  const mdIt = new MarkdownIt({
    html: true,
    linkify: true
  });

  let open = $state(true);

  let formattedDate = $state<string>("")

  function closeEnd() {
    $showUpdateModal = false;
  }

  onMount(() => {
    const dateString = $updateManifest!.releaseDate;

    const lang = "en-US";
    const formatter = new Intl.DateTimeFormat(lang, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });

    const date = new Date(dateString);
    formattedDate = formatter.format(date);
  })
</script>

<ModalBody
  icon={Celebration}
  headline="Update Available!"
  open={open}
  onclose={closeEnd}
>
  <div class="content">
    <div class="update-info">
      <div class="field">
        <div class="label">Version:</div>
        <div class="version">
          <div>v{$updateManifest?.currentVersion}</div>
          <Icon icon={ForwardArrow} />
          <div>{$updateManifest?.version}</div>
        </div>
      </div>
      <div class="field">
        <div class="label">Released On:</div>
        <div>{formattedDate}</div>
      </div>
    </div>
    <div class="changelog styled-scrollbar" in:fade={{ duration: 300 }}>
      <div>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html mdIt.render($updateManifest?.body ?? "No update details found")}
      </div>
    </div>
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={() => open = false}>Acknowledge</Button>
    </div>
  {/snippet}
</ModalBody>

<style>
  .content {
    width: 100%;
    height: 390px;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    
    font-size: 1rem;
  }

  .update-info .field {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .field .label {
    font-weight: bold;
  }

  .version {
    display: flex;
    align-items: center;

    gap: 0.25rem;
  }

  .changelog {
    width: 100%;
    height: 100%;

    border-radius: 10px;
    background-color: rgb(var(--m3-scheme-surface-container-lowest));
    overflow-x: hidden;
    overflow-y: scroll;
  }
  .changelog > div {
    margin-bottom: 1rem;
  }

  :global(.changelog > div p) {
    margin-left: 0.5rem;
    margin-bottom: 0.1rem;
    font-weight: bold;
  }

  :global(.changelog > div ul) {
    margin: 0.3rem 0;
    padding-left: 2rem;
  }

  :global(.changelog > div li) {
    margin-bottom: 0.1rem;
  }

  :global(.changelog > div a) {
    color: rgb(var(--m3-scheme-primary));
  }
  :global(.changelog > div a:hover),
  :global(.changelog > div a:active) {
    color: rgb(var(--m3-scheme-on-primary-container));
  }
</style>
