<script lang="ts">
  import { Button, DateField } from "@interactables";
  import { BottomSheet } from "@layout";
  import { showReleaseDateFilterSheet } from "@stores/Sheets";
  import { searchFilters } from "@stores/State";
  import { search } from "@utils";
  
  let startReleaseDate = $state<string>("");
  let endReleaseDate = $state<string>("");

  let open = $state(true);

  function onSubmit() {
    search({
      ...$searchFilters,
      startReleaseDate: startReleaseDate,
      endReleaseDate: endReleaseDate,
    });

    open = false;
  }

  function onClose() {
    $showReleaseDateFilterSheet = false;
  }
</script>

<BottomSheet padding="0rem 1rem 1rem 1rem" open={open} onclose={onClose}>
  <div class="body">
    <div class="label m3-font-title-large">Release Date Range</div>
    <DateField
      name="Start Date"
      bind:date={startReleaseDate}
    />
    <DateField
      name="End Date"
      bind:date={endReleaseDate}
    />
    <div class="actions">
      <Button type="outlined" onclick={() => open = false}>Cancel</Button>
      <Button type="tonal" onclick={onSubmit}>Search</Button>
    </div>
  </div>
</BottomSheet>

<style>
  .body {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .label {
    text-align: center;

    padding: 1rem 0;

    padding-bottom: 0;
  }

  .actions {
    margin-top: 0.5rem;
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
  }
</style>