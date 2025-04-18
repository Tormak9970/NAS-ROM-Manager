<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import { Checkmark } from "@icons";
  import type { Snippet } from "svelte";
  import type { HTMLLabelAttributes } from "svelte/elements";

  type Props = {
    extraOptions?: HTMLLabelAttributes;
    id: string;
    name: string;
    checked: boolean;
    icon?: IconifyIcon | undefined;
    children?: Snippet;
    oninput?: (e: Event) => void;
  }

  let {
    extraOptions = {},
    id,
    name,
    checked,
    icon = undefined,
    children,
    oninput = () => {}
  }: Props = $props();
</script>

<input type="radio" name={name} id={id} checked={checked} {oninput} />
<label
  for={id}
  class="m3-font-label-large"
  {...extraOptions}
>
  <div class="layer"></div>
  {#if icon}
    <div class="custom icon">
      <Icon {icon} />
    </div>
  {/if}
  <div class="check icon">
    <Icon icon={Checkmark} />
  </div>
  <div class="start-pad pad"></div>
  {@render children?.()}
  {#if !icon}
    <div class="end-pad pad"></div>
  {/if}
</label>


<style>
  label {
    padding: 0 1rem;
    /* flex: 1; */
    min-width: 3rem;

    display: flex;

    overflow: hidden;

    align-items: center;
    justify-content: center;

    --text-color: var(--m3-scheme-on-surface);
    color: rgb(var(--text-color));
    transition: all 200ms;

    cursor: pointer;
    white-space: nowrap;
    user-select: none;
    position: relative;
  }
  :global(label.m3-font-label-large) ~ label.m3-font-label-large {
    border-left: 0.0625rem solid rgb(var(--m3-scheme-outline));
  }
  input:disabled + label {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
    cursor: auto;
  }
  .layer {
    position: absolute;
    inset: 0;
    transition: all 200ms;
  }
  .icon {
    height: 1.125rem;
    transition: all 200ms;
    flex-shrink: 0;
    transform-origin: 0.563rem 0.563rem;
  }
  .icon > :global(svg) {
    width: 1.125rem;
    height: 1.125rem;
  }

  .check.icon {
    width: 0;
    opacity: 0;
  }
  input:checked + label > .check.icon {
    opacity: 1;
  }
  .custom.icon + .check.icon {
    rotate: -60deg;
  }
  input:checked + label > .custom.icon + .check.icon {
    rotate: 0deg;
  }
  .custom.icon {
    width: 0;
    opacity: 0;
    rotate: 60deg;
  }
  input:not(:checked) + label > .custom.icon {
    opacity: 1;
    rotate: 0deg;
  }

  .pad {
    transition: all 200ms;
    flex-shrink: 0;
  }
  .start-pad {
    width: 0.8125rem;
  }
  .end-pad {
    width: 0.8125rem;
  }
  input:checked + label > .start-pad,
  .custom.icon ~ .start-pad {
    width: 1.625rem;
  }
  input:checked + label > .end-pad {
    width: 0rem;
  }

  label {
    -webkit-tap-highlight-color: transparent;
  }
  @media (hover: hover) {
    input:not(:disabled) + label:hover > .layer {
      background-color: rgb(var(--text-color) / 0.08);
    }
  }

  input:checked + label {
    background-color: rgb(var(--m3-scheme-secondary-container));
    --text-color: var(--m3-scheme-on-secondary-container);
  }
  input:enabled:focus-visible + label > .layer,
  input:enabled + label:active > .layer {
    background-color: rgb(var(--text-color) / 0.12);
  }

  input {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }
</style>