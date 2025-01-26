<script lang="ts">
  let { children, color, isUppercase = false } = $props();
</script>

<div class="rom-tag" class:uppercase={isUppercase} style:--background-color={color}>
  {@render children()}
</div>

<style>
  .rom-tag {
    background-color: rgb(var(--background-color) / 0.8);
    color: white;
    
    border: 1px solid rgb(var(--background-color));
    border-radius: var(--m3-util-rounding-medium);

    font-size: .8rem;

    padding: 0.1rem 0.5rem;

    --shadow: 0 4px 6px -1px rgba(0,0,0,.1), 0 2px 4px -2px rgba(0,0,0,.1);
    box-shadow: 0 0 #0000, 0 0 #0000, var(--shadow);
  }

  .uppercase {
    font-size: .75rem;
    line-height: 1rem;
  }
</style>