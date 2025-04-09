<script lang="ts">
  import { Icon } from "@component-utils";
  import type { HTMLAttributes } from "svelte/elements";
  
  import { ArrowDropDown, ArrowDropUp } from "@icons";
  import { stopPropagation } from "@utils";
  import { onMount } from "svelte";
  import SelectOption from "./SelectOption.svelte";

  const id = crypto.randomUUID();


  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLAttributes<HTMLDivElement>;
    name: string;
    disabled?: boolean;
    value?: string;
    options: SelectItem[];
    open?: boolean;
    maxHeight?: string;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    name,
    disabled = false,
    value = $bindable(""),
    options,
    open = $bindable(false),
    maxHeight = "300px"
  }: Props = $props();

  // @ts-expect-error This will always be defined before its usage.
  let selectElement: HTMLDivElement = $state();
  let menuElement: any = $state();


  function onMouseUp(e: Event) {
    if (e.target !== selectElement) open = false;
  }

  function onClick(e: Event) {
    open = !open;
  }

  function handleClick(newVal: string) {
    value = newVal;
  }

  onMount(() => {
    menuElement.anchorElement = selectElement;

    const shadowRoot = menuElement.shadowRoot;
    const style = document.createElement('style');
    style.innerHTML = `
    .items {
      scrollbar-width: none;
    }
    `
    shadowRoot.insertBefore(style, shadowRoot.firstChild);
  });
  
  $effect(() => {
    menuElement && (menuElement.open = open);
  });
  
  const label = $derived(options.find((option) => option.value === value)?.label);
</script>

<svelte:window onmouseup={onMouseUp} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  onclick={stopPropagation()}
  onmousedown={stopPropagation()}
  style:--md-menu-container-color="rgb(var(--m3-scheme-surface-container))"
  style:--md-menu-item-container-color="rgb(var(--m3-scheme-surface-container))"
  style:--md-menu-item-selected-container-color="rgb(var(--m3-scheme-secondary-container))"
>
  <div
    class="m3-container"
    class:focus={open}
    class:disabled
    class:empty={!label}
    {...extraWrapperOptions}
    onclick={onClick}
    onmouseup={stopPropagation()}
    bind:this={selectElement}
  >
    <div class="select-mimic m3-font-body-large" {id} {...extraOptions}>
      <div class="current-label">{label}</div>
    </div>
    <div class="layer"></div>
    <label class="m3-font-body-large" for={id}>{name}</label>
    <div class="trailing">
      {#if open}
        <Icon icon={ArrowDropUp} />
      {:else}
        <Icon icon={ArrowDropDown} />
      {/if}
    </div>
  </div>
  <md-menu positioning="popover" style:max-height={maxHeight} bind:this={menuElement}>
    {#each options as option}
      <SelectOption value={option.value} selected={value === option.value} width={selectElement?.clientWidth} onclick={handleClick}>{option.label}</SelectOption>
    {/each}
  </md-menu>
</div>

<style>
  :root {
    --m3-textfield-outlined-shape: var(--m3-util-rounding-extra-small);
  }
  .m3-container {
    position: relative;
    align-items: center;
    height: 3rem;
    min-width: 15rem;
    width: 100%;
    
    cursor: pointer;

    display: inline-flex;
  }
  .select-mimic {
    width: calc(100% - 3rem);
    height: 100%;
    padding-left: 0.75rem;
    color: rgb(var(--m3-scheme-on-surface));
    
    display: flex;
    align-items: center;
  }
  .current-label {
    max-width: 260px;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }
  label {
    position: absolute;
    left: 0.75rem;
    top: 0.7rem;
    color: rgb(var(--error, var(--m3-scheme-on-surface-variant)));
    background-color: rgb(var(--m3-util-background, var(--m3-scheme-surface-container-high)));
    padding: 0 0.25rem;
    pointer-events: none;
    transition:
      all 200ms,
      font-size 300ms,
      line-height 300ms,
      letter-spacing 300ms;
  }
  .layer {
    position: absolute;
    inset: 0;
    border: 0.0625rem solid rgb(var(--error, var(--m3-scheme-outline)));
    border-radius: var(--m3-textfield-outlined-shape);
    pointer-events: none;
    transition: all 200ms;
  }
  .m3-container :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
    color: rgb(var(--m3-scheme-on-surface-variant));
    pointer-events: none;
  }
  .trailing {
    padding-left: 0.75rem;
    padding-right: 0.75rem;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background-color: transparent;
  }

  .m3-container.focus label,
  .m3-container:not(.empty) label {
    top: calc(var(--m3-font-body-small-height, 1rem) * -0.5);
    font-size: var(--m3-font-body-small-size, 0.85rem);
    line-height: var(--m3-font-body-small-height, 1rem);
    letter-spacing: var(--m3-font-body-small-tracking, 0.4);
  }
  .m3-container:hover label {
    color: rgb(var(--error, var(--m3-scheme-on-surface)));
  }
  .m3-container:hover .layer {
    border-color: rgb(var(--error, var(--m3-scheme-on-surface)));
  }
  .m3-container.focus label {
    color: rgb(var(--error, var(--m3-scheme-primary)));
  }
  .m3-container.focus .layer {
    border-color: rgb(var(--error, var(--m3-scheme-primary)));
    border-width: 0.125rem;
  }

  .m3-container:hover :global(svg) { color: rgb(var(--m3-scheme-on-surface)); }
  .m3-container.focus :global(svg) { color: rgb(var(--m3-scheme-primary)); }

  .disabled { pointer-events: none; }
  .disabled .select-mimic {color: rgb(var(--m3-scheme-on-surface) / 0.38); }
  .disabled label {color: rgb(var(--m3-scheme-on-surface) / 0.38); }
  .disabled .layer { border-color: rgb(var(--m3-scheme-on-surface) / 0.38); }
  .disabled :global(svg) { color: rgb(var(--m3-scheme-on-surface) / 0.38); }
</style>