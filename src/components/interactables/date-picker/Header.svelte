<script lang="ts">
  import { Icon } from "@component-utils";
  import { ArrowDropDown, ChevronLeft, ChevronRight } from "@icons";

  type Props = {
    currentView: "calendar" | "year" | "month";
    focusedMonth: number;
    focusedYear: number;
    startYear: number;
    endYear: number;
  }

  let {
    currentView = $bindable(),
    focusedMonth = $bindable(),
    focusedYear = $bindable(),
    startYear,
    endYear
  }: Props = $props();

  const yearClick = () => (currentView = currentView == "calendar" ? "year" : "calendar");
  const monthClick = () => (currentView = currentView == "calendar" ? "month" : "calendar");
  const getShortMonth = (month: number) =>
    new Date(0, month).toLocaleDateString(undefined, { month: "short" });
</script>

<div class="m3-container" class:choosing={currentView != "calendar"}>
  <div>
    <button
      type="button"
      class="arrow"
      onclick={() => (focusedMonth = (focusedMonth - 1 + 12) % 12)}
    >
      <Icon icon={ChevronLeft} />
    </button>
    <button
      type="button"
      class="chooser m3-font-label-large"
      onclick={monthClick}
      disabled={currentView == "year"}
    >
      {getShortMonth(focusedMonth)}
      <Icon icon={ArrowDropDown} />
    </button>
    <button type="button" class="arrow" onclick={() => (focusedMonth = (focusedMonth + 1) % 12)}>
      <Icon icon={ChevronRight} />
    </button>
  </div>
  <div>
    <button
      type="button"
      class="arrow"
      disabled={focusedYear <= startYear}
      onclick={() => focusedYear--}
    >
      <Icon icon={ChevronLeft} />
    </button>
    <button
      type="button"
      class="chooser m3-font-label-large"
      onclick={yearClick}
      disabled={currentView == "month"}
    >
      {focusedYear}
      <Icon icon={ArrowDropDown} />
    </button>
    <button
      type="button"
      class="arrow"
      disabled={focusedYear >= endYear}
      onclick={() => focusedYear++}
    >
      <Icon icon={ChevronRight} />
    </button>
  </div>
</div>

<style>
  .m3-container {
    display: flex;
    height: 4rem;
    transition: all 200ms;
    border-bottom: solid 0.0625rem transparent;
    flex-shrink: 0;
  }
  .m3-container > div {
    display: flex;
    flex: 1;
  }

  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;

    background-color: transparent;
    color: rgb(var(--m3-scheme-on-surface-variant));
    border: none;
    padding: 0;
    transition: all 200ms;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
  }
  button:enabled:hover {
    background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.08);
  }
  button:enabled:focus-visible,
  button:enabled:active {
    background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.12);
  }
  button:disabled {
    cursor: auto;
    color: rgb(var(--m3-scheme-on-surface-variant) / 0.38);
  }

  .chooser {
    flex-grow: 1;
  }
  .chooser :global(svg) {
    width: 1.125rem;
    height: 1.125rem;
    margin-right: -0.25rem;
    margin-left: 0.5rem;
  }
  .arrow {
    width: 3rem;
  }
  .arrow :global(svg) {
    width: 1.5rem;
    height: 1.5rem;
  }

  .choosing {
    border-color: rgb(var(--m3-scheme-outline-variant));
  }
  .choosing .arrow {
    opacity: 0;
    width: 0;
  }
</style>