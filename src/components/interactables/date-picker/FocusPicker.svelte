<script lang="ts">
  import { Icon } from "@component-utils";
  import { Checkmark } from "@icons";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  const conditionalScroll = (node: Element, shouldScroll: boolean) => {
    if (shouldScroll) node.scrollIntoView({ block: "nearest" });
  };
  type Props = {
    options: {
      id: number;
      name: string;
      selected: boolean;
    }[];
    onchosen: (id: number) => void;
  }

  let { options, onchosen }: Props = $props();
</script>

<div class="m3-container">
  {#each options as { id, name, selected }}
    <button
      class="m3-font-body-large"
      onclick={() => onchosen(id)}
      use:conditionalScroll={selected}
    >
      {#if selected}
        <Icon icon={Checkmark} />
      {/if}
      {name}
    </button>
  {/each}
</div>

<style>
  .m3-container {
    display: flex;
    flex-direction: column;
    flex: 1 0;
    overflow: auto;
    margin-bottom: 1.25rem;
  }

  button {
    display: inline-flex;
    align-items: center;
    height: 3rem;
    padding-left: 3.5rem;
    flex-shrink: 0;

    background-color: transparent;
    color: rgb(var(--m3-scheme-on-surface));
    border: none;
    position: relative;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }
  button :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
    position: absolute;
    left: 1rem;
    top: 50%;
    transform: translateY(-50%);
  }

  button:hover {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.08);
  }

  button:focus-visible,
  button:active {
    background-color: rgb(var(--m3-scheme-on-surface) / 0.12);
  }
</style>