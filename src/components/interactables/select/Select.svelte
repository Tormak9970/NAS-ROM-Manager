<script lang="ts" module>
  export type ComboBoxOption = {
    id?: string;
    label: string;
    value: string;
  };
</script>

<script lang="ts">
  import { Icon } from "@component-utils";
  import { focusOutside, shortcuts } from "@directives";
  import { Search, UnfoldMore } from "@icons";
  import { onMount, tick } from 'svelte';
  import type { FormEventHandler } from 'svelte/elements';
  import { fly } from 'svelte/transition';
  type Props = {
    label: string;
    hideLabel?: boolean;
    options?: ComboBoxOption[];
    selectedOption?: ComboBoxOption | undefined;
    placeholder?: string;
    /**
     * whether creating new items is allowed.
     */
    allowCreate?: boolean;
    /**
     * select first matching option on enter key.
     */
    defaultFirstOption?: boolean;
    onSelect?: (option: ComboBoxOption | undefined) => void;
    value?: string;
  }

  let {
    label,
    hideLabel = false,
    options = [],
    selectedOption = $bindable(),
    placeholder = '',
    allowCreate = false,
    defaultFirstOption = false,
    onSelect = () => {},
    value = $bindable("")
  }: Props = $props();

  /**
   * Unique identifier for the combobox.
   */
  const id = crypto.randomUUID();
  /**
   * Indicates whether or not the dropdown autocomplete list should be visible.
   */
  let isOpen = $state(false);
  /**
   * Keeps track of whether the combobox is actively being used.
   */
  let isActive = $state(false);
  let searchQuery = $state(selectedOption?.label || '');
  let selectedIndex: number | undefined = $state();
  let optionRefs: HTMLElement[] = $state([]);
  let input = $state<HTMLInputElement>();
  let bounds: DOMRect | undefined = $state();

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

  onMount(() => {
    if (!input) {
      return;
    }
    observer.observe(input);
    // TODO: this won't work with my setup. Need to find a different solution
    const scrollableAncestor = input?.closest('.overflow-y-auto, .overflow-y-scroll');
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
    selectedIndex = defaultFirstOption ? 0 : undefined;
    optionRefs[0]?.scrollIntoView({ block: 'nearest' });
  };

  let handleSelect = (option: ComboBoxOption) => {
    selectedOption = option;
    searchQuery = option.label;
    onSelect(option);
    closeDropdown();
  };

  const onClear = () => {
    input?.focus();
    selectedIndex = undefined;
    selectedOption = undefined;
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

  let filteredOptions = $derived.by(() => {
    const _options = options.filter((option) => option.label.toLowerCase().includes(searchQuery.toLowerCase()));

    if (allowCreate && searchQuery !== '' && _options.filter((option) => option.label === searchQuery).length === 0) {
      _options.unshift({ label: searchQuery, value: searchQuery });
    }

    return _options;
  });
  let position = $derived(calculatePosition(bounds));
  let dropdownDirection: 'bottom' | 'top' = $derived(getComboboxDirection(bounds, visualViewport));
</script>

<svelte:window onresize={onPositionChange} />
<label class="immich-form-label" class:sr-only={hideLabel} for={inputId}>{label}</label>
<div
  class="relative w-full dark:text-gray-300 text-gray-700 text-base"
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
  <div>
    {#if isActive}
      <div class="absolute inset-y-0 start-0 flex items-center ps-3">
        <div class="dark:text-immich-dark-fg/75">
          <Icon icon={Search} />
        </div>
      </div>
    {/if}

    <input
      {placeholder}
      aria-activedescendant={selectedIndex || selectedIndex === 0 ? `${listboxId}-${selectedIndex}` : ''}
      aria-autocomplete="list"
      aria-controls={listboxId}
      aria-expanded={isOpen}
      autocomplete="off"
      bind:this={input}
      class:!ps-8={isActive}
      class:!rounded-b-none={isOpen && dropdownDirection === 'bottom'}
      class:!rounded-t-none={isOpen && dropdownDirection === 'top'}
      class:cursor-pointer={!isActive}
      class="immich-form-input text-sm w-full !pe-12 transition-all"
      id={inputId}
      onfocus={activate}
      oninput={onInput}
      role="combobox"
      type="text"
      value={searchQuery}
      use:shortcuts={[
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
      ]}
    />

    <div
      class="absolute end-0 top-0 h-full flex px-4 justify-center items-center content-between"
      class:pe-2={selectedOption}
      class:pointer-events-none={!selectedOption}
    >
      {#if selectedOption}
        <!-- <CircleIconButton onclick={onClear} title={$t('clear_value')} icon={mdiClose} size="16" padding="2" /> -->
      {:else if !isOpen}
        <Icon icon={UnfoldMore} />
      {/if}
    </div>
  </div>

  <ul
    role="listbox"
    id={listboxId}
    transition:fly={{ duration: 250 }}
    class="options-list"
    class:rounded-b-xl={dropdownDirection === 'bottom'}
    class:rounded-t-xl={dropdownDirection === 'top'}
    class:shadow={dropdownDirection === 'bottom'}
    class:border={isOpen}
    style:top={position?.top}
    style:bottom={position?.bottom}
    style:left={position?.left}
    style:width={position?.width}
    style:max-height={position?.maxHeight}
    tabindex="-1"
  >
    {#if isOpen}
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
          {allowCreate ? searchQuery : "No Results"}
        </li>
      {/if}
      {#each filteredOptions as option, index (option.id || option.label)}
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
    {/if}
  </ul>
</div>

<style>
  .options-list {
    position: fixed;
    text-align: left;
    font-size: 0.875rem; /* text-sm in Tailwind */
    width: 100%;
    overflow-y: auto;
    background-color: #2d3748; /* bg-gray-800 */
    border-color: #1a202c; /* border-gray-900 */
    z-index: 10000;
  }
  .option {
    text-align: left;
    width: 100%;
    padding-left: 1rem; /* 4 in Tailwind */
    padding-right: 1rem; /* 4 in Tailwind */
    padding-top: 0.5rem; /* 2 in Tailwind */
    padding-bottom: 0.5rem; /* 2 in Tailwind */
    transition: all 0.2s ease-in-out;
    cursor: pointer;
    word-wrap: break-word; /* break-words in Tailwind */
  }

  .option:hover {
    background-color: #374151; /* bg-gray-700 */
  }

  .option[aria-selected="true"] {
    background-color: #374151; /* bg-gray-700 */
  }
</style>