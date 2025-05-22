<script lang="ts">
  import RadioInput from "./RadioInput.svelte";

  type Props = {
    name: string;
    checked?: boolean;
    label: string;
    oninput?: (e: Event) => void;
    onclick?: () => void;
  }

  let { name, checked = false, label, oninput = () => {}, onclick = () => {} }: Props = $props();
</script>

<button class="item m3-font-label-large" disabled {onclick}>
  <!-- svelte-ignore a11y_label_has_associated_control -->
  <label>
    <RadioInput name={name} checked={checked} {oninput} />
    <div class="radio font-label">{label}</div>
  </label>
</button>

<style>
  .item {
    display: flex;
    align-items: center;
    height: 2.5rem;
    padding: 0 0.75rem;
    white-space: nowrap;

    border: none;
    background-color: transparent;
    color: rgb(var(--m3-scheme-on-surface));
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }

  label {
    display: flex;
    align-items: center;
  }
  
  .radio {
    margin-left: 15px;
  }
</style>