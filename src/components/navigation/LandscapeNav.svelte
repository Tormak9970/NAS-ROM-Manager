<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from '$app/state';
  import { routes, type Route } from "$lib/routes";
  import { landscapeViews } from "@stores/State";
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";

  let { condenseNav = false } = $props();

  const landscapeRoutes = $derived($landscapeViews.reduce((filtered: Route[], view: string) => {
    const route = routes[view];

    if (route) filtered.push(route);

    return filtered;
  }, []));
</script>

<NavList type="rail">
  {#each landscapeRoutes as view}
    {@const selected = page.url.pathname.startsWith(view.path)}
    <NavListButton
      type="rail"
      icon={selected ? view.selectedIcon : view.icon}
      selected={selected}
      onclick={() => goto(view.path)}
    >
      {#if !condenseNav}
        {view.label}
      {/if}
    </NavListButton>
  {/each}
</NavList>