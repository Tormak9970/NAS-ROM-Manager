<script lang="ts">
  import { debounce } from "@utils";
  import { onMount } from "svelte";
  import type { HTMLAttributes, HTMLInputAttributes } from "svelte/elements";
  import { Spring } from "svelte/motion";

  type Props = {
    extraWrapperOptions?: HTMLAttributes<HTMLDivElement>;
    extraOptions?: HTMLInputAttributes;
    value: number;
    min?: number;
    max?: number;
    step?: number | "any";
    disabled?: boolean;
    trackHeight?: string;
    thumbSize?: string;
    trackGap?: string;
    trackContainerColor?: string;
    trackColor?: string;
  }

  let {
    extraWrapperOptions = {},
    extraOptions = {},
    value = $bindable(),
    min = 0,
    max = 100,
    step = "any",
    disabled = false,
    trackHeight = "0.5rem",
    thumbSize = "1rem",
    trackGap = "0.75rem",
    trackContainerColor = "var(--m3-scheme-primary-container)",
    trackColor = "var(--m3-scheme-primary)"
  }: Props = $props();

  function setValue(newValue: number) {
    value = newValue;
  }

  // @ts-expect-error we're binding context to ensure that the slider's value gets set, but ts won't be happy
  const debouncedSet = debounce(setValue.bind(this), 100);

  // TODO: make this bindable
  export const valueDisplayed = new Spring(value, { stiffness: 0.3, damping: 1 });

  function updateValue(e: Event & { currentTarget: EventTarget & HTMLInputElement }) {
    const newValue = Number(e.currentTarget.value);
    e.preventDefault();
    debouncedSet(newValue);
    $valueDisplayed = newValue;
  };

  // @ts-expect-error This will always be defined before its usage.
  let range: number = $state();
  // @ts-expect-error This will always be defined before its usage.
  let percent: number = $state();

  $effect(() => {
    range = max - min;
    percent = (valueDisplayed.current - min) / range;
  });
  
  onMount(() => {
    valueDisplayed.set(value, { hard: true });
  });
</script>

<div class="m3-container" style:--percent="{percent * 100}%" style:--track-height={trackHeight} style:--thumb-size={thumbSize} style:--track-gap={trackGap} style:--track-container-color={trackContainerColor} style:--track-color={trackColor} {...extraWrapperOptions}>
  <input
    type="range"
    oninput={updateValue}
    value={valueDisplayed.current}
    {min}
    {max}
    {step}
    {disabled}
    {...extraOptions}
  />
  <div class="track"></div>
  <div class="thumb"></div>
</div>

<style>
  :root {
    --m3-slider-track-out-shape: 0.5rem;
    --m3-slider-track-in-shape: 0.125rem;
    --m3-slider-thumb-shape: var(--m3-util-rounding-full);
  }
  .m3-container {
    position: relative;
    height: 2.75rem;
    min-width: 10rem;
  }
  input {
    position: absolute;
    left: -0.5rem;
    right: -0.5rem;
    width: calc(100% + 1rem);
    height: 100%;

    opacity: 0;
    appearance: none;
    margin: 0;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
  }

  .track::before {
    position: absolute;
    content: " ";
    left: 0;
    top: 50%;
    translate: 0 -50%;
    width: calc(var(--percent) - var(--track-gap));
    height: var(--track-height);
    pointer-events: none;

    background-color: rgb(var(--track-color));
    border-start-start-radius: var(--m3-slider-track-out-shape);
    border-end-start-radius: var(--m3-slider-track-out-shape);
    border-start-end-radius: var(--m3-slider-track-in-shape);
    border-end-end-radius: var(--m3-slider-track-in-shape);
  }
  .track::after {
    position: absolute;
    content: " ";
    right: 0;
    top: 50%;
    translate: 0 -50%;
    width: calc(100% - var(--percent) - var(--track-gap));
    height: var(--track-height);
    pointer-events: none;

    background-color: rgb(var(--track-container-color));
    border-start-start-radius: var(--m3-slider-track-in-shape);
    border-end-start-radius: var(--m3-slider-track-in-shape);
    border-start-end-radius: var(--m3-slider-track-out-shape);
    border-end-end-radius: var(--m3-slider-track-out-shape);
  }

  .thumb {
    position: absolute;
    left: var(--percent);
    top: 50%;
    translate: -50% -50%;
    width: var(--thumb-size);
    height: var(--thumb-size);
    border-radius: 50%;
    background-color: rgb(var(--track-color));

    pointer-events: none;
    transition: width 200ms;
  }

  input:focus-visible ~ .thumb {
    outline: auto;
    outline-offset: 0.5rem;
  }

  input:disabled {
    cursor: auto;
  }
  input:disabled ~ .track::before {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
  input:disabled ~ .track::after {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }
  input:disabled ~ .thumb {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
</style>