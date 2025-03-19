<script lang="ts">
  import type { IGDBRelatedGame } from "@types";

  type Props = {
    label: string,
    value: IGDBRelatedGame[],
  }

  let { label, value }: Props = $props();
</script>

<div class="related-game-table-row">
  <div class="inner-row">
    <div class="label">{label}</div>
    <div class="value">
      {#if value.length > 0}
        {#each value as game}
          <a class="game-link" href="https://www.igdb.com/games/{game.slug}" target="noreferrer noopenner">
            {game.name}
          </a>
        {/each}
      {:else}
        None
      {/if}
    </div>
  </div>
</div>

<style>
  .related-game-table-row {
    width: inherit;

    border-bottom: 1px solid rgb(var(--m3-scheme-surface-container-highest));
  }

  .inner-row {
    padding: 0.5rem;
    width: calc(100% - 1rem);

    display: flex;
    justify-content: space-between;
  }

  .label {
    font-weight: bold;
  }
  
  .value {
    color: rgb(var(--m3-scheme-on-surface-variant));
    
    max-width: 12rem;

    display: flex;
    flex-direction: column;
  }

  .game-link {
    text-overflow: ellipsis;
    text-wrap: nowrap;
    overflow: hidden;
  }

  .game-link:hover {
    text-decoration: underline;
  }
</style>