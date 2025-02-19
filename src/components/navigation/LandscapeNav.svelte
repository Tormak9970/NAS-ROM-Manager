<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from '$app/state';
  import { landscapeRoutes, type Route } from "$lib/routes";
  import { landscapeViews } from "@stores/State";
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";


  let { condenseNav = false } = $props();

  let routes = $derived($landscapeViews.reduce((filtered: Route[], view: string) => {
    const route = landscapeRoutes[view];

    if (route) filtered.push(route);

    return filtered;
  }, []));
</script>

<NavList type="rail">
  {#each routes as view}
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