<script lang="ts">
  let { children, backgroundColor, borderColor = null, isUppercase = false } = $props();
</script>

<div
  class="tag"
  class:uppercase={isUppercase}
  style:--background-color={backgroundColor}
  style:--border-color={borderColor ?? backgroundColor}
>
  {@render children()}
</div>

<style>
  .tag {
    background-color: rgb(var(--background-color) / 0.8);
    color: white;
    
    border: 1px solid rgb(var(--border-color));
    border-radius: var(--m3-util-rounding-medium);

    font-size: .8rem;

    padding: 0.1rem 0.5rem;

    --shadow: 0 4px 6px -1px rgba(0,0,0,.1), 0 2px 4px -2px rgba(0,0,0,.1);
    box-shadow: 0 0 #0000, 0 0 #0000, var(--shadow);

    pointer-events: all;
  }

  .uppercase {
    font-size: .75rem;
    line-height: 1rem;
  }
</style>