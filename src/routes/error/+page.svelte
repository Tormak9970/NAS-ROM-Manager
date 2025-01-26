<script lang="ts">
  import type { PageData } from './$types';
  import { Icon } from "@component-utils";
  import { EmergencyHome } from "@icons";

  let { data }: { data: PageData } = $props();
</script>

<div id="error">
  <div class="container">
    <div class="icon">
      <Icon icon={EmergencyHome} />
    </div>
    <div class="text">
      <div class="message">{data.message}</div>
      <div class="fix">{data.fix}</div>
    </div>
  </div>
</div>

<style>
  #error {

  }
</style>