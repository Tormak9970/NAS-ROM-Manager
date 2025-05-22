<script lang="ts">
  import { Icon } from "@component-utils";
  import type { IconifyIcon } from "@iconify/types";
  import type { Snippet } from "svelte";

  type Props = {
    icon?: IconifyIcon | "space" | undefined;
    disabled?: boolean;
    children: Snippet;
    onclick?: () => void;
  }

  let {
    icon = undefined,
    disabled = false,
    children,
    onclick,
  }: Props = $props();
</script>

<button class="item m3-font-label-large font-body" {disabled} {onclick}>
  {#if icon == "space"}
    <span class="icon"></span>
  {:else if icon}
    <span class="icon">
      <Icon {icon} />
    </span>
  {/if}
  {@render children()}
</button>

<style>
  .item {
    display: flex;
    align-items: center;
    height: 3rem;
    padding: 0 0.75rem;
    white-space: nowrap;
    width: calc(100% - 1.5rem);

    border: none;
    background-color: transparent;
    color: rgb(var(--m3-scheme-on-surface));
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }
  .icon {
    width: 1.5rem;
    height: 1.5rem;
    margin-right: 0.75rem;
  }
  .icon > :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
    color: rgb(var(--m3-scheme-on-surface-variant));
  }

  @media (hover: hover) {
    .item:enabled:hover {
      background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
    }
  }
  .item:enabled:active,
  .item:enabled:focus-visible {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }
  .item:disabled {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
    cursor: auto;
  }
  .item:disabled > .icon > :global(svg) {
    color: rgb(var(--m3-scheme-on-surface) / 0.38);
  }
</style>