<script lang="ts">
  import { Icon } from "@component-utils";
  import { Download } from "@icons";
  import { Button } from "@interactables";
  import { Card } from "@layout";
  import { BiosFileService } from "@services";


  type Props = {
    system: string;
    biosFiles: string[];
  }

  let {system, biosFiles}: Props = $props();
</script>

<div class="bios-files">
  {#each biosFiles as file}
    <Card type="outlined" padding="0.5rem">
      <div class="file-container">
        <a
          href="http://{import.meta.env.NRM_SERVER_URL}/rest/bios-files/download?filePath={encodeURIComponent(BiosFileService.getFilePath(system, file))}"
          target="_blank"
          rel="noreferrer noopenner"
        >
          {file}
        </a>
        <div class="left">
          <Button iconType="full" type="text" onclick={() => BiosFileService.download(system, file)}>
            <Icon icon={Download} />
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