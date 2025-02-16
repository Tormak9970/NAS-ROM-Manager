<script lang="ts">
  type Props = {
    label: string;
    value: string | number;
  }

  let { label, value}: Props = $props();
</script>

<li>
  <b>{label}</b>
  <div>{value}</div>
</li>

<style>
  li {
    width: 100%;

    margin-bottom: 0.25rem;

    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  b {
    color: rgb(var(--m3-scheme-on-surface) / 0.8);
  }

  div {
    color: rgb(var(--m3-scheme-primary));
  }
</style>