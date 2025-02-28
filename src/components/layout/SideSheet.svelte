<script lang="ts">
  import { easeEmphasizedAccel, easeEmphasizedDecel } from "@utils";
  import { createEventDispatcher, onDestroy, onMount } from "svelte";
  import { tweened } from "svelte/motion";
  import type { Unsubscriber } from "svelte/store";

  let heightUnsub: Unsubscriber;

  let dialogElement: HTMLDialogElement;

  export let width = 400;
  export let padding = "1rem";

  let leaving = false;
  let hasMounted = false;

  const dispatch = createEventDispatcher();

  const actualWidth = tweened(0, {
    duration: 400,
    easing: easeEmphasizedDecel
  });

  /**
   * Handles opening the sheet.
   */
  const open = (node: HTMLDialogElement) => {
    node.inert = true;
    node.showModal();
    node.inert = false;
  }

  const close = () => {
    leaving = true;
    actualWidth.set(0, {
      duration: 200,
      easing: easeEmphasizedAccel
    });
  }

  onMount(() => {
    $actualWidth = width;
    heightUnsub = actualWidth.subscribe((width) => {
      if (width === 0 && hasMounted) dispatch("close");
    });
    hasMounted = true;
  });

  onDestroy(() => {
    if (heightUnsub) heightUnsub();
  });
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
  class="m3-container"
  class:leaving
  style:width="{$actualWidth}px"
  use:open
  on:cancel|preventDefault={close}
  on:click|self={close}
  bind:this={dialogElement}
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    style:padding={padding}
    class="sheet-container"
    on:click|stopImmediatePropagation
  >
    <slot />
  </div>
</dialog>

<style>
  :root {
    --m3-bottom-sheet-shape: var(--m3-util-rounding-large);
  }

  .m3-container {
    min-height: calc(100%);

    background-color: rgb(var(--m3-scheme-surface-container-low));
    color: rgb(var(--m3-scheme-on-surface));

    border-radius: var(--m3-bottom-sheet-shape) 0 0 var(--m3-bottom-sheet-shape);
    border: none;

    padding: 0;

    margin-right: 0;
    margin-top: 0;
    margin-bottom: 0;
  }
  dialog::backdrop {
    background-color: rgb(var(--m3-scheme-scrim) / 0.5);
    animation: backdrop 400ms;
  }
  dialog:global(.leaving)::backdrop {
    background-color: transparent;
    animation: backdropReverse 200ms;
  }
  .sheet-container {
    width: 100%;
    height: 100vh;
    
    overflow: hidden;
  }

  @keyframes backdrop {
    0% {
      background-color: transparent;
    }
    100% {
      background-color: rgb(var(--m3-scheme-scrim) / 0.5);
    }
  }
  @keyframes backdropReverse {
    0% {
      background-color: rgb(var(--m3-scheme-scrim) / 0.5);
    }
    100% {
      background-color: transparent;
    }
  }
</style>