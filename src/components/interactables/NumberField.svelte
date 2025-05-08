<script lang="ts">
  import { Icon } from "@component-utils";
  import { KeyboardArrowDown, KeyboardArrowUp } from "@icons";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLInputAttributes;
    name: string;
    disabled?: boolean;
    required?: boolean;
    error?: boolean;
    value?: string;
    onchange?: (e: Event) => void;
    oninput?: (e: Event) => void;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    name,
    disabled = false,
    required = false,
    error = false,
    value = $bindable(""),
    onchange = () => {},
    oninput = () => {},
  }: Props = $props();
  
  const id = crypto.randomUUID();

  function increment(step: number) {
    if (value === "") {
      value = step.toString();
    } else {
      value = (parseInt(value) + step).toString();
    }
  }
</script>

<div
  class="m3-container"
  class:error
  {...extraWrapperOptions}
>
  <input
    class="m3-font-body-large"
    placeholder=" "
    type="number"
    autocomplete="off"
    bind:value
    {id}
    {disabled}
    {required}
    {...extraOptions}
    {onchange}
    oninput={(e: Event) => {
      (e.target as HTMLInputElement).value = (e.target as HTMLInputElement).value.replace(/[^0-9]/g, '');
      oninput(e);
    }}
  />
  <label class="m3-font-body-large" for={id}>{name}</label>
  <div class="trailing">
    <button onclick={() => increment(1)} class="up" disabled={disabled}>
      <Icon icon={KeyboardArrowUp} />
    </button>
    <button onclick={() => increment(-1)} class="down" disabled={disabled}>
      <Icon icon={KeyboardArrowDown} />
    </button>
  </div>
</div>

<style>
  :root {
    --m3-numberfield-outlined-shape: var(--m3-util-rounding-medium);
  }
  .m3-container {
    position: relative;

    display: flex;
    flex-direction: column-reverse;

    height: 4.25rem;
    min-width: 15rem;
  }
  input {
    width: 100%;
    height: 100%;
    border: none;
    outline: none;
    padding: 0.75rem;

    color: rgb(var(--m3-scheme-on-surface));
    
    border-radius: var(--m3-numberfield-outlined-shape);
    background-color: rgb(var(--m3-util-background, var(--m3-scheme-surface-variant)));

    -webkit-appearance: textfield;
    -moz-appearance: textfield;
    appearance: textfield;

    padding-right: 3.25rem;
  }

  input::-webkit-inner-spin-button,
  input::-webkit-outer-spin-button {
    -webkit-appearance: none;
  }

  label {
    color: rgb(var(--error, var(--m3-scheme-on-surface-variant)));
    font-weight: bold;

    transition: color 200ms;

    font-size: 0.9rem;
  }
  .m3-container :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
    color: rgb(var(--m3-scheme-on-surface-variant));
    pointer-events: none;
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

    height: calc(100% - 1.5rem);
    width: 2.25rem;
    right: 0;
    bottom: 0;

    border: none;
    background-color: transparent;

    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;

    display: flex;
    flex-direction: column;
  }

  button {
    height: 50%;
    width: 2.25rem;
    
    border: none;
    background-color: transparent;

    cursor: pointer;
  }
  .up {
    border-top-right-radius: var(--m3-numberfield-outlined-shape);
  }
  .down {
    border-bottom-right-radius: var(--m3-numberfield-outlined-shape);
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

  .error {
    --error: var(--m3-scheme-error);
  }
  .error > input:hover ~ label {
    --error: var(--m3-scheme-on-error-container);
  }
  .error > input {
    background-color: rgb(var(--m3-scheme-tertiary-container));
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
  button:disabled :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.18);
  }
</style>