<script lang="ts">
  type Props = {
    label: string,
    value: string,
  }

  let { label, value }: Props = $props();
</script>

<div class="table-row">
  <div class="inner-row">
    <div class="label">{label}</div>
    <div class="value">{value}</div>
  </div>
</div>

<style>
  .table-row {
    width: inherit;

    border-bottom: 2px solid rgb(var(--m3-scheme-outline));
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
  }
</style>