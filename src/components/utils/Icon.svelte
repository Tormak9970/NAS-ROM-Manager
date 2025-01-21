<script lang="ts">
  import type { IconifyIcon } from "@iconify/types";
  export let icon: IconifyIcon;
  export let width: string = "1em";
  export let height: string = "1em";
  let className: string | undefined = undefined;
  export { className as class };
</script>

<svg {width} {height} class={className} viewBox="0 0 {icon.width} {icon.height}">
  {@html icon.body}
</svg>