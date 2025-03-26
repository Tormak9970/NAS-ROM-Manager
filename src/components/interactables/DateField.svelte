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