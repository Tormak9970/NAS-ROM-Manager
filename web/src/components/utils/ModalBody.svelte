<script lang="ts">
  import type { IconifyIcon } from "@iconify/types";
  import { LoadingSpinner } from "@layout";
  import type { Snippet } from "svelte";
  import type { HTMLAttributes, HTMLDialogAttributes } from "svelte/elements";
  import Icon from "./Icon.svelte";

  type Props = {
    extraWrapperOptions?: HTMLDialogAttributes;
    extraOptions?: HTMLAttributes<HTMLDivElement>;
    icon?: IconifyIcon | undefined;
    maxWidth?: string;
    headline?: string;
    open: boolean;
    canClose?: boolean;
    loading?: boolean;
    headless?: boolean;
    onclose?: () => void;
    headlineAction?: Snippet;
    children: Snippet;
    buttons?: Snippet;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    icon = undefined,
    maxWidth = "30rem",
    headline = "",
    open,
    canClose = true,
    loading = false,
    headless = false,
    onclose,
    headlineAction,
    children,
    buttons,
  }: Props = $props();

  let dialog: HTMLDialogElement | undefined = $state();

  /**
   * Handles opening the modal.
   */
  function openModal(node: HTMLDialogElement) {
    node.inert = true;
    node.showModal();
    node.inert = false;
  }

  $effect(() => {
    if (!dialog) return;

    if (open) {
      openModal(dialog);
    } else {
      hideDialog = true;
    }
  });

  let hideDialog = $state(false);

  function onAnimationEnd() {
    if (hideDialog) {
      hideDialog = false;
      dialog!.close();
      onclose?.();
    }
  }

  function onCancel(e: Event) {
    if (canClose) {
      open = false;
    } else {
      e.preventDefault();
    }
  }

  function onClick(e: Event) {
    if (canClose && !(e.target as HTMLElement).closest(".modal-body")) {
      open = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  oncancel={onCancel}
  onclick={onClick}
  onanimationend={onAnimationEnd}
  bind:this={dialog}
  class:hide={hideDialog}
  {...extraWrapperOptions}
  style:max-width={maxWidth}
>
  <div class="modal-body">
    {#if loading}
      <div class="loading-container-modal">
        <LoadingSpinner />
      </div>
    {/if}
    <div class="m3-container">
      <div class="action-container">
        {@render headlineAction?.()}
      </div>
      {#if !headless}
        <div class="header">
          {#if icon}
            <Icon {icon} />
          {/if}
          <p class="headline m3-font-headline-small" class:center={icon}>{headline}</p>
        </div>
      {/if}
      <div class="content m3-font-body-medium" class:headless {...extraOptions}>
        {@render children()}
      </div>
      <div class="buttons">
        {@render buttons?.()}
      </div>
    </div>
  </div>
</dialog>

<style>
  :root {
    --m3-dialog-shape: var(--m3-util-rounding-large);
  }
  dialog {
    border: none;
    padding: 0;
    background-color: transparent;

    display: flex;

    width: calc(100% - 2rem); 
  }
  .modal-body {
    background-color: rgb(var(--m3-scheme-surface-container));
    border-radius: var(--m3-dialog-shape);
    min-width: 17.5rem;
    width: 100%;
    overflow: hidden;

    position: relative;
  }
  .m3-container {
    display: flex;
    flex-direction: column;
    padding: 1.5rem;

    width: 100%;
    height: 100%;

    position: relative;
    z-index: 1;
  }

  .action-container {
    position: absolute;
    right: 1.25rem;
    top: 1.25rem;
  }

  .m3-container > :global(svg) {
    color: rgb(var(--m3-scheme-secondary));
    width: 1.5rem;
    height: 1.5rem;
    margin: 0 auto 1rem auto;
  }
  .header {
    margin-bottom: 1rem;

    display: flex;
    align-items: center;

    gap: 0.5rem;
  }
  .header > :global(svg) {
    color: rgb(var(--m3-scheme-primary));
    width: 1.5rem;
    height: 1.5rem;
  }
  .headline {
    color: rgb(var(--m3-scheme-on-surface));
    margin-top: 0;
    margin-bottom: 0;
  }
  .headline.center {
    text-align: center;
  }
  .content {
    color: rgb(var(--m3-scheme-on-surface-variant));
    margin-bottom: 1.5rem;

    height: calc(100% - 7rem);
    width: 100%;
  }
  .content.headless {
    height: calc(100% - 4rem);
  }

  .loading-container-modal {
    position: absolute;
    z-index: 4;

    background-color: rgb(var(--m3-scheme-scrim) / 0.7);
    color: rgb(var(--m3-scheme-secondary));
    
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .buttons {
    width: 100%;
  }

  .buttons > :global(div) {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
  }
  
  .buttons > :global(div button) {
    flex-grow: 1;
    min-width: calc(50% - 0.5rem);
  }

  dialog {
    position: fixed;
    inset: 0;
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
    transition:
      opacity 200ms,
      visibility 200ms;
  }
  dialog[open] {
    opacity: 1;
    visibility: visible;
    pointer-events: auto;
    animation:
      dialogIn 0.5s cubic-bezier(0.05, 0.7, 0.1, 1),
      opacity 100ms cubic-bezier(0.05, 0.7, 0.1, 1);
  }

  dialog.hide {
    animation: dialogOut 0.3s cubic-bezier(0.05, 0.7, 0.1, 1);
  }

  dialog[open] .headline {
    animation: opacity 150ms;
  }
  dialog[open] .content {
    animation: opacity 200ms;
  }
  dialog[open] .buttons {
    position: relative;
    animation:
      buttonsIn 0.5s cubic-bezier(0.05, 0.7, 0.1, 1),
      opacity 200ms 100ms backwards;
  }
  dialog::backdrop {
    background-color: rgb(var(--m3-scheme-scrim) / 0.3);
    animation: opacity 400ms;
  }
  @keyframes dialogIn {
    0% {
      transform: translateY(-3rem) scaleY(90%);
      clip-path: inset(0 0 100% 0 round var(--m3-dialog-shape));
    }
    100% {
      transform: translateY(0) scaleY(100%);
      clip-path: inset(0 0 0 0 round var(--m3-dialog-shape));
    }
  }
  @keyframes buttonsIn {
    0% {
      bottom: 100%;
    }
    100% {
      bottom: 0;
    }
  }
  @keyframes opacity {
    0% {
      opacity: 0;
    }
    100% {
      opacity: 1;
    }
  }
  
  @keyframes dialogOut {
    0% {
      transform: translateY(0) scaleY(100%);
      clip-path: inset(0 0 0 0 round var(--m3-dialog-shape));
    }
    100% {
      transform: translateY(-3rem) scaleY(90%);
      clip-path: inset(0 0 100% 0 round var(--m3-dialog-shape));
    }
  }
</style>