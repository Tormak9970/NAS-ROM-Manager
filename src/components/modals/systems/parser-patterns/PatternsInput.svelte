<script lang="ts">
  import { Button } from "@interactables";
  import type { ParserPattern } from "@types";
  import Parser from "./Parser.svelte";

  type Props = {
    patterns: ParserPattern[];
  }

  let {
    patterns = $bindable()
  }: Props = $props();

  let canAdd = $state(false);

  function addEmptyParser() {
    
  }
</script>

<div class="patterns-input">
  <div class="patterns">
    {#each patterns as _, i}
      <Parser label={`Parser ${i + 1}`} bind:pattern={patterns[i]} />
    {/each}
  </div>
  <!-- TODO: add button -->
  <Button type="tonal" onclick={addEmptyParser} disabled={!canAdd}>Create</Button>
</div>

<style>
  
</style>