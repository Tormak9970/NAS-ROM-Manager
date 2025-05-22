<script lang="ts">
  import type { IconifyIcon } from "@iconify/types";
  import Chip from "./Chip.svelte";

  type Props = {
    options: { label: string; value: string; icon?: IconifyIcon }[];
    chosenOptions?: string[];
    type: "input" | "assist" | "general";
  }

  let {
    options,
    chosenOptions = $bindable([]),
    type
  }: Props = $props();
</script>

<div class="m3-container">
  {#each options as option}
    <Chip
      type={type}
      selected={chosenOptions.includes(option.value)}
      onclick={() =>
        chosenOptions.includes(option.value)
          ? (chosenOptions = chosenOptions.filter((o) => o != option.value))
          : (chosenOptions = [...chosenOptions, option.value])}
    >
      {option.label}
    </Chip>
  {/each}
</div>

<style>
  .m3-container {
    display: flex;
    width: fit-content;
    gap: 0.5rem;
  }
</style>