<script lang="ts">
  import { ProgressIndicator } from "@interactables";
  import { Card } from "@layout";
  import { WebsocketService } from "@services";
  import type { AvailableStorage } from "@types";
  import { formatFileSize } from "@utils";
  import { onMount } from "svelte";

  let { extraOptions } = $props();

  let storageInfo: AvailableStorage = $state({
    usedSpace: 0,
    totalSpace: 1,
  });

  onMount(() => {
    WebsocketService.getStorageInfo().then((info) => {
      storageInfo = info;
    });
  });
</script>

<Card type="filled" extraOptions={extraOptions}>
  <div class="m3-font-title-medium" style="font-weight: bold;">Storage</div>
  <div class="storage-tracker">
    <div style="margin-bottom: 0.5rem; margin-top: 0.75rem;">
      {formatFileSize(storageInfo.usedSpace)} of {formatFileSize(storageInfo.totalSpace)} used
    </div>
    <ProgressIndicator percent={storageInfo.usedSpace / storageInfo.totalSpace * 100} />
  </div>
</Card>