<script lang="ts">
  import { onMount } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  import { Icon } from "@component-utils";
  import { focusOutside, shortcuts } from "@directives";
  import { CalendarToday } from "@icons";
  import { fly } from "svelte/transition";
  import DatePicker from "./date-picker/DatePicker.svelte";
  
  type Props = {
    name: string;
    date?: string;
    required?: boolean;
    disabled?: boolean;
    readonly?: boolean;
    extraOptions?: HTMLInputAttributes;
    validate?: (date: string) => boolean;
  }

  let {
    name,
    date = $bindable(""),
    required = false,
    disabled = false,
    readonly = false,
    extraOptions = {},
    validate = (date: string) => { return true; }
  }: Props = $props();

  const id = crypto.randomUUID();
  let isOpen = $state(false);
  let input = $state<HTMLInputElement>();
  let bounds: DOMRect | undefined = $state();
  
  let error = $derived(!validate(date));

  /**
   * Buffer distance between the dropdown and top/bottom of the viewport.
   */
  const dropdownOffset = 15;
  /**
   * Minimum space required for the dropdown to be displayed at the bottom of the input.
   */
  const bottomBreakpoint = 225;
  const observer = new IntersectionObserver(
    (entries) => {
      const inputEntry = entries[0];
      if (inputEntry.intersectionRatio < 1) {
        isOpen = false;
      }
    },
    { threshold: 0.5 },
  );

  function getScrollParent(node: any) {
    if (node == null) {
      return null;
    }

    if (node.scrollHeight > node.clientHeight) {
      return node;
    } else {
      return getScrollParent(node.parentNode);
    }
  }

  onMount(() => {
    if (!input) {
      return;
    }
    observer.observe(input);
    
    const scrollableAncestor = getScrollParent(input);
    scrollableAncestor?.addEventListener('scroll', onPositionChange);
    window.visualViewport?.addEventListener('resize', onPositionChange);
    window.visualViewport?.addEventListener('scroll', onPositionChange);

    return () => {
      observer.disconnect();
      scrollableAncestor?.removeEventListener('scroll', onPositionChange);
      window.visualViewport?.removeEventListener('resize', onPositionChange);
      window.visualViewport?.removeEventListener('scroll', onPositionChange);
    };
  });

  const openDropdown = () => {
    isOpen = true;
    bounds = getInputPosition();
  };

  const closeDropdown = () => {
    isOpen = false;
  };

  const calculatePosition = (boundary: DOMRect | undefined) => {
    const visualViewport = window.visualViewport;

    if (!boundary) {
      return;
    }

    const left = (boundary.left + (visualViewport?.offsetLeft || 0)) + boundary.width - 328;
    const offsetTop = visualViewport?.offsetTop || 0;

    if (dropdownDirection === 'top') {
      return {
        bottom: `${window.innerHeight - boundary.top - offsetTop}px`,
        left: `${left}px`,
        width: `${boundary.width}px`,
        maxHeight: maxHeight(boundary.top - dropdownOffset),
      };
    }

    const viewportHeight = visualViewport?.height || 0;
    const availableHeight = viewportHeight - boundary.bottom;
    return {
      top: `${boundary.bottom + offsetTop}px`,
      left: `${left}px`,
      width: `${boundary.width}px`,
      maxHeight: maxHeight(availableHeight - dropdownOffset),
    };
  };

  const maxHeight = (size: number) => `min(${size}px,26.75rem)`;

  const onPositionChange = () => {
    if (!isOpen) {
      return;
    }
    bounds = getInputPosition();
  };

  const getPickerDirection = (
    boundary: DOMRect | undefined,
    visualViewport: VisualViewport | null,
  ): 'bottom' | 'top' => {
    if (!boundary) {
      return 'bottom';
    }

    const visualHeight = visualViewport?.height || 0;
    const heightBelow = visualHeight - boundary.bottom;
    const heightAbove = boundary.top;

    const isViewportScaled = visualHeight && Math.floor(visualHeight) !== Math.floor(window.innerHeight);

    return heightBelow <= bottomBreakpoint && heightAbove > heightBelow && !isViewportScaled ? 'top' : 'bottom';
  };

  const getInputPosition = () => input?.getBoundingClientRect();

  let position = $derived(calculatePosition(bounds));
  let dropdownDirection: 'bottom' | 'top' = $derived(getPickerDirection(bounds, visualViewport));
</script>

<div
  class="m3-container"
  class:error
  class:disabled
  use:focusOutside={{ onFocusOut: closeDropdown }}
  use:shortcuts={[
    {
      shortcut: { key: 'Escape' },
      onShortcut: (event) => {
        event.stopPropagation();
        closeDropdown();
      },
    },
  ]}
>
  <div class="input-wrapper">
    <input
      type="date"
      class="m3-font-body-large"
      {disabled}
      {required}
      {readonly}
      {id}
      bind:value={date}
      {...extraOptions}
      bind:this={input}
    />
  </div>
  <label class="m3-font-body-large" for={id}>{name}</label>
  <button type="button" class="trailing" {disabled} onclick={() => {
    if (isOpen) {
      closeDropdown();
    } else {
      openDropdown();
    }
  }}>
    <Icon icon={CalendarToday} />
  </button>
  {#if isOpen}
    <div
    class="picker"
    class:rounded-bottom={dropdownDirection === 'bottom'}
    class:rounded-top={dropdownDirection === 'top'}
    class:shadow={dropdownDirection === 'bottom'}
    class:border={isOpen}
    style:top={position?.top}
    style:bottom={position?.bottom}
    style:left={position?.left}
    style:max-height={position?.maxHeight}
    transition:fly={{ y: 25, duration: 250 }}
  >
    <DatePicker
      clearable={!required}
      bind:date
      onclose={closeDropdown}
      onsetDate={(newDate: string) => {
        date = newDate;
      }}
    />
  </div>
  {/if}
</div>

<style>
  :root {
    --m3-datefield-outlined-shape: var(--m3-util-rounding-medium);
  }
  .m3-container {
    position: relative;

    display: flex;
    flex-direction: column-reverse;

    height: 4.25rem;
    width: 100%;
  }
  .input-wrapper {
    width: 100%;
    
    border-radius: var(--m3-datefield-outlined-shape);
    background-color: rgb(var(--m3-util-background, var(--m3-scheme-surface-variant)));
  }
  input {
    width: 100%;
    height: 100%;
    border: none;
    outline: none;
    padding: 0.6rem 0.75rem;

    color: rgb(var(--m3-scheme-on-surface));
    
    background-color: transparent;
    
    clip-path: inset(0 3.5rem 0 0);
  }

  label {
    color: rgb(var(--error, var(--m3-scheme-on-surface-variant)));
    font-weight: bold;

    transition: color 200ms;

    font-size: 0.9rem;
  }
  .m3-container .trailing :global(svg) {
    width: 1.4rem;
    height: 1.4rem;
    color: rgb(var(--m3-scheme-on-surface-variant));
    pointer-events: none;
  }
  
  .trailing {
    position: absolute;
    z-index: 2;

    height: 2.25rem;
    width: 2.25rem;
    right: 0.3rem;
    bottom: 0.3rem;

    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background-color: transparent;
    border-radius: 1.125rem;

    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }

  .input-wrapper:has(> input:not(:disabled):hover) ~ label,
  .input-wrapper:has(input:not(:disabled):focus) ~ label {
    color: rgb(var(--error, var(--m3-scheme-on-surface)));
  }
  @media (hover: hover) {
    button:not(:disabled):hover {
      background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.08);
    }
  }
  button:focus-visible,
  button:active {
    background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.12);
  }

  .error {
    --error: var(--m3-scheme-error);
  }
  .error > .input-wrapper:hover ~ label {
    --error: var(--m3-scheme-on-error-container);
  }
  .error > .input-wrapper {
    background-color: rgb(var(--m3-scheme-tertiary-container));
  }

  input:read-only {
    caret-color: transparent;
  }

  input:disabled {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
  .input-wrapper:has(> input:disabled) {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
  }
  button:disabled {
    pointer-events: none;
  }
  button.trailing:disabled :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.18);
  }

  .picker {
    box-sizing: border-box;
    padding: 0;
    margin: 0;

    position: fixed;
    overflow-y: auto;
    z-index: 10000;
  }
</style>