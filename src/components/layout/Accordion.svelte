<script lang="ts">
    import { Icon } from "@component-utils";
    import { ChevronRight } from "@icons";
    import { Card } from "@layout";
    import { stopImmediatePropagation } from "@utils";
    import type { Snippet } from "svelte";
    import { slide } from "svelte/transition";

  type Props = {
    label: string;
    open?: boolean;
    children: Snippet;
  }

  let {
    label,
    open = false,
    children
  }: Props = $props();

	let isOpen = $state(open);
  
	const toggle = () => { isOpen = !isOpen; }
</script>

<div class="accordion" aria-expanded={isOpen}>
  <Card type="filled" extraOptions={{ style: "width: 100%;" }}>
    <button class="header m3-font-title-medium" onclick={toggle}>
      <Icon icon={ChevronRight} width="1.25rem" height="1.25rem" />
      {label}
    </button>
    {#if isOpen}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="drawer" onclick={stopImmediatePropagation()} transition:slide={{ duration: 300 }}>
        {@render children()}
      </div>
    {/if}
  </Card>
</div>

<style>
  .accordion {
    width: 100%;
  }

  .drawer {
    padding: 0px 6px;
    padding-top: 2rem;
  }

  .header {
    display: flex;
    align-items: center;

    padding: 1rem;

    margin: -1rem;

    color: rgb(var(--m3-scheme-on-surface));
    background-color: rgb(var(--m3-scheme-surface-container-highest));

    border: none;
    outline: none;

    transition: background-color 0.2s ease-out;
  }

  .header:hover {
    cursor: pointer;
    background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
  }

	.accordion .header :global(svg) { transition: transform 0.2s ease-in; margin-right: 3px; }
	.accordion[aria-expanded=true] .header :global(svg) { transform: rotate(0.25turn); }

  .accordion {
    --m3-util-background: var(--m3-scheme-surface-container-highest);
  }
</style>