<script lang="ts">
  import { easeEmphasizedAccel, easeEmphasizedDecel, preventDefault, self } from "@utils";
  import { onMount, type Snippet } from "svelte";
  import { drag } from "svelte-gesture";
  import { Spring, Tween } from "svelte/motion";

  type Props = {
    maxHeight?: number;
    closeThreshold?: number;
    padding?: string;
    onclose?: () => void;
    children: Snippet;
  }

  let {
    maxHeight = 1000,
    closeThreshold = 0.4,
    padding = "0 1rem",
    onclose,
    children,
  }: Props = $props();

  let dialogElement: HTMLDialogElement | undefined = $state();

  let leaving = $state(false);
  let hasMounted = $state(false);

  const actualHeight = new Tween(0, {
    duration: 400,
    easing: easeEmphasizedDecel
  });
  const dragHeight = new Spring(0, {});

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
    actualHeight.set(0, {
      duration: 200,
      easing: easeEmphasizedAccel
    });
  }

  function dragHandler({ detail }: any) {
    const { active, movement: [, my], direction: [, dy], offset: [, oy] } = detail;

    $dragHeight = my;
    
    if ((actualHeight.current - oy) / actualHeight.current <= closeThreshold && !active) {
      close();
      return;
    }

    if (!active) $dragHeight = 0;
  }

  $effect(() => {
    if (actualHeight.current === 0 && actualHeight.target === 0 && hasMounted) {
      onclose?.();
    }
  })

  onMount(() => {
    actualHeight.set(Math.min(maxHeight, dialogElement!.scrollHeight));
    hasMounted = true;
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  class="m3-container"
  class:leaving
  style:max-height="{actualHeight.current - dragHeight.current}px"
  use:open
  oncancel={preventDefault(close)}
  onclick={self(close)}
  bind:this={dialogElement}
  use:drag
  ondrag={dragHandler}
>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    style:padding={padding}
    class="sheet-container"
    onclick={(e) => e.stopImmediatePropagation()}
  >
    <div class="handle-container" >
      <div class="handle"></div>
    </div>
    {@render children()}
  </div>
</dialog>

<style>
  :root {
    --m3-bottom-sheet-shape: var(--m3-util-rounding-large);
  }

  .m3-container {
    margin-bottom: 0;
    width: 100%;
    max-width: 40rem;
    overflow: hidden;
    touch-action: none;

    background-color: rgb(var(--m3-scheme-surface-container-low));
    color: rgb(var(--m3-scheme-on-surface));
    border-radius: var(--m3-bottom-sheet-shape) var(--m3-bottom-sheet-shape) 0 0;
    border: none;
    padding: 0;
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
    touch-action: auto;
  }
  .handle-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 2rem;
    cursor: grab;
  }
  .handle {
    background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.4);
    width: 2rem;
    height: 0.25rem;
    border-radius: 0.25rem;
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