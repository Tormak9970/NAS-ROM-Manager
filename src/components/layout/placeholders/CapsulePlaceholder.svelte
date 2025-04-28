<script lang="ts">
  import { Icon } from "@component-utils";
  import { Landscape } from "@icons";
</script>

<div class="capsule-placeholder">
  <Icon icon={Landscape} height="2rem" width="2rem" />
</div>

<style>
  .capsule-placeholder {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    background-color: rgb(var(--m3-scheme-surface-container));
    color: rgb(var(--m3-scheme-secondary-container));
  }
</style>