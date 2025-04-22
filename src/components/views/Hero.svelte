<script lang="ts">
  type Props = {
    src: string;
    portrait: boolean;
  }

  let { src, portrait }: Props = $props();
</script>

<div class="banner" class:landscape={!portrait}>
  <img src={src} alt="Banner placeholder">
  <div class="fade"></div>
</div>

<style>
  .banner {
    position: absolute;

    width: 100%;

    height: 30rem;
  }
  .banner.landscape {
    border-top-left-radius: var(--m3-util-rounding-medium);
    border-top-right-radius: var(--m3-util-rounding-medium);
    overflow: hidden;
  }

  .banner img {
    object-fit: cover;
    width: 100%;
    height: 100%;
  }

  .banner .fade {
    background: linear-gradient(rgb(var(--m3-scheme-background) / 0.47) 0%, rgb(var(--m3-scheme-background)) 100%);
    position: absolute;
    bottom: 0;
    
    width: 100%;
    height: 100%;
  }
</style>