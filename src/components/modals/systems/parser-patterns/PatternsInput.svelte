<script lang="ts">
  import type { ParserPattern } from "@types";
  import Parser from "./Parser.svelte";

  type Props = {
    patterns: ParserPattern[];
  }

  let {
    patterns = $bindable()
  }: Props = $props();
</script>

<div class="patterns-input">
  <div class="patterns">
    {#each patterns as _, i}
      <Parser label={`Parser ${i + 1}`} bind:pattern={patterns[i]} />
    {/each}
  </div>
  <!-- TODO: add button -->
</div>

<style>
  
</style>