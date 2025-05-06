<script lang="ts">
  import { Icon } from "@component-utils";
  import { Draft, Folder } from "@icons";
  import type { FilePickerEntry } from "@types";

  type Props = {
    entry: FilePickerEntry;
    index: number;
    onSelect: () => void;
  }

  let { entry, index, onSelect }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore event_directive_deprecated -->
<div
  class="entry"
  class:even={index % 2 === 0}
  onclick={onSelect}
>
  <div class="name">{entry.name}</div>
  <Icon icon={entry.isDir ? Folder : Draft} />
</div>

<style>
  .entry {
    width: calc(100% + 8px);

    display: flex;
    align-items: center;
    justify-content: space-between;

    margin-left: -8px;

    padding: 0.5rem;

    background-color: rgb(var(--m3-scheme-surface-container-low) / 0.4);

    transition: background-color 0.2s;

    cursor: pointer;

    border-bottom: 1px solid rgb(var(--m3-scheme-surface-container-high));
  }
  .entry:hover {
    background-color: rgb(var(--m3-scheme-surface-container-high));
  }

  @supports (-moz-appearance:none) {
    .entry {
      width: 100%;
      margin-left: 0;
    }
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