<script lang="ts">
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";
  import { views } from "$lib/routes";
    
  import { page } from '$app/state';  
</script>

<div class="view-nav">
  <NavList type="bar" extraOptions={{ style: "padding: 0.75rem 0.5rem; height: 56px;" }}>
    {#each views as view}
      <NavListButton
        type="bar"
        icon={view.icon}
        selected={view.path === page.url.pathname}
        on:click={() => {}}
      />
    {/each}
  </NavList>
</div>

<style>
  .view-nav {
    width: 100%;
    height: 56px;

    overflow: hidden;

    position: relative;
    z-index: 4;
  }
</style>