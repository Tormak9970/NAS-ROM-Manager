<script lang="ts">
  type Props = {
    disabled?: boolean;
    today?: boolean;
    selected?: boolean;
    label: string;
    onclick: () => {};
  }

  let {
    disabled = false,
    today = false,
    selected = false,
    label,
    onclick
  }: Props = $props();
</script>

<button
  {disabled}
  class="item m3-font-body-small"
  type="button"
  class:today
  class:selected
  {onclick}
>
  {label}
</button>

<style>
  .item {
    display: inline-flex;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 2.5rem;
    align-items: center;
    justify-content: center;

    --text: var(--m3-scheme-on-surface);
    background-color: transparent;
    color: rgb(var(--text));
    border: none;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }
  button:enabled:hover {
    background-color: rgb(var(--text) / 0.08);
  }
  button:enabled:focus-visible,
  button:enabled:active {
    background-color: rgb(var(--text) / 0.12);
  }
  button:disabled {
    cursor: auto;
    color: rgb(var(--m3-scheme-on-surface-variant) / 0.38);
  }
  .today {
    background-color: rgb(var(--m3-scheme-primary) / 0.32);
  }
  .selected {
    background-color: rgb(var(--m3-scheme-primary));
    --text: var(--m3-scheme-on-primary);
  }
  .selected:enabled:hover {
    background-color: rgb(var(--m3-scheme-primary));
    --text: var(--m3-scheme-on-primary);
  }
</style>