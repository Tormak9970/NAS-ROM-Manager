<script lang="ts">
  import { onMount } from "svelte";

  type Props = {
    /** DOM element of the wrapper element */
    wrapper: any;
    /** indicator of the popup state */
    isOpen: boolean;
    /** if set to true, the wrapper should have a dialog role and be absolute. It should be relative otherwise */
    isDialog: boolean;
    children?: import('svelte').Snippet;
  }

  let {
    wrapper = $bindable(),
    isOpen,
    isDialog,
    children
  }: Props = $props();

  onMount(() => {
    wrapper.anchorElement = wrapper.parentElement?.childNodes[1];
  });
</script>

<md-menu bind:this={wrapper} open={isOpen} positioning="popover" role={isDialog ? 'dialog' : ''} style="--md-menu-container-color: rgb(var(--m3-scheme-surface-container)); --md-menu-item-container-color: rgb(var(--m3-scheme-surface-container)); --md-menu-item-selected-container-color: rgb(var(--m3-scheme-secondary-container));">
  <div class="wrapper" style="padding: 0px 8px;">
    {@render children?.()}
  </div>
</md-menu>