<script lang="ts">
  import { SportsEsports } from "@icons";
  import Icon from "./Icon.svelte";

  type Props = {
    name: string;
    index: number;
    selected: boolean;
    onSelect: () => void;
  }

  let { name, index, selected, onSelect }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore event_directive_deprecated -->
<div
  class="entry"
  class:even={index % 2 === 0}
  class:selected
  onclick={onSelect}
>
  <div class="name">{name}</div>
  <Icon icon={SportsEsports} />
</div>

<style>
  .entry {
    width: 100%;

    display: flex;
    align-items: center;
    justify-content: space-between;

    padding: 0.5rem;

    background-color: rgb(var(--m3-scheme-surface-container-low) / 0.4);

    transition: background-color 0.2s;

    cursor: pointer;

    border-bottom: 1px solid rgb(var(--m3-scheme-surface-container-high));
  }
  .entry:hover {
    background-color: rgb(var(--m3-scheme-surface-container-high));
  }

  .entry.selected {
    background-color: rgb(var(--m3-scheme-secondary-container));
  }

  .name {
    max-width: 90%;
    overflow: hidden;
    text-overflow: ellipsis;
    text-wrap: nowrap;
  }

  .even {
    background-color: rgb(var(--m3-scheme-surface-container-low));
  }
</style>