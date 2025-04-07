<script lang="ts">
    import { Icon } from "@component-utils";
    import { ChevronRight } from "@icons";
    import { CardClickable } from "@layout";
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
  <CardClickable type="filled" onclick={toggle} extraOptions={{ style: "width: 100%;" }}>
    <div class="label m3-font-title-medium">
      <Icon icon={ChevronRight} width="1.25rem" height="1.25rem" />
      {label}
    </div>
    {#if isOpen}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="content" onclick={stopImmediatePropagation()} transition:slide={{ duration: 300 }}>
        {@render children()}
      </div>
    {/if}
  </CardClickable>
</div>

<style>
  .accordion {
    width: 100%;
  }

  .content {
    padding: 0px 6px;
    padding-top: 1rem;
  }

  .label {
    display: flex;
    align-items: center;
  }

	.accordion :global(svg) { transition: transform 0.2s ease-in; margin-right: 3px; }
	.accordion[aria-expanded=true] :global(svg) { transform: rotate(0.25turn); }

  .accordion {
    --m3-util-background: var(--m3-scheme-surface-container-highest);
  }
</style>