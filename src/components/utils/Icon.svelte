<script lang="ts">
  import type { IconifyIcon } from "@iconify/types";

  type Props = {
    icon: IconifyIcon;
    width?: string;
    height?: string;
    class?: string;
  }

  let {
    icon,
    width = "1rem",
    height = "1rem",
    class: className
  }: Props = $props();
</script>

<svg {width} {height} class={className} viewBox="0 0 {icon.width} {icon.height}">
  {@html icon.body}
</svg>