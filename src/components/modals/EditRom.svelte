<script lang="ts">
  import { ModalBody } from "@component-utils";
  import { Button, FileField, Select } from "@interactables";
  import { editIsPostUpload, romEditingId, showEditRomModal } from "@stores/Modals";
  import { libraries, systems } from "@stores/State";

  let open = $state(true);
  
  let libraryOptions: SelectItem[] = Object.keys($libraries).map((key) => {
    return { label: key, value: key };
  });

  let systemOptions: SelectItem[] = Object.keys($systems).map((key) => {
    return { label: key, value: key };
  });

  let library = $state(libraryOptions[0].value);
  let system = $state(systemOptions[0].value);
  let file = $state<File | null>(null);
  
  let isZip = $derived(file && file.name.endsWith(".zip"));
  let needsUnzip = $state(false);
  let okStructure = $state(false);

  let canSave = $derived(!!library && !!system && !!file && (!isZip || okStructure));

  /**
   * Function to run on confirmation.
   */
  async function onSave(): Promise<void> {
    // $roms[$romEditingId!] = {

    // }
  }

  /**
   * Function to run on cancel.
   */
  async function onCancel(): Promise<void> {
    open = false;
  }

  function closeEnd() {
    $showEditRomModal = false;
    $editIsPostUpload = false;
    $romEditingId = null;
  }
</script>

<ModalBody headline={$editIsPostUpload ? "Confirm Details" : "Edit ROM"} open={open} canClose={false} on:closeEnd={closeEnd}>
  <div class="content">
    <Select name="Library" options={libraryOptions} disabled={libraryOptions.length === 1} bind:value={library} />
    <Select name="System" options={systemOptions} disabled={systemOptions.length === 1} bind:value={system} />
    <FileField name="File" on:change={(e) => file = e.detail.value} />
    
  </div>
  <div slot="buttons" class="buttons">
    <Button type="text" on:click={onCancel}>Cancel</Button>
    <Button type="text" on:click={onSave} disabled={!canSave}>Save</Button>
  </div>
</ModalBody>

<style>
  .content {
    width: 300px;

    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 20px;
  }
</style>
