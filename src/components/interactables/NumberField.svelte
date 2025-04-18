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
    disabled?: boolean;
    required?: boolean;
    error?: boolean;
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
    disabled = false,
    required = false,
    error = false,
    value = $bindable(""),
    onchange = () => {},
    oninput = () => {},
    ontrailingClick,
  }: Props = $props();
  
  const id = crypto.randomUUID();
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
    type="number"
    autocomplete="off"
    bind:value
    {id}
    {disabled}
    {required}
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
    --m3-numberfield-outlined-shape: var(--m3-util-rounding-extra-small);
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
    border-radius: var(--m3-numberfield-outlined-shape);
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
    border-radius: var(--m3-numberfield-outlined-shape);
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
  .trailing {
    position: absolute;
    padding-left: 0.75rem;
    padding-right: 0.75rem;
    height: 100%;
    right: 0;

    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background-color: transparent;
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;

    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }

  input:focus ~ label,
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
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
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