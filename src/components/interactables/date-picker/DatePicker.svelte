<script lang="ts">
  import Actions from "./Actions.svelte";
  import CalendarPicker from "./CalendarPicker.svelte";
  import FocusPicker from "./FocusPicker.svelte";
  import Header from "./Header.svelte";

  const now = new Date();

  type Props = {
    display?: string;
    date?: string;
    clearable: boolean;
    focusedMonth?: number;
    focusedYear?: any;
    startYear?: any;
    endYear?: any;
    dateValidator?: (date: string) => boolean;
    onsetDate?: (date: string) => void;
    onclose?: () => void;
  }

  let {
    display = "flex",
    date = $bindable(""),
    clearable,
    focusedMonth = $bindable(parseInt(date.slice(5, 7)) - 1 || now.getMonth()),
    focusedYear = $bindable(parseInt(date.slice(0, 4)) || now.getFullYear()),
    startYear = now.getFullYear() - 50,
    endYear = now.getFullYear() + 10,
    dateValidator = (date: string) => true,
    onsetDate,
    onclose
  }: Props = $props();

  let currentView: "calendar" | "year" | "month" = $state("calendar");
  let chosenDate: string = $state(date);

  const getLongMonth = (month: number) => new Date(0, month).toLocaleDateString(undefined, { month: "long" });
</script>

<div class="m3-container" style="display: {display};">
  <Header bind:currentView bind:focusedMonth bind:focusedYear {startYear} {endYear} />
  {#if currentView == "calendar"}
    <CalendarPicker {focusedMonth} {focusedYear} {dateValidator} bind:chosenDate />
    <Actions
      {clearable}
      chosenDate={Boolean(chosenDate)}
      onclear={() => (chosenDate = "")}
      oncancel={() => {
        chosenDate = date;
        onclose?.();
      }}
      onok={() => {
        onsetDate?.(chosenDate);
        onclose?.();
      }}
    />
  {:else}
    <FocusPicker
      options={currentView == "month"
        ? Array.from({ length: 12 }, (_, i) => ({
            id: i,
            name: getLongMonth(i),
            selected: i == focusedMonth,
          }))
        : Array.from({ length: endYear - startYear }, (_, i) => ({
            id: startYear + i + 1,
            name: (startYear + i + 1).toString(),
            selected: startYear + i + 1 == focusedYear,
          }))}
      onchosen={(id) => {
        if (currentView == "month") focusedMonth = id;
        else focusedYear = id;
        currentView = "calendar";
      }}
    />
  {/if}
</div>

<style>
  :root {
    --m3-date-picker-shape: var(--m3-util-rounding-large);
  }

  .m3-container {
    position: relative;
    overflow: hidden;
    flex-direction: column;
    background-color: rgb(var(--m3-scheme-surface-container-high));
    width: 20.5rem;
    height: 26.75rem;
    border-radius: var(--m3-date-picker-shape);
  }
</style>