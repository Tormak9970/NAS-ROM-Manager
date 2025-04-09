<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLInputAttributes;
    name: string;
    leadingIcon?: IconifyIcon | undefined;
    trailingIcon?: IconifyIcon | undefined;
    validate?: (value: string) => Promise<boolean>;
    disabled?: boolean;
    required?: boolean;
    readonly?: boolean;
    value?: string;
    onchange?: (e: Event) => void;
    oninput?: (e: Event) => void;
    ontrailingClick?: () => void;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    name,
    leadingIcon = undefined,
    trailingIcon = undefined,
    validate = async (value: string) => { return true; },
    disabled = false,
    required = false,
    readonly = false,
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
  class:leading-icon={leadingIcon}
  class:trailing-icon={trailingIcon}
  class:error
  {...extraWrapperOptions}
>
  <input
    class="m3-font-body-large"
    placeholder=" "
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
  <div class="layer"></div>
  <label class="m3-font-body-large" for={id}>{name}</label>
  {#if leadingIcon}
    <Icon icon={leadingIcon} class="leading" />
  {/if}
  {#if trailingIcon}
    <button onclick={ontrailingClick} class="trailing">
      <Icon icon={trailingIcon} />
    </button>
  {/if}
</div>

<style>
  :root {
    --m3-textfield-outlined-shape: var(--m3-util-rounding-extra-small);
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
    border-radius: var(--m3-textfield-outlined-shape);
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

  .leading-icon > input {
    padding-left: 3.25rem;
  }
  .leading-icon > input:not(:focus):placeholder-shown ~ label {
    left: 3rem;
  }
  .trailing-icon > input {
    padding-right: 3.25rem;
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
</style>