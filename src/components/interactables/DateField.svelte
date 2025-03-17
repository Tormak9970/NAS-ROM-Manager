<!-- <script lang="ts">
  import { CalendarMonth } from "@icons";
  import { DatePicker } from "@svelte-plugins/datepicker";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";
  import datefield from "./datefield.svelte";

  export let extraWrapperOptions: HTMLAttributes<HTMLDivElement> = {};
  export let extraOptions: HTMLInputAttributes = {};
  export let name: string;
  
  export let value: Date = new Date();

  export let disabled = false;
  
  let isOpen = false;
  
  function format(date: Date) {
    return (date.getMonth() + 1) + "/" + date.getDate() + "/" + date.getFullYear();
  }

  function formatDate(date: Date) {
    // @ts-expect-error Date can be treated as number.
    if (isNaN(new Date(date))) {
      return '';
    }

    return date && format(new Date(date)) || '';
  }

  $: formattedStartDate = formatDate(value);
</script>

<div class="date-picker-wrapper">
  <DatePicker
    bind:isOpen
    bind:startDate={value}
    includeFont={false}
  >
    <datefield
      name={name}
      trailingIcon={CalendarMonth}
      disabled={disabled}
      readonly
      on:trailingClick={() => isOpen = !isOpen}
      extraOptions={extraOptions}
      extraWrapperOptions={extraWrapperOptions}
      bind:value={formattedStartDate}
    />
  </DatePicker>
</div>

<style>
  .date-picker-wrapper {
    --datepicker-state-active: rgb(var(--m3-scheme-primary) / 0.8);
    --datepicker-state-hover: rgb(var(--m3-scheme-primary) / 0.08);

    --datepicker-color: rgb(var(--m3-scheme-on-surface));
    --datepicker-calendar-range-selected-background: rgb(var(--m3-scheme-primary) / 0.8);
    --datepicker-calendar-dow-color: #8b9198;

    --datepicker-font-family: 'Google-sans';
    --datepicker-container-font-family: var(--datepicker-font-family);
    --datepicker-presets-button-font-family: var(--datepicker-font-family);
    --datepicker-timepicker-input-font-family: var(--datepicker-font-family);
    --datepicker-calendar-day-font-family: var(--datepicker-font-family);

    --datepicker-container-background: rgb(var(--m3-scheme-surface-container-high));
    --datepicker-container-border: 1px solid transparent;
    --datepicker-container-border-radius: var(--m3-util-rounding-medium);
    --datepicker-container-box-shadow: 0px 2px 4px -1px rgb(var(--m3-scheme-shadow) / 0.2),
    0px 4px 5px 0px rgb(var(--m3-scheme-shadow) / 0.14),
    0px 1px 10px 0px rgb(var(--m3-scheme-shadow) / 0.12);
    --datepicker-calendar-header-color: var(--datepicker-color);
    --datepicker-calendar-header-month-nav-background-hover: var(--datepicker-state-hover);
    --datepicker-calendar-header-month-nav-color: var(--datepicker-color);
    --datepicker-calendar-header-month-nav-icon-next-filter: invert(1);
    --datepicker-calendar-header-month-nav-icon-prev-filter: invert(1);
    --datepicker-calendar-header-text-color: var(--datepicker-color);
    --datepicker-calendar-header-year-nav-icon-next-filter: invert(1);
    --datepicker-calendar-header-year-nav-icon-prev-filter: invert(1);
    --datepicker-calendar-day-background-hover: var(--datepicker-state-hover);
    --datepicker-calendar-day-color: rgb(var(--m3-scheme-on-surface));
    --datepicker-calendar-day-color-disabled: rgb(var(--m3-scheme-on-surface) / 0.38);
    --datepicker-calendar-day-color-hover: rgb(var(--m3-scheme-primary));
    --datepicker-calendar-day-other-color: rgb(var(--m3-scheme-on-surface) / 0.38);
    --datepicker-calendar-today-border: 1px solid transparent;
  }
</style> -->

<script lang="ts">
  import { onMount } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { TransitionConfig } from "svelte/transition";

  import { Icon } from "@component-utils";
  import { CalendarTodayOutline } from "@icons";
  import { easeEmphasized } from "@utils";
  import DatePicker from "./date-picker/DatePicker.svelte";

  export let name: string;
  export let date = "";
  export let required = false;
  export let disabled = false;
  export let readonly = false;
  export let extraOptions: HTMLInputAttributes = {};
  
  export let validate: (date: string) => boolean = (date: string) => { return true; };

  const id = crypto.randomUUID();
  let hasJs = false;

  onMount(() => {
    hasJs = true;
  });

  let picker = false;
  let container: HTMLDivElement;

  const clickOutside = (_node: Node) => {
    const handleClick = (event: Event) => {
      if (!container.contains(event.target as Node)) {
        picker = false;
      }
    };
    
    document.addEventListener("click", handleClick, true);

    return {
      destroy() {
        document.removeEventListener("click", handleClick, true);
      },
    };
  };

  const enterExit = (_node: Node): TransitionConfig => {
    return {
      duration: 400,
      easing: easeEmphasized,
      css: (t, u) => `clip-path: inset(-100% 0 ${u * 100}% 0 round 1rem);
transform-origin: top;
transform: scaleY(${(t * 0.3 + 0.7) * 100}%);
opacity: ${Math.min(t * 3, 1)};`,
    };
  };
  
  $: error = !validate(date);
</script>

<div class="m3-container" class:has-js={hasJs} class:error class:disabled bind:this={container}>
  <input
    type="date"
    class="m3-font-body-large"
    {disabled}
    {required}
    {readonly}
    {id}
    bind:value={date}
    {...extraOptions}
  />
  <div class="layer"></div>
  <label class="m3-font-body-small" for={id}>{name}</label>
  <button type="button" class="trailing" {disabled} on:click={() => (picker = !picker)}>
    <Icon icon={CalendarTodayOutline} />
  </button>
  {#if picker}
    <div class="picker" use:clickOutside transition:enterExit>
      <DatePicker
        clearable={!required}
        bind:date
        on:close={() => (picker = false)}
        on:setDate={(e: { detail: string; }) => (date = e.detail)}
      />
    </div>
  {/if}
</div>

<style>
  :root {
    --m3-datefield-outlined-shape: var(--m3-util-rounding-extra-small);
  }

  .m3-container {
    position: relative;
    
    display: inline-flex;
    align-items: center;

    height: 3rem;
    min-width: 15rem;
  }
  input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    border: none;
    outline: none;
    padding: 1rem;
    border-radius: var(--m3-datefield-outlined-shape);
    background-color: transparent;
    color: rgb(var(--m3-scheme-on-surface));
  }

  label {
    position: absolute;
    left: 0.75rem;
    top: 0.7rem;
    color: rgb(var(--error, var(--m3-scheme-on-surface-variant)));
    background-color: rgb(var(--m3-util-background, var(--m3-scheme-surface)));
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
    border-radius: var(--m3-datefield-outlined-shape);
    pointer-events: none;
    transition: all 200ms;
  }
  .m3-container :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
    color: rgb(var(--m3-scheme-on-surface-variant));
    pointer-events: none;
  }
  .m3-container > :global(.leading) {
    position: relative;
    margin-left: 0.75rem;
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

  input:not(:read-only):focus ~ label,
  input:not(:placeholder-shown) ~ label {
    top: calc(var(--m3-font-body-small-height, 1rem) * -0.5);
    font-size: var(--m3-font-body-small-size, 0.85rem);
    line-height: var(--m3-font-body-small-height, 1rem);
    letter-spacing: var(--m3-font-body-small-tracking, 0.4);
  }
  input:hover ~ label {
    color: rgb(var(--error, var(--m3-scheme-on-surface)));
  }
  input:hover ~ .layer {
    border-color: rgb(var(--error, var(--m3-scheme-on-surface)));
  }
  input:focus ~ label {
    color: rgb(var(--error, var(--m3-scheme-primary)));
  }
  input:focus ~ .layer {
    border-color: rgb(var(--error, var(--m3-scheme-primary)));
    border-width: 0.125rem;
  }
  @media (hover: hover) {
    button:hover {
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
  .error > input:hover ~ label,
  .error > input:hover ~ .layer {
    --error: var(--m3-scheme-on-error-container);
  }

  input:read-only {
    caret-color: transparent;
  }

  input:disabled {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
  input:disabled ~ label {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
  input:disabled ~ .layer {
    border-color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
  input:disabled ~ :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }

  @supports (-moz-appearance: none) {
    input {
      padding-left: 0.75rem;
    }
  }

  .picker {
    position: absolute;
    top: calc(100% + 1rem);
    right: 0;
    z-index: 1;
  }

  @media (min-width: 37.5rem) {
    .has-js button {
      display: flex;
    }
    .has-js input {
      clip-path: inset(0 3.5rem 0 0);
    }
  }
</style>