<script lang="ts">
  import { systems } from "@stores/State";
  import { goToSystem, stopPropagation } from "@utils";
  import Tag from "@views/Tag.svelte";

  let { system }: { system: string } = $props();

  const systemTagConfig = $derived($systems[system].tagConfig);

  const DEFAULT_COLOR = "var(--m3-scheme-tertiary-container)";
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<Tag
  backgroundColor={systemTagConfig?.backgroundColor ?? DEFAULT_COLOR}
  borderColor={systemTagConfig?.borderColor ?? DEFAULT_COLOR}
  isUppercase
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore event_directive_deprecated -->
  <b onclick={stopPropagation(() => goToSystem(system))}>{system}</b>
</Tag>

<style>
  b:hover {
    cursor: pointer;
    text-decoration: underline;
  }
</style>