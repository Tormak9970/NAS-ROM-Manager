<script lang="ts">
  import { themePrimaryColor } from "@stores/State";

  const colorPresets = ["#51dd28", "#dd5527", "#4f5add", "#a74bf2", "#ffd28d"];
</script>

<div class="settings-entry">
  <div class="color-presets">
    {#each colorPresets as hex}
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions a11y-click-events-have-key-events a11y-no-static-element-interactions -->
      <div class="preset" onclick={() => $themePrimaryColor = hex}>
        <div class="preset-color" style:background={hex}></div>
      </div>
    {/each}
  </div>
</div>

<style>
  .color-presets {
    width: 100%;

    padding: 0rem 1rem;

    display: flex;
    align-items: center;
    gap: 0.75rem;

    height: 30px;
  }

  .preset {
		position: relative;
		display: block;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 30px;

    cursor: pointer;
	}

	.preset-color {
		position: absolute;
		width: 30px;
		height: 30px;
		border-radius: 50%;
		user-select: none;
	}
</style>