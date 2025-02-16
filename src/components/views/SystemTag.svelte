<script lang="ts">
  import { systems } from "@stores/State";
  import Tag from "@views/Tag.svelte";

  let { system }: { system: string } = $props();

  let systemTagConfig = $derived($systems[system].tagConfig);

  const DEFAULT_COLOR = "var(--m3-scheme-tertiary-container)";
</script>

<Tag
  backgroundColor={systemTagConfig?.backgroundColor ?? DEFAULT_COLOR}
  borderColor={systemTagConfig?.borderColor ?? DEFAULT_COLOR}
  isUppercase
>
  <b>{system}</b>
</Tag>