<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLInputAttributes;
    name: string;
    trailingIcon?: IconifyIcon | undefined;
    validate?: (value: string) => Promise<boolean>;
    disabled?: boolean;
    required?: boolean;
    readonly?: boolean;
    value?: string;
    placeholder?: string;
    onchange?: (e: Event) => void;
    oninput?: (e: Event) => void;
    ontrailingClick?: () => void;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    name,
    trailingIcon = undefined,
    validate = async (value: string) => { return true; },
    disabled = false,
    required = false,
    readonly = false,
    placeholder = " ",
    value = $bindable(""),
    onchange = () => {},
    oninput = () => {},
    ontrailingClick,
  }: Props = $props();
  
  const id = crypto.randomUUID();
  
  let error = $state(false);

  $effect(() => {
    validate(value).then((isValid) => error = !isValid);
  });
</script>

<div
  class="m3-container"
  class:trailing-icon={trailingIcon}
  class:error
  {...extraWrapperOptions}
>
  <input
    class="m3-font-body-large"
    placeholder={placeholder}
    autocomplete="off"
    bind:value
    {id}
    {disabled}
    {required}
    {readonly}
    {...extraOptions}
    {onchange}
    {oninput}
  />
  <label class="m3-font-body-large" for={id}>{name}</label>
  {#if trailingIcon}
    <button onclick={ontrailingClick} class="trailing" disabled={disabled}>
      <Icon icon={trailingIcon} />
    </button>
  {/if}
</div>

<style>
  :root {
    --m3-textfield-outlined-shape: var(--m3-util-rounding-medium);
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
    
    border-radius: var(--m3-textfield-outlined-shape);
    /* background-color: rgb(var(--m3-util-background, var(--m3-scheme-secondary-container))); */
    background-color: rgb(var(--m3-util-background, var(--m3-scheme-surface-variant)));
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

  .trailing-icon > input {
    padding-right: 3.25rem;
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
  button.trailing:disabled :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.18);
  }
</style>