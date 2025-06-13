<script lang="ts">
  import { Icon } from "@component-utils";
  import { focusOutside, shortcuts, type ShortcutOptions } from "@directives";
  import { Close, Search, UnfoldMore } from "@icons";
  import { onMount, tick } from 'svelte';
  import type { FormEventHandler } from 'svelte/elements';
  import { fly } from 'svelte/transition';

  type Props = {
    name: string;
    hideLabel?: boolean;
    options?: SelectItem[];
    placeholder?: string;
    disabled?: boolean;
    clearable?: boolean;
    onSelect?: (option: SelectItem | undefined) => void;
    value?: string;
  }

  let {
    name,
    hideLabel = false,
    options = [],
    placeholder = '',
    disabled = false,
    clearable = false,
    onSelect = () => {},
    value = $bindable("")
  }: Props = $props();

  const id = crypto.randomUUID();
  let isOpen = $state(false);
  let isActive = $state(false);
  let selectedOption: SelectItem | undefined = $derived(value ? options.find((option) => option.value === value) : undefined);
  
  // svelte-ignore state_referenced_locally
  let searchQuery = $state(selectedOption?.label || '');
  let selectedIndex: number | undefined = $state();
  let optionRefs: HTMLElement[] = $state([]);
  let input = $state<HTMLInputElement>();
  let bounds: DOMRect | undefined = $state();

  $effect(() => {
    if (selectedIndex) {
      value = options[selectedIndex].value
    }
  });

  $effect(() => {
    if (value) {
      searchQuery = selectedOption?.label || '';
    }
  });

  const inputId = `combobox-${id}`;
  const listboxId = `listbox-${id}`;
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

  const activate = () => {
    isActive = true;
    searchQuery = '';
    openDropdown();
  };

  const deactivate = () => {
    searchQuery = selectedOption ? selectedOption.label : '';
    isActive = false;
    closeDropdown();
  };

  const openDropdown = () => {
    isOpen = true;
    bounds = getInputPosition();
  };

  const closeDropdown = () => {
    isOpen = false;
    selectedIndex = undefined;
  };

  const incrementSelectedIndex = async (increment: number) => {
    if (filteredOptions.length === 0) {
      selectedIndex = 0;
    } else if (selectedIndex === undefined) {
      selectedIndex = increment === 1 ? 0 : filteredOptions.length - 1;
    } else {
      selectedIndex = (selectedIndex + increment + filteredOptions.length) % filteredOptions.length;
    }
    await tick();
    optionRefs[selectedIndex]?.scrollIntoView({ block: 'nearest' });
  };

  const onInput: FormEventHandler<HTMLInputElement> = (event) => {
    openDropdown();
    searchQuery = event.currentTarget.value;
    selectedIndex = undefined;
    optionRefs[0]?.scrollIntoView({ block: 'nearest' });
  };

  let handleSelect = (option: SelectItem) => {
    value = option.value;
    searchQuery = option.label;
    onSelect(option);
    closeDropdown();
  };

  const onClear = () => {
    input?.focus();
    selectedIndex = undefined;
    value = "";
    searchQuery = '';
    onSelect(selectedOption);
  };

  const calculatePosition = (boundary: DOMRect | undefined) => {
    const visualViewport = window.visualViewport;

    if (!boundary) {
      return;
    }

    const left = boundary.left + (visualViewport?.offsetLeft || 0);
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

  const maxHeight = (size: number) => `min(${size}px,18rem)`;

  const onPositionChange = () => {
    if (!isOpen) {
      return;
    }
    bounds = getInputPosition();
  };

  const getComboboxDirection = (
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

  let filteredOptions = $derived(options.filter((option) => option.label.toLowerCase().includes(searchQuery.toLowerCase())));
  let position = $derived(calculatePosition(bounds));
  let dropdownDirection: 'bottom' | 'top' = $derived(getComboboxDirection(bounds, visualViewport));

  const inputShortcuts: ShortcutOptions[] = [
    {
      shortcut: { key: 'ArrowUp' },
      onShortcut: () => {
        openDropdown();
        void incrementSelectedIndex(-1);
      },
    },
    {
      shortcut: { key: 'ArrowDown' },
      onShortcut: () => {
        openDropdown();
        void incrementSelectedIndex(1);
      },
    },
    {
      shortcut: { key: 'ArrowDown', alt: true },
      onShortcut: () => {
        openDropdown();
      },
    },
    {
      shortcut: { key: 'Enter' },
      onShortcut: () => {
        if (selectedIndex !== undefined && filteredOptions.length > 0) {
          handleSelect(filteredOptions[selectedIndex]);
        }
        closeDropdown();
      },
    },
    {
      shortcut: { key: 'Escape' },
      onShortcut: (event) => {
        event.stopPropagation();
        closeDropdown();
      },
    },
  ];
</script>

<svelte:window onresize={onPositionChange} />
<div
  class="select-container"
  use:focusOutside={{ onFocusOut: deactivate }}
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
  <div class="m3-container">
    {#if isActive}
      <div class="leading">
        <Icon icon={Search} width="1rem" height="1rem" />
      </div>
    {/if}

    <input
      class="m3-font-body-medium"
      {placeholder}
      aria-activedescendant={selectedIndex || selectedIndex === 0 ? `${listboxId}-${selectedIndex}` : ''}
      aria-autocomplete="list"
      aria-controls={listboxId}
      aria-expanded={isOpen}
      autocomplete="off"
      bind:this={input}
      class:padded-start={isActive}
      class:flat-bottom={isOpen && dropdownDirection === 'bottom'}
      class:flat-top={isOpen && dropdownDirection === 'top'}
      class:cursor-pointer={!isActive}
      id={inputId}
      onfocus={activate}
      oninput={onInput}
      role="combobox"
      type="text"
      value={searchQuery}
      disabled={disabled}
      use:shortcuts={inputShortcuts}
    />
    {#if !hideLabel}
      <label class="m3-font-body-large" for={inputId}>{name}</label>
    {/if}

    {#if selectedOption && clearable}
      <button onclick={onClear} class="trailing" disabled={disabled}>
        <Icon icon={Close} />
      </button>
    {:else if !isOpen}
      <div class="trailing" style:pointer-events="none">
        <Icon icon={UnfoldMore} />
      </div>
    {/if}
  </div>

  {#if isOpen}
    <ul
      role="listbox"
      id={listboxId}
      transition:fly={{ y: 25, duration: 250 }}
      class="options-list"
      class:rounded-bottom={dropdownDirection === 'bottom'}
      class:rounded-top={dropdownDirection === 'top'}
      class:shadow={dropdownDirection === 'bottom'}
      class:border={isOpen}
      style:top={position?.top}
      style:bottom={position?.bottom}
      style:left={position?.left}
      style:width={position?.width}
      style:max-height={position?.maxHeight}
      tabindex="-1"
    >
      {#if filteredOptions.length === 0}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <li
          role="option"
          aria-selected={selectedIndex === 0}
          aria-disabled={true}
          class="option"
          id={`${listboxId}-${0}`}
          onclick={closeDropdown}
        >
          No Results
        </li>
      {/if}
      {#each filteredOptions as option, index (option.label)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <li
          aria-selected={index === selectedIndex}
          bind:this={optionRefs[index]}
          class="option"
          id={`${listboxId}-${index}`}
          onclick={() => handleSelect(option)}
          role="option"
        >
          {option.label}
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  :root {
    --m3-select-outlined-shape: var(--m3-util-rounding-medium);
  }

  .select-container {
    position: relative;
    width: 100%;
  }

  .m3-container {
    position: relative;

    display: flex;
    flex-direction: column-reverse;
  }
  input {
    width: 100%;
    height: 2.9rem;
    border: none;
    outline: none;
    padding: 0.75rem;

    color: rgb(var(--m3-scheme-on-surface));
    
    border-radius: var(--m3-select-outlined-shape);
    background-color: rgb(var(--m3-util-background, var(--m3-scheme-surface-variant)));

    transition-property: all;
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    transition-duration: 300ms; 
  }

  .padded-start {
    padding-left: 2rem;
  }

  .leading {
    display: flex;
    align-items: center;
    
    position: absolute;
    left: 0.6rem;
    bottom: 0.3rem;
    
    height: 2.25rem;
    width: 2.25rem;
  }

  .cursor-pointer {
    cursor: pointer;
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

  input:not(:disabled):hover ~ label,
  input:not(:disabled):focus ~ label {
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

  input:read-only {
    caret-color: transparent;
  }

  input:disabled {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
    background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
  }
  button:disabled {
    pointer-events: none;
  }
  button.trailing:disabled :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.18);
  }

  .options-list {
    box-sizing: border-box;
    list-style: none;
    padding: 0;
    margin: 0;

    position: fixed;
    text-align: left;
    font-size: 0.875rem;
    width: 100%;
    overflow-y: auto;
    background-color: rgb(var(--m3-scheme-surface-container-high));
    z-index: 10000;
  }
  .border {
    border: 1px solid rgb(var(--m3-util-background, var(--m3-scheme-surface-variant)));
  }


  .rounded-bottom {
    border-bottom-left-radius: var(--m3-select-outlined-shape);
    border-bottom-right-radius: var(--m3-select-outlined-shape);
  }
  .flat-bottom {
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }
  .rounded-top {
    border-top-left-radius: var(--m3-select-outlined-shape);
    border-top-right-radius: var(--m3-select-outlined-shape);
  }
  .flat-top {
    border-top-left-radius: 0;
    border-top-right-radius: 0;
  }
  .shadow {
    box-shadow: var(--m3-util-elevation-1);
  }


  .option {
    text-align: left;
    width: 100%;
    padding: 0.5rem 1rem;
    transition: all 0.2s ease-in-out;
    cursor: pointer;
    word-wrap: break-word;
  }

  .option:hover,
  .option[aria-selected="true"] {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
  }
</style>