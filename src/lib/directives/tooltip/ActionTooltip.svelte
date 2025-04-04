<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { computeTooltipPosition, formatVariableKey, getMinWidth, isElementInViewport } from './helpers';

  type Props = {
    targetElement?: HTMLElement | null;
    content?: string | { component: any; props?: Record<string, any>; };
    align?: 'left' | string;
    position?: 'top' | string;
    maxWidth?: number;
    style?: Record<string, string> | null;
    theme?: string;
    animation?: string;
    delay?: number;
    arrow?: boolean;
    autoPosition?: boolean;
    show?: boolean;
  }

  let {
    targetElement = null,
    content = $bindable(''),
    align = 'left',
    position = 'top',
    maxWidth = 200,
    style = null,
    theme = '',
    animation = '',
    delay = 200,
    arrow = true,
    autoPosition = false,
    show = $bindable(false),
  }: Props = $props();

  const inverse = {
    left: 'right',
    right: 'left',
    top: 'bottom',
    bottom: 'top'
  };

  let tooltipRef: HTMLDivElement | null = $state(null);

  let minWidth: number = $state(0);

  let component: any = $state(null);

  let animationEffect: string | null = $state(null);

  let visible: boolean = $state(false);

  let coords: any = $state({
    bottom: 0,
    top: 0,
    right: 0,
    left: 0
  });

  const animationDelay = $derived(animation ? delay : 0);
  const isComponent = $derived(typeof content === 'object');
  
  $effect(() => {
    if (tooltipRef && show) {
      setTimeout(() => (visible = true), 0)
    } else {
      visible = false
    }
  });

  const onHandleResize = () => {
    if (visible) {
      coords = computeTooltipPosition(
        targetElement,
        tooltipRef,
        position,
        coords
      );
    }
  };

  onMount(() => {
    if (tooltipRef !== null) {
      if (isComponent && !component) {
        // @ts-expect-error if isComponent then this is valid.
        component = new content.component({
          target: tooltipRef,
          // @ts-expect-error if isComponent then this is valid.
          props: { ...content.props }
        });
      }

      minWidth = getMinWidth(tooltipRef, maxWidth);

      if (style && typeof style === 'object') {
        for (let prop in style) {
          const key = formatVariableKey(prop);
          const value = style[prop];

          tooltipRef.style.setProperty(`--tooltip-${key}`, value);
        }
      }
    }

    if (autoPosition && !isElementInViewport(tooltipRef!, targetElement, position)) {
      // @ts-expect-error position will always index inverse.
      position = inverse[position];
    }

    coords = computeTooltipPosition(
      targetElement,
      tooltipRef,
      position,
      coords
    );

    if (animation) {
      animationEffect = animation;
    }

    setTimeout(() => (visible = true), animationDelay);
  });

  onDestroy(() => {
    if (component) {
      component.$destroy();
      component = null;
      visible = false;
      show = false;
    }
  });
</script>

{#if content}
  <div
    bind:this={tooltipRef}
    class="tooltip animation-{animationEffect} {position} {theme}"
    class:show={visible}
    class:arrowless={!arrow}
    style="bottom: auto; right: auto; left: {coords.left}px; min-width: {minWidth}px; max-width: {maxWidth}px; text-align: {align}; top: {coords.top}px;"
  >
    {#if !isComponent}
      {@html content}
    {/if}
  </div>
{/if}

<svelte:window onresize={onHandleResize} />

<style>
  /*--------------------------*
   * Theme Variables
   *--------------------------*/

  :root {
    --tooltip-arrow-size: 10px;
    --tooltip-background-color: rgba(0, 0, 0, 0.9);
    --tooltip-border-radius: 4px;
    --tooltip-box-shadow: 0 1px 20px rgba(0, 0, 0, 0.25);
    --tooltip-font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
      Oxygen-Sans, Ubuntu, Cantarell, 'Helvetica Neue', sans-serif;
    --tooltip-font-size: 14px;
    --tooltip-font-weight: 500;
    --tooltip-line-height: 1.25rem;
    --tooltip-color: #fff;
    --tooltip-offset-x: 12px;
    --tooltip-offset-y: 12px;
    --tooltip-padding: 12px;
    --tooltip-pointer-events: none;
    --tooltip-white-space-hidden: nowrap;
    --tooltip-white-space-shown: normal;
    --tooltip-z-index: 100;
  }

  /*--------------------------*
   * Tooltip Styling
   *--------------------------*/

  .tooltip {
    background-color: var(--tooltip-background-color);
    box-shadow: var(--tooltip-box-shadow);
    border-radius: var(--tooltip-border-radius);
    color: var(--tooltip-color);
    opacity: 0;
    font-family: var(--tooltip-font-family);
    font-size: var(--tooltip-font-size);
    font-style: normal;
    font-weight: var(--tooltip-font-weight);
    line-height: var(--tooltip-line-height);
    padding: var(--tooltip-padding);
    pointer-events: var(---tooltip-pointer-events);
    position: absolute;
    text-align: left;
    visibility: hidden;
    white-space: var(--tooltip-white-space-hidden);
    z-index: var(--tooltip-z-index);
  }

  .tooltip.show {
    opacity: 1;
    visibility: visible;
    white-space: var(--tooltip-white-space-shown);
  }

  .tooltip.bottom:after,
  .tooltip.left:after,
  .tooltip.right:after,
  .tooltip.top:after {
    border: var(--tooltip-arrow-size) solid var(--tooltip-background-color);
    content: ' ';
    position: absolute;
  }

  .tooltip.arrowless:after {
    border: 0 !important;
  }

  .tooltip.bottom,
  .tooltip.top {
    --tooltip-offset-x: 0px;
    --tooltip-offset-y: 12px;
  }

  .tooltip.left,
  .tooltip.right {
    --tooltip-offset-x: 12px;
    --tooltip-offset-y: 0px;
  }

  .tooltip.bottom {
    bottom: 0;
    left: 50%;
    transform: translate(
      calc(-50% + var(--tooltip-offset-x)),
      calc(100% + var(--tooltip-offset-y))
    );
  }

  .tooltip.bottom:after {
    border-color: transparent transparent var(--tooltip-background-color);
    left: 50%;
    top: 0;
    transform: translate(-50%, -99%);
  }

  .tooltip.top {
    left: 50%;
    top: 0;
    transform: translate(
      calc(-50% + var(--tooltip-offset-x)),
      calc(-100% - var(--tooltip-offset-y))
    );
  }

  .tooltip.top:after {
    border-color: var(--tooltip-background-color) transparent transparent
      transparent;
    bottom: 0;
    left: 50%;
    transform: translate(-50%, 99%);
  }

  .tooltip.left {
    left: 0;
    top: 50%;
    transform: translate(
      calc(-100% - var(--tooltip-offset-x)),
      calc(-50% - var(--tooltip-offset-y))
    );
  }

  .tooltip.left:after {
    border-color: transparent transparent transparent
      var(--tooltip-background-color);
    right: 0;
    top: 50%;
    transform: translate(99%, -50%);
  }

  .tooltip.right {
    right: 0;
    top: 50%;
    transform: translate(
      calc(100% + var(--tooltip-offset-x)),
      calc(-50% - var(--tooltip-offset-y))
    );
  }

  .tooltip.right:after {
    border-color: transparent var(--tooltip-background-color) transparent
      transparent;
    left: 0;
    top: 50%;
    transform: translate(-99%, -50%);
  }

  /*--------------------------*
   * CSS Animations
   *--------------------------*/

  /* Fade */

  .tooltip.animation-fade {
    opacity: 0;
    transition: opacity 0.25s ease-in-out;
  }

  .tooltip.animation-fade.show {
    opacity: 1;
  }

  /* Slide */

  .tooltip.top.animation-slide {
    margin-top: 10px;
    opacity: 0;
    transition:
      opacity 0.25s ease-in-out,
      margin 0.25s ease-in-out;
  }

  .tooltip.top.animation-slide.show {
    margin-top: 0;
    opacity: 1;
  }

  .tooltip.bottom.animation-slide {
    margin-bottom: 20px;
    opacity: 0;
    transition:
      opacity 0.25s ease-in-out,
      margin 0.25s ease-in-out;
  }

  .tooltip.bottom.animation-slide.show {
    margin-bottom: 0;
    opacity: 1;
  }

  .tooltip.right.animation-slide {
    margin-right: 20px;
    opacity: 0;
    transition:
      opacity 0.25s ease-in-out,
      margin 0.25s ease-in-out;
  }

  .tooltip.right.animation-slide.show {
    margin-right: 0;
    opacity: 1;
  }

  .tooltip.left.animation-slide {
    margin-left: 20px;
    opacity: 0;
    transition:
      opacity 0.25s ease-in-out,
      margin 0.25s ease-in-out;
  }

  .tooltip.left.animation-slide.show {
    margin-left: 0;
    opacity: 1;
  }

  /* Puff */

  .tooltip.left.animation-puff {
    filter: blur(2px);
    opacity: 0;
    transform: translate(
        calc(-100% - var(--tooltip-offset-x)),
        calc(-50% - var(--tooltip-offset-y))
      )
      scale(2, 2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      filter 0.25s ease-in-out,
      transform 0.25s ease-in-out;
  }

  .tooltip.left.animation-puff.show {
    filter: blur(0);
    opacity: 1;
    transform: translate(
        calc(-100% - var(--tooltip-offset-x)),
        calc(-50% - var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  .tooltip.right.animation-puff {
    filter: blur(2px);
    opacity: 0;
    transform: translate(
        calc(100% + var(--tooltip-offset-x)),
        calc(-50% - var(--tooltip-offset-y))
      )
      scale(2, 2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      filter 0.25s ease-in-out,
      transform 0.25s ease-in-out;
  }

  .tooltip.right.animation-puff.show {
    filter: blur(0);
    opacity: 1;
    transform: translate(
        calc(100% + var(--tooltip-offset-x)),
        calc(-50% - var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  .tooltip.top.animation-puff {
    filter: blur(2px);
    opacity: 0;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(-100% - var(--tooltip-offset-y))
      )
      scale(2, 2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      filter 0.25s ease-in-out,
      transform 0.25s ease-in-out;
  }

  .tooltip.top.animation-puff.show {
    filter: blur(0);
    opacity: 1;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(-100% - var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  .tooltip.bottom.animation-puff {
    filter: blur(2px);
    opacity: 0;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(100% + var(--tooltip-offset-y))
      )
      scale(2, 2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      filter 0.25s ease-in-out,
      transform 0.25s ease-in-out;
  }

  .tooltip.bottom.animation-puff.show {
    filter: blur(0);
    opacity: 1;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(100% + var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  /* Bounce */

  .tooltip.left.animation-bounce {
    opacity: 0;
    transform: translate(
        calc(-100% - var(--tooltip-offset-x)),
        calc(-50% + var(--tooltip-offset-y))
      )
      scale(1.2, 1.2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      transform 0.25s cubic-bezier(0.5, -1, 0.5, 3);
  }

  .tooltip.left.animation-bounce.show {
    opacity: 1;
    transform: translate(
        calc(-100% - var(--tooltip-offset-x)),
        calc(-50% + var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  .tooltip.right.animation-bounce {
    opacity: 0;
    transform: translate(
        calc(100% + var(--tooltip-offset-x)),
        calc(-50% + var(--tooltip-offset-y))
      )
      scale(1.2, 1.2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      transform 0.25s cubic-bezier(0.5, -1, 0.5, 3);
  }

  .tooltip.right.animation-bounce.show {
    opacity: 1;
    transform: translate(
        calc(100% + var(--tooltip-offset-x)),
        calc(-50% + var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  .tooltip.top.animation-bounce {
    opacity: 0;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(-100% - var(--tooltip-offset-y))
      )
      scale(1.2, 1.2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      transform 0.25s cubic-bezier(0.5, -1, 0.5, 3);
  }

  .tooltip.top.animation-bounce.show {
    opacity: 1;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(-100% - var(--tooltip-offset-y))
      )
      scale(1, 1);
  }

  .tooltip.bottom.animation-bounce {
    opacity: 0;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(100% + var(--tooltip-offset-y))
      )
      scale(1.2, 1.2);
    transform-origin: 50% 50%;
    transition:
      opacity 0.25s ease-in-out,
      transform 0.25s cubic-bezier(0.5, -1, 0.5, 3);
  }

  .tooltip.bottom.animation-bounce.show {
    opacity: 1;
    transform: translate(
        calc(-50% + var(--tooltip-offset-x)),
        calc(100% + var(--tooltip-offset-y))
      )
      scale(1, 1);
  }
</style>