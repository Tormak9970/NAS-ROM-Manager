<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button } from "@interactables";
  import { LoadingSpinner } from "@layout";
  import { showChangelogModal } from "@stores/Modals";
  import MarkdownIt from "markdown-it";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  
  const mdIt = new MarkdownIt({
    html: true,
    linkify: true
  });

  let open = $state(true);
  let changelogData = $state<string>("");
  let loading = $state(true);

  function closeEnd() {
    $showChangelogModal = false;
  }

  onMount(async () => {
    const data = await fetch("https://raw.githubusercontent.com/Tormak9970/NAS-ROM-Manager/refs/heads/main/CHANGELOG.md");
    changelogData = (await data.text()).slice(85);

    loading = false;
  });
</script>

<ModalBody
  headline="NRM Changelog"
  maxWidth="40rem"
  open={open}
  onclose={closeEnd}
>
  <div class="content">
    {#if loading}
      <div class="loading-container">
        <LoadingSpinner />
      </div>
    {:else}
      <div class="changelog styled-scrollbar" in:fade={{ duration: 300 }}>
        <div>
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html mdIt.render(changelogData ?? "No update details found")}
        </div>
      </div>
    {/if}
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
