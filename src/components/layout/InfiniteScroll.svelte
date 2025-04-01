<script lang="ts">
  import { onDestroy } from "svelte";

  type Props = {
    threshold?: number;
    horizontal?: boolean;
    hasMore?: boolean;
    loadMore: () => void;
  };

  let {
    threshold = 0,
    horizontal = false,
    hasMore = true,
    loadMore,
  }: Props = $props();

  let isLoadMore = $state(false);
  let component: any = $state();
  let listening = $state(false);

  $effect(() => {
    if (component && !listening) {
      const element = component.parentNode;

      element.addEventListener("scroll", onScroll);
      element.addEventListener("resize", onScroll);

      listening = true;
    }
  });

  function onScroll(e: any) {
    const offset = horizontal
      ? e.target.scrollWidth - e.target.clientWidth - e.target.scrollLeft
      : e.target.scrollHeight - e.target.clientHeight - e.target.scrollTop;

    if (offset <= threshold) {
      if (!isLoadMore && hasMore) {
        loadMore();
      }
      isLoadMore = true;
    } else {
      isLoadMore = false;
    }
  }

  onDestroy(() => {
    if (component) {
      const element = component.parentNode;

      element.removeEventListener("scroll", null);
      element.removeEventListener("resize", null);
    }
  });
</script>

<div bind:this={component} style="width:0px"></div>