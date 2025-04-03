<script lang="ts">
  import { page } from "$app/state";
  import { routes } from "$lib/routes";
  import { Icon } from "@component-utils";
  import { AuthController } from "@controllers";
  import type { ContextMenuItem } from "@directives";
  import { Add, Person, PlaylistAdd } from "@icons";
  import { Button, MenuButton } from "@interactables";
  import { showAddRomModal, showAddSystemModal } from "@stores/Modals";
  import { isLandscape, systems } from "@stores/State";
  import SearchBar from "./SearchBar.svelte";

  const menuItems: ContextMenuItem[] = [
    {
      text: "Logout",
      action: AuthController.logout,
    }
  ];

  const systemKeys = $derived(Object.keys($systems));
</script>

<div id="header">
  <div class="branding">
    <img src="/logo.svg" alt="Logo" />
    <div class="m3-font-headline-small"><i>NRM</i></div>
    {#if $isLandscape}
      <SearchBar />
    {/if}
  </div>
  <div class="buttons">
    {#if page.url.pathname.startsWith(routes["Library"].path)}
      <Button type="text" iconType="full" onclick={() => $showAddRomModal = true} disabled={systemKeys.length === 0}>
        <Icon icon={Add}/>
      </Button>
    {:else if page.url.pathname.startsWith(routes["Systems"].path)}
      <Button type="text" iconType="full" onclick={() => $showAddSystemModal = true}>
        <Icon icon={PlaylistAdd}/>
      </Button>
    {/if}
    <MenuButton items={menuItems} icon={Person} />
  </div>
</div>

<style>
  #header {
    width: 100%;
    height: 60px;

    display: flex;
    align-items: center;
    justify-content: space-between;

    border-bottom: 1px solid rgb(var(--m3-scheme-surface-container-highest));
  }

  .branding {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-left: 1.25rem;
    flex: 1 1 0%;
  }

  .branding > div {
    font-weight: bold;
    color: rgb(var(--m3-scheme-primary));
    flex-shrink: 0;
  }

  .branding > img {
    width: 2.5rem;
    height: 2.5rem;
  }

  .buttons {
    display: flex;
    align-items: center;
    gap: 0.25rem;

    margin: 0rem 1rem;
  }
</style>