<script lang="ts">
  import { isLandscapeRoutes, showEditRouteOrderModal } from "@stores/Modals";

  import { Icon, ModalBody } from "@component-utils";
  import { Button, Checkbox } from "@interactables";
  
  import { routes } from "$lib/routes";
  import { DragIndicator } from "@icons";
  import { landscapeViews, portraitViews, showWarningSnackbar } from "@stores/State";
  import { clamp, swap } from "@utils";
  import { drag } from "svelte-gesture";

  let open = $state(true);

  const entryHeight = 40;
  
  let selectedViews = $state($isLandscapeRoutes ? $landscapeViews : $portraitViews);
  const initialRoutes = Object.keys(routes).sort((a, b) => {
    const aIndex = selectedViews.indexOf(a);
    const bIndex = selectedViews.indexOf(b);

    if (aIndex === -1 && bIndex === -1) {
      return a < b ? -1 : 1;
    }
    
    if (aIndex !== -1 && bIndex !== -1) {
      return aIndex - bIndex;
    }

    return aIndex !== -1 ? -1 : 1;
  });

  let routeList = $state(initialRoutes);
  let newOrder = $state(initialRoutes.map((_, i) => i));
  let checkDict = $state(Object.fromEntries(initialRoutes.map((route) => [route, selectedViews.includes(route)])) as Record<string, boolean>);

  let reset = $state(false);

  let draggingIndex = $state(-1);
  let dragHeight = $state(0);

  function getDragHandler(originalIndex: number) {
    return ({ detail }: any) => {
      const { active, movement: [_, y] } = detail;

      draggingIndex = originalIndex;
      const curIndex = newOrder.indexOf(originalIndex);
      const curRow = clamp(Math.round((originalIndex * entryHeight + y) / entryHeight), 0, routeList.length - 1);
      newOrder = swap(newOrder, curIndex, curRow);
      
      dragHeight = y;

      if (!active) {
        draggingIndex = -1;
        routeList = newOrder.map((index) => routeList[index]);
        newOrder = routeList.map((_, i) => i);
        dragHeight = 0;
      }
    }
  }

  /**
   * Handles checking if a checkbox can be checked/unchecked.
   * @param route The route to set the checked status of.
   */
  function checkboxHandler(route: string) {
    return (e: Event) => {
      const checked = (e.currentTarget as HTMLInputElement).checked;
      const numChecked = routeList.filter((route) => checkDict[route]).length;
      
      if (numChecked === 3 && !checked) {
        $showWarningSnackbar({ message: "You must have at least 3 routes!" });
        reset = !reset;
      } else if (numChecked === 5 && checked && !$isLandscapeRoutes) {
        $showWarningSnackbar({ message: "You can't have more than 5 routes!" });
        reset = !reset;
      } else {
        checkDict[route] = checked;
      }
      
      checkDict = { ...checkDict };
    }
  }

  /**
   * Saves the user's changes
   */
  function done() {
    if($isLandscapeRoutes) {
      $landscapeViews = routeList.filter((route) => checkDict[route]); 
    } else {
      $portraitViews = routeList.filter((route) => checkDict[route]);
    }
    
    open = false;
  }
</script>

<ModalBody maxWidth="25rem" open={open} headline={`Edit ${$isLandscapeRoutes ? "Desktop" : "Mobile"} Route Order`} oncloseend={() => $showEditRouteOrderModal = false}>
  <div>
    {#key reset}
      <div class="drag-container" style:height="{routeList.length * entryHeight}px">
        {#each routeList as view, i (view)}
          <div
            class="entry"
            class:being-dragged={draggingIndex === i}
            style:top="{draggingIndex === i ? i * entryHeight + dragHeight : newOrder.indexOf(i) * entryHeight}px"
          >
            <div class="left">
              <div class="checkbox-container">
                <Checkbox checked={checkDict[view]} oninput={checkboxHandler(view)} />
              </div>
              <div class="font-label">{view}</div>
            </div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <!-- svelte-ignore event_directive_deprecated -->
            <div class="handle" use:drag ondrag={getDragHandler(i)}>
              <Icon icon={DragIndicator} height="30px" width="24px" />
            </div>
          </div>
        {/each}
      </div>
    {/key}
  </div>
  {#snippet buttons()}
    <div>
      <Button type="tonal" onclick={() => open = false }>Cancel</Button>
      <Button type="tonal" onclick={done}>Save</Button>
    </div>
  {/snippet}
</ModalBody>

<style>
  .drag-container {
    margin-left: -1rem;
		width: calc(100% + 2rem);

    position: relative;
  }

  .entry {
    position: absolute;
    width: 100%;
    height: 40px;

    padding-left: 1rem;

    display: flex;
    align-items: center;
    justify-content: space-between;

    user-select: none;

    background-color: rgb(var(--m3-scheme-surface-container));

    transition: background-color 0.2 ease-out;
    border-radius: 4px;
    
    transition: top 0.3s ease-out, scale 0.3s ease-out;

    z-index: 1;
    scale: 1;
	}

  .being-dragged {
    z-index: 2;
    box-shadow: var(--m3-util-elevation-2);

    background-color: rgb(var(--m3-scheme-surface-container-high));

    transition: scale 0.3s ease-out;
    scale: 1.05
  }

  .left {
    height: 100%;
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .checkbox-container,
  .handle {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .handle { padding-right: 0.75rem; touch-action: none; cursor: grab; }
  .handle:active { cursor: grabbing; }
</style>