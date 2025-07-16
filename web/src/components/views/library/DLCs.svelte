<script lang="ts">
  import { Icon } from "@component-utils";
  import { Add, Close, Download, Upload } from "@icons";
  import { Button } from "@interactables";
  import { Card } from "@layout";
  import { ExtraFileService } from "@services";
  import { addExtraFileRomId, addExtraFileSystem, addExtraFileType, showAddExtraFileModal } from "@stores/Modals";
  import { romDLCs } from "@stores/State";
  import { ExtraFileType, type System } from "@types";

  type Props = {
    romId: string;
    system: System;
  }

  let { romId, system }: Props = $props();

  const type = ExtraFileType.DLC;
  const folder = $derived(system?.folder);
</script>

<div>
  <div class="header">
    <h2>DLC Files</h2>
    <Button iconType="full" type="text" onclick={() => {
      $addExtraFileType = type;
      $addExtraFileRomId = romId;
      $addExtraFileSystem = system.abbreviation;
      $showAddExtraFileModal = true;
    }}>
      <Icon icon={Add} />
    </Button>
  </div>
  <div class="dlc-files">
    {#each ($romDLCs[romId] ?? []) as file}
      <Card type="outlined" padding="0.5rem">
        <div class="file-container">
          <a
            href="http://{import.meta.env.NRM_SERVER_URL}/rest/rom-extras/download?filePath={encodeURIComponent(ExtraFileService.getFilePath(type, folder!, romId, file))}"
            target="_blank"
            rel="noreferrer noopenner"
          >
            {file}
          </a>
          <div class="left">
            <Button iconType="full" type="text" onclick={() => ExtraFileService.replaceFile(type, folder!, romId, file)}>
              <Icon icon={Upload} />
            </Button>
            <Button iconType="full" type="text" onclick={() => ExtraFileService.download(type, folder!, romId, file)}>
              <Icon icon={Download} />
            </Button>
            <Button iconType="full" type="text" warning onclick={() => ExtraFileService.delete(type, folder!, romId, file)}>
              <Icon icon={Close} />
            </Button>
          </div>
        </div>
      </Card>
    {:else}
      <div class="message-container">No DLCs found for this game.</div>
    {/each}
  </div>
</div>

<style>
  .header {
    width: 100%;

    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .dlc-files {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .file-container {
    width: 100%;

    display: flex;

    align-items: center;
    justify-content: space-between;

    margin-left: 0.5rem;
  }

  .left {
    display: flex;
    align-items: center;

    margin-right: 0.5rem;
  }
  
  .message-container {
    width: 100%;
  }
</style>