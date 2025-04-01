<script lang="ts" module>
  type CacheEntry = {
    start: number;
    end: number;
    heightMap: any[];
    top: number;
    bottom: number;
    listScrollTop: number;
  };
  
  let virtualListCache: Record<string, CacheEntry> = {};

  function getCacheEntry(name: string, saveState: boolean): CacheEntry {
    if (!virtualListCache[name] || !saveState) {
      virtualListCache[name] = {
        start: 0,
        end: 0,
        heightMap: [],
        top: 0,
        bottom: 0,
        listScrollTop: 0
      }
    }

    return virtualListCache[name];
  }
</script>

<script lang="ts">
	import { scrollShadow } from "@directives";
	import { onMount, tick, type Snippet } from "svelte";
  
  interface Props {
    name: string;
    items: any[];
    height?: string;
    width?: string;
    itemHeight: number;
    isScrolled?: boolean;
    saveState?: boolean;
    row: Snippet<[any]>;
    keyFunction?: (entry: any) => any;
  }

  let {
    name,
    items,
    height = "100%",
    width = "100%",
    itemHeight,
    isScrolled = $bindable(true),
    saveState = true,
    row,
    keyFunction = (entry: any) => entry.index
  }: Props = $props();

  let cacheEntry = $state(getCacheEntry(name, saveState));
  
  // * Whenever `items` changes, invalidate the current heightmap.
  $effect(() => {
    if (mounted) refresh(items, viewportHeight, itemHeight);
  });

  let mounted: boolean = $state(false);
  let entries: HTMLCollectionOf<HTMLElement>;
  const visible: any[] = $derived(items.slice(cacheEntry.start, cacheEntry.end).map((data, i) => {
    return { index: i + cacheEntry.start, data };
  }));

  // @ts-expect-error This is always assigned on mount.
  let viewport: HTMLElement = $state();
  let viewportHeight = $state(0);

  // @ts-expect-error This is always assigned on mount.
  let contents: HTMLElement = $state();

	let averageHeight: number;

  /**
   * Refreshes the contents of the virtual list.
   * @param items The items to render.
   * @param viewportHeight The viewport height.
   * @param itemHeight The height of the elements being rendered.
   */
	async function refresh(items: any[], viewportHeight: number, itemHeight: number) {
    if (!viewport) return;
    
		const { scrollTop } = viewport;

    // * Wait until the DOM is up to date.
		await tick();

		let contentHeight = cacheEntry.top - scrollTop;
		let i = cacheEntry.start;

		while (contentHeight < viewportHeight && i < items.length) {
			let row = entries[i - cacheEntry.start];

			if (!row) {
				cacheEntry.end = i + 1;
        // * Render the newly visible entry.
				await tick();
				row = entries[i - cacheEntry.start];
			}

			const rowHeight = cacheEntry.heightMap[i] = itemHeight || row.offsetHeight;
			contentHeight += rowHeight;
			i++;
		}

		cacheEntry.end = i;

		const remaining = items.length - cacheEntry.end;
		averageHeight = (cacheEntry.top + contentHeight) / cacheEntry.end;

		cacheEntry.bottom = remaining * averageHeight;
		cacheEntry.heightMap.length = items.length;

    virtualListCache = { ...virtualListCache }
	}

  /**
   * Handles when the virtual list is scrolled.
   */
	async function handleScroll() {
		const { scrollTop } = viewport;
    cacheEntry.listScrollTop = scrollTop;
    isScrolled = scrollTop !== 0;

		const oldStart = cacheEntry.start;

		for (let v = 0; v < entries.length; v += 1) {
			cacheEntry.heightMap[cacheEntry.start + v] = itemHeight || entries[v].offsetHeight;
		}

		let i = 0;
		let y = 0;

		while (i < items.length) {
			const rowHeight = cacheEntry.heightMap[i] || averageHeight;

			if (y + rowHeight > scrollTop) {
				cacheEntry.start = i;
				cacheEntry.top = y;

				break;
			}

			y += rowHeight;
			i++;
		}

		while (i < items.length) {
			y += cacheEntry.heightMap[i] || averageHeight;
			i++;

			if (y > scrollTop + viewportHeight) break;
		}

		cacheEntry.end = i;

		const remaining = items.length - cacheEntry.end;
		averageHeight = y / cacheEntry.end;

		while (i < items.length) {
      cacheEntry.heightMap[i++] = averageHeight;
    }

		cacheEntry.bottom = remaining * averageHeight;

		// * Prevent jumping if we scrolled up into unknown territory.
		if (cacheEntry.start < oldStart) {
			await tick();

			let expectedHeight = 0;
			let actualHeight = 0;

			for (let i = cacheEntry.start; i < oldStart; i +=1) {
				if (entries[i - cacheEntry.start]) {
					expectedHeight += cacheEntry.heightMap[i];
					actualHeight += itemHeight || entries[i - cacheEntry.start].offsetHeight;
				}
			}

			const d = actualHeight - expectedHeight;
			viewport.scrollTo(0, scrollTop + d);
		}
    
    virtualListCache = { ...virtualListCache }
	}

	// * Trigger initial refresh.
	onMount(() => {
		entries = contents.getElementsByTagName("svelte-virtual-list-row") as HTMLCollectionOf<HTMLElement>;
    viewport.scrollTo(0, cacheEntry.listScrollTop);
		mounted = true;
	});
</script>

<div style="width: {width}; height: {height};">
  <svelte-virtual-list-viewport
    style="height: {height};"
    class="styled-scrollbar"
    use:scrollShadow={{ background: "--m3-scheme-background" }}
    onscroll={handleScroll}
    bind:offsetHeight={viewportHeight}
    bind:this={viewport}
  >
    <svelte-virtual-list-contents
      style="padding-top: {cacheEntry.top}px; padding-bottom: {cacheEntry.bottom + 60 + 2}px;"
      bind:this={contents}
    >
      {#each visible as entry (keyFunction(entry))}
        <svelte-virtual-list-row>
          {@render row(entry.data)}
        </svelte-virtual-list-row>
      {/each}
    </svelte-virtual-list-contents>
  </svelte-virtual-list-viewport>
</div>

<style>
	svelte-virtual-list-viewport {
		position: relative;
		overflow-y: auto;
    overflow-x: hidden;
		display: block;
	}

	svelte-virtual-list-contents {
		display: block;
	}

	svelte-virtual-list-row {
    margin-left: 10px;
    margin-right: 10px;
		display: flex;
	}
</style>