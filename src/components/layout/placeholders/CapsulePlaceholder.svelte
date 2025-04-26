<script lang="ts">
  import { Icon } from "@component-utils";
  import { Landscape } from "@icons";
</script>

<div class="placeholder">
  <Icon icon={Landscape} height="2rem" width="2rem" />
</div>

<style>
  .placeholder {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    /* TODO: set background and svg color */
  }
</style>