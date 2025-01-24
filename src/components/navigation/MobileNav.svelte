<script lang="ts">
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";
  import { mobileViews } from "$lib/routes";
    
  import { page } from '$app/state';  
  import { goto } from "$app/navigation";
</script>

<div class="view-nav">
  <NavList type="bar" extraOptions={{ style: "padding: 0.75rem 0.5rem; height: 56px;" }}>
    {#each mobileViews as view}
      {@const selected = view.path === page.url.pathname}
      <NavListButton
        type="bar"
        icon={selected ? view.selectedIcon : view.icon}
        selected={selected}
        on:click={() => goto(view.path)}
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