<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from '$app/state';
  import { routes, type Route } from "$lib/routes";
  import { portraitViews } from "@stores/State";
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";

  const portraitRoutes = $derived($portraitViews.reduce((filtered: Route[], view: string) => {
    const route = routes[view];

    if (route) filtered.push(route);

    return filtered;
  }, []));
</script>

<div class="view-nav">
  <NavList type="bar" extraOptions={{ style: "padding: 0.75rem 0.5rem; height: 56px;" }}>
    {#each portraitRoutes as view}
      {@const selected = page.url.pathname.startsWith(view.path)}
      <NavListButton
        type="bar"
        icon={selected ? view.selectedIcon : view.icon}
        selected={selected}
        onclick={() => goto(view.path)}
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

    border-radius: var(--m3-util-rounding-small) var(--m3-util-rounding-small) 0 0;
  }
</style>