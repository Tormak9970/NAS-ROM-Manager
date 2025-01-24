<script lang="ts">
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";
  import { desktopViews } from "$lib/routes";

  import { page } from '$app/state';  
  import { goto } from "$app/navigation";

  let { condenseNav = false } = $props();
</script>

<NavList type="rail">
  {#each desktopViews as view}
    {@const selected = view.path === page.url.pathname}
    <NavListButton
      type="rail"
      icon={selected ? view.selectedIcon : view.icon}
      selected={selected}
      on:click={() => goto(view.path)}
    >
      {#if !condenseNav}
        {view.label}
      {/if}
    </NavListButton>
  {/each}
</NavList>