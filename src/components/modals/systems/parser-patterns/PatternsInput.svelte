<script lang="ts">
  import { SystemController } from "@controllers";
  import { Button } from "@interactables";
  import type { ParserPattern } from "@types";
  import { asyncEvery } from "@utils";
  import Parser from "./Parser.svelte";

  type Props = {
    systemFolder: string;
    patterns: ParserPattern[];
  }

  let {
    systemFolder,
    patterns = $bindable()
  }: Props = $props();

  let canAdd = $state(false);

  $effect(() => {
    asyncEvery(patterns, SystemController.validateParserPattern).then((isValid: boolean) => {
      canAdd = isValid;
    });
  });

  function addEmptyParser() {
    patterns.push({
      glob: "*.(?i){nsp,xci,nca}",
      regex: ".+\\/(?<title>.+)\\.(?:nsp|xci|nca)$",
      downloadStrategy: {
        type: "single-file"
      }
    });
  }

  function deleteParser(index: number) {
    patterns.splice(index, 1);
  }
</script>

<div class="patterns-input">
  <div class="header m3-font-title-medium">Parser Patterns</div>
  <div class="patterns">
    {#each patterns as _, i}
      <Parser
        label={`Parser ${i + 1}`}
        bind:pattern={patterns[i]}
        onDelete={() => deleteParser(i)}
        systemFolder={systemFolder}
      />
    {/each}
  </div>
  <Button
    type="tonal"
    onclick={addEmptyParser}
    disabled={!canAdd}
    extraOptions={{ style: "width: 100%" }}
  >
    Add Parser
  </Button>
</div>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;

    margin-bottom: 0.5rem;
  }

  .patterns {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    
    margin-bottom: 0.5rem;
  }
</style>