<script lang="ts">
  import { SportsEsports } from "@icons";
  import { createEventDispatcher } from "svelte";
  import Icon from "./Icon.svelte";

  export let name: string;
  export let index: number;

  const dispatch = createEventDispatcher();

  function select() {
    dispatch("select");
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore event_directive_deprecated -->
<div
  class="entry"
  class:even={index % 2 === 0}
  on:click={select}
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

    background-color: rgb(var(--m3-scheme-surface-container) / 0.6);

    transition: background-color 0.2s;

    cursor: pointer;

    border-bottom: 1px solid rgb(var(--m3-scheme-surface-container-highest));
  }
  .entry:hover {
    background-color: rgb(var(--m3-scheme-surface-container-highest));
  }

  .name {
    max-width: 90%;
    overflow: hidden;
    text-overflow: ellipsis;
    text-wrap: nowrap;
  }

  .even {
    background-color: rgb(var(--m3-scheme-surface-container));
  }
</style>