<script lang="ts">
  let { thumbPath }: { thumbPath: string } = $props();
</script>

<div class="cover" style:--cover-url='url("{thumbPath === "No Grids" ? "" : thumbPath}")'></div>

<style>
  .cover {
    position: absolute;
    top: 0;
    left: 0;

    width: 100%;
    height: 100%;
    border-radius: var(--m3-util-rounding-medium);

    background-image: var(--cover-url);
    background-repeat: no-repeat;
    background-size: contain;
  }
</style>