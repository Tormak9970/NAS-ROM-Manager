<!-- @component Must be wrapped in a `<label>`. This is because clicking on a label passes the click event to the toggle. -->
<script lang="ts">
  import { Icon } from "@component-utils";
  import { Checkmark } from "@icons";
  import type { HTMLAttributes } from "svelte/elements";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLAttributes<HTMLDivElement>;
    checked?: boolean;
    disabled?: boolean;
    onchange?: (e: Event) => void;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    checked = $bindable(false),
    disabled = false,
    onchange = () => {},
  }: Props = $props();

  let startX: number | undefined = $state();
  const handleMouseUp = (e: MouseEvent) => {
    if (!startX) return;
    const distance = e.clientX - startX;
    if (distance > 16 && !checked) checked = true;
    if (distance < -16 && checked) checked = false;
    startX = undefined;
  };
</script>

<svelte:window onmouseup={handleMouseUp} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="m3-container"
  {...extraWrapperOptions}
  onmousedown={(e) => {
    if (!disabled) {
      startX = e.clientX;
    }
  }}
>
  <input
    type="checkbox"
    role="switch"
    {disabled}
    bind:checked
    {...extraOptions}
    onkeydown={(e) => {
      if (e.code == "Enter") checked = !checked;
      if (e.code == "ArrowLeft") checked = false;
      if (e.code == "ArrowRight") checked = true;
    }}
    {onchange}
  />
  <div class="layer">
    <Icon icon={Checkmark} />
  </div>
</div>

<style>
  :root {
    --m3-switch-track-shape: var(--m3-util-rounding-full);
    --m3-switch-handle-shape: var(--m3-util-rounding-full);
  }
  .m3-container {
    position: relative;
    width: 3.25rem;
    height: 2rem;

    display: inline-flex;
  }
  input {
    appearance: none;
    width: 3.25rem;
    height: 2rem;
    margin: 0;
    border-radius: var(--m3-switch-track-shape);

    background-color: rgb(var(--m3-scheme-surface-container-highest));
    border: solid 0.125rem rgb(var(--m3-scheme-outline));
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
    transition: all 300ms;
  }
  .layer {
    position: absolute;
    width: 1rem;
    height: 1rem;
    border-radius: var(--m3-switch-handle-shape);

    background-color: rgb(var(--m3-scheme-outline));
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
    transition: all 300ms cubic-bezier(0.271, -0.011, 0, 1.449);

    left: 0.5rem;
    top: 50%;
    transform: translate(0, -50%);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .layer > :global(svg) {
    width: 1rem;
    height: 1rem;
    color: rgb(var(--m3-scheme-on-primary-container));
    opacity: 0;
    transition: opacity 300ms cubic-bezier(0.271, -0.011, 0, 1.449);
  }
  .layer::before {
    content: " ";
    display: block;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 2.5rem;
    height: 2.5rem;
    border-radius: var(--m3-util-rounding-full);
    transition: all 200ms;
  }

  .m3-container:hover > input:enabled + .layer,
  .m3-container > input:enabled:is(:global(:active, :focus-visible)) + .layer {
    background-color: rgb(var(--m3-scheme-on-surface-variant));
  }
  .m3-container:hover > input:enabled:checked + .layer,
  .m3-container > input:enabled:checked:is(:global(:active, :focus-visible)) + .layer {
    background-color: rgb(var(--m3-scheme-primary-container));
  }
  .m3-container:hover > input + .layer::before {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
  }
  .m3-container:hover > input:checked + .layer::before {
    background-color: rgb(var(--m3-scheme-primary) / 0.08);
  }
  .m3-container > input:is(:global(:active, :focus-visible)) + .layer::before {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }
  .m3-container > input:checked:is(:global(:active, :focus-visible)) + .layer::before {
    background-color: rgb(var(--m3-scheme-primary) / 0.12);
  }

  input:checked {
    background-color: rgb(var(--m3-scheme-primary));
    border-color: rgb(var(--m3-scheme-primary));
  }
  input:checked + .layer {
    background-color: rgb(var(--m3-scheme-on-primary));
    width: 1.5rem;
    height: 1.5rem;
    left: 1.5rem; /* 1.5 + 1.5 + 0.25 = 3.25 */
  }
  input:checked + .layer > :global(svg) {
    opacity: 1;
  }
  .m3-container:active > input:enabled + .layer {
    width: 1.75rem;
    height: 1.75rem;
    transform: translate(-0.375rem, -50%); /* 0.75 / 2 */
  }
  .m3-container:active > input:enabled:checked + .layer {
    transform: translate(-0.125rem, -50%); /* 0.25 / 2 */
  }

  input:disabled {
    background-color: rgb(var(--m3-scheme-surface-container-highest) / 0.12);
    border-color: rgb(var(--m3-scheme-outline) / 0.12);
    cursor: auto;
  }
  input:disabled:checked {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
    border-color: transparent;
  }
  input:disabled + .layer {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.38);
    cursor: auto;
  }
  input:disabled:checked + .layer {
    background-color: rgb(var(--m3-scheme-surface));
  }
  input:disabled:checked + .layer > :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
  input:disabled + .layer::before {
    display: none;
  }
</style>