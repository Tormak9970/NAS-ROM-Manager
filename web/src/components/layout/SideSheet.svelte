<script lang="ts">
  import { easeEmphasizedAccel, easeEmphasizedDecel, preventDefault, self } from "@utils";
  import { onMount, type Snippet } from "svelte";
  import { Tween } from "svelte/motion";

  type Props = {
    width?: number;
    padding?: string;
    onclose?: () => void;
    children: Snippet;
  }

  let {
    width = 400,
    padding = "1rem",
    onclose,
    children,
  }: Props = $props();

  let dialogElement: HTMLDialogElement | undefined = $state();

  let leaving = $state(false);
  let hasMounted = $state(false);

  const actualWidth = new Tween(0, {
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

  $effect(() => {
    if (actualWidth.current === 0 && actualWidth.target === 0 && hasMounted) {
      onclose?.();
    }
  });

  onMount(() => {
    actualWidth.set(width);
    hasMounted = true;
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  class="m3-container"
  class:leaving
  style:width="{actualWidth.current}px"
  use:open
  oncancel={preventDefault(close)}
  onclick={self(close)}
  bind:this={dialogElement}
>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    style:padding={padding}
    class="sheet-container"
    onclick={(e) => e.stopImmediatePropagation()}
  >
    {@render children()}
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