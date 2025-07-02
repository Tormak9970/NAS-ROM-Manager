<script lang="ts">
  import { Icon } from "@component-utils";
  import { Close, Download, Upload } from "@icons";
  import { Button } from "@interactables";
  import { Card } from "@layout";
  import { BiosFileService } from "@services";
  import type { System } from "@types";

  type Props = {
    system: System | undefined;
  }

  let {system}: Props = $props();

  const biosFiles = $derived(system?.biosFiles ?? []);
  const folder = $derived(system?.folder);
  const key = $derived(system?.abbreviation);
</script>

<div class="bios-files">
  {#each biosFiles as file}
    <Card type="outlined" padding="0.5rem">
      <div class="file-container">
        <a
          href="http://{import.meta.env.NRM_SERVER_URL}/rest/bios-files/download?filePath={encodeURIComponent(BiosFileService.getFilePath(folder!, file))}"
          target="_blank"
          rel="noreferrer noopenner"
        >
          {file}
        </a>
        <div class="left">
          <Button iconType="full" type="text" onclick={() => BiosFileService.replaceFile(key!, file)}>
            <Icon icon={Upload} />
          </Button>
          <Button iconType="full" type="text" onclick={() => BiosFileService.download(folder!, file)}>
            <Icon icon={Download} />
          </Button>
          <Button iconType="full" type="text" warning onclick={() => BiosFileService.delete(key!, file)}>
            <Icon icon={Close} />
          </Button>
        </div>
      </div>
    </Card>
  {/each}
</div>

<style>
  .bios-files {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    margin-bottom: 2rem;
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
</style>