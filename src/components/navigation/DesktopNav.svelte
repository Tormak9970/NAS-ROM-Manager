<script lang="ts">
  import NavList from "./NavList.svelte";
  import NavListButton from "./NavListButton.svelte";
  import { views } from "$lib/routes";

  import { page } from '$app/state';  

  let { condenseNav = false } = $props();
</script>

<NavList type="rail">
  {#each views as view}
    <NavListButton
      type="rail"
      icon={view.icon}
      selected={view.path === page.url.pathname}
      on:click={() => {}}
    >
      {#if !condenseNav}
        {view.label}
      {/if}
    </NavListButton>
  {/each}
</NavList>