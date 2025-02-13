<script lang="ts">
  import { CalendarMonth } from "@icons";
  import { DatePicker } from "@svelte-plugins/datepicker";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";
  import TextField from "./TextField.svelte";

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
    <TextField
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
</style>