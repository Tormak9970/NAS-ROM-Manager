<script lang="ts">
  import { Icon } from "@component-utils";
  import type { ContextMenuItem } from "@directives";
  import type { IconifyIcon } from "@iconify/types";
  import { contextMenuItems, contextMenuPosition, showContextMenu } from "@stores/ContextMenu";
  import { stopImmediatePropagation } from "@utils";
  import type { HTMLAttributes } from "svelte/elements";
  import Button from "./Button.svelte";

  type Props = {
    extraOptions?: HTMLAttributes<HTMLButtonElement>;
    size?: string;
    iconSize?: string;
    width?: string;
    height?: string;
    icon: IconifyIcon;
    menuWidth?: number;
    items: ContextMenuItem[];
  }

  let {
    extraOptions = {},
    size = "2.5rem",
    iconSize = "1.5rem",
    width = "36px",
    height = "36px",
    icon,
    items,
    menuWidth = 122
  }: Props = $props();

  let buttonElement: any = $state();

  function onButtonClick() {
    $contextMenuItems = items;

    const buttonBB = buttonElement.getButtonElement().getBoundingClientRect() as DOMRect;

    $contextMenuPosition = {
      x: buttonBB.x + buttonBB.width - menuWidth,
      y: buttonBB.y + buttonBB.height,
    };

    $showContextMenu = true;
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div onclick={stopImmediatePropagation()} onmousedown={stopImmediatePropagation()}>
  <Button
    type="text"
    iconType="full"
    size={size}
    iconSize={iconSize}
    onclick={onButtonClick}
    extraOptions={extraOptions}
    bind:this={buttonElement}
  >
    <Icon icon={icon} width={width} height={height} />
  </Button>
</div>