<script lang="ts">
  import { ColorPicker } from "@interactables";
  import Tag from "@views/Tag.svelte";
  import type { RgbaColor } from "svelte-awesome-color-picker";

  type Props = {
    tagColor?: RgbaColor;
  }

  let {
    tagColor = $bindable<RgbaColor>({
      r: 228,
      g: 0,
      b: 15,
      a: 1
    })
  }: Props = $props();

  const tagConfigColor = $derived(`${tagColor.r} ${tagColor.g} ${tagColor.b}`);
</script>

<label>
  <div class="color-picker-label">
    <div class="m3-font-title-medium">Tag Color</div>
    <Tag backgroundColor={tagConfigColor} borderColor={tagConfigColor}>Preview</Tag>
  </div>
  <ColorPicker bind:rgb={tagColor} />
</label>

<style>
  .color-picker-label {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
</style>