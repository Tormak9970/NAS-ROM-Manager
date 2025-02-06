<script lang="ts">
  import { Icon } from "@component-utils";
  import { AuthController } from "@controllers";
  import type { ContextMenuItem } from "@directives";
  import { Add, Person } from "@icons";
  import { Button, MenuButton } from "@interactables";
  import { showAddRomModal } from "@stores/Modals";
  import { isLandscape, libraries, systems } from "@stores/State";
  import SearchBar from "./SearchBar.svelte";

  const menuItems: ContextMenuItem[] = [
    {
      id: "logout",
      text: "Logout",
      action: AuthController.logout,
    }
  ];

  let libraryKeys = $derived(Object.keys($libraries));
  let systemKeys = $derived(Object.keys($systems));
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
    <Button type="text" iconType="full" on:click={() => $showAddRomModal = true} disabled={libraryKeys.length === 0 || systemKeys.length === 0}>
      <Icon icon={Add}/>
    </Button>
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