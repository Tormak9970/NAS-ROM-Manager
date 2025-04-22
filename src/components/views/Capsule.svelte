<script lang="ts">
  let { thumbCapsulePath }: { thumbCapsulePath: string } = $props();
</script>

<div class="capsule" style:--cover-url='url("{thumbCapsulePath === "No Grids" ? "" : thumbCapsulePath}")'></div>

<style>
  .capsule {
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