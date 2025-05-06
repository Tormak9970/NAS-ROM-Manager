<script lang="ts">
  import { onMount } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { TransitionConfig } from "svelte/transition";

  import { Icon } from "@component-utils";
  import { CalendarTodayOutline } from "@icons";
  import { easeEmphasized } from "@utils";
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
  let hasJs = $state(false);

  onMount(() => {
    hasJs = true;
  });

  let picker = $state(false);

  // @ts-expect-error This will always be defined before its usage.
  let container: HTMLDivElement = $state();

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
  
  let error = $derived(!validate(date));
</script>

<div class="m3-container" class:has-js={hasJs} class:error class:disabled bind:this={container}>
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
    />
  </div>
  <label class="m3-font-body-large" for={id}>{name}</label>
  <button type="button" class="trailing" {disabled} onclick={() => (picker = !picker)}>
    <Icon icon={CalendarTodayOutline} />
  </button>
  {#if picker}
    <div class="picker" use:clickOutside transition:enterExit>
      <DatePicker
        clearable={!required}
        bind:date
        onclose={() => (picker = false)}
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
    min-width: 15rem;
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
    position: absolute;
    top: calc(100% + 1rem);
    right: 0;
    z-index: 3;
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