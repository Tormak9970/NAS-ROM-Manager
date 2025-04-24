<script lang="ts">
  import { contextMenu, type ContextMenuItem } from "@directives";

  type Props = {
    src: string;
    portrait: boolean;
    onEdit: () => void;
  }

  let { src, portrait, onEdit }: Props = $props();

  const menuItems = $derived<ContextMenuItem[]>([
    {
      text: "Edit Banner",
      action: onEdit
    }
  ])
</script>

<div class="hero" class:landscape={!portrait}>
  {#key src}
    <img src={src} alt="Banner placeholder">
  {/key}
  <div class="fade" use:contextMenu={{ items: menuItems }}></div>
</div>

<style>
  .hero {
    position: absolute;

    width: 100%;

    height: 30rem;
    
    pointer-events: all;
  }
  .hero.landscape {
    border-top-left-radius: var(--m3-util-rounding-medium);
    border-top-right-radius: var(--m3-util-rounding-medium);
    overflow: hidden;
  }

  .hero img {
    object-fit: cover;
    width: 100%;
    height: 100%;
  }

  .hero .fade {
    background: linear-gradient(rgb(var(--m3-scheme-background) / 0.47) 0%, rgb(var(--m3-scheme-background)) 100%);
    position: absolute;
    bottom: 0;
    
    width: 100%;
    height: 100%;
  }
</style>