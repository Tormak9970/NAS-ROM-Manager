<script lang="ts">
  import Button from "../Button.svelte";

  type Props = {
    clearable: boolean;
    chosenDate: boolean;
    onclear: () => void;
    oncancel: () => void;
    onok: () => void;
  }

  let { clearable, chosenDate, onclear, oncancel, onok }: Props = $props();
</script>

<div class="m3-container">
  {#if clearable}
    <span class="left">
      <Button type="text" onclick={onclear} extraOptions={{ type: "button" }}>
        Clear
      </Button>
    </span>
  {/if}
  <Button type="text" onclick={oncancel} extraOptions={{ type: "button" }}>
    Cancel
  </Button>
  <Button
    type="text"
    disabled={!clearable && !chosenDate}
    onclick={onok}
    extraOptions={{ type: "button" }}
  >
    OK
  </Button>
</div>

<style>
  .m3-container {
    display: flex;
    padding: 0.5rem 0.75rem;
    gap: 0.5rem;
    justify-content: flex-end;
  }
  .m3-container > .left {
    margin-right: auto;
  }
</style>