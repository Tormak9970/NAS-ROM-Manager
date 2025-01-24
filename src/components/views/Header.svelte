<script lang="ts">
  import { MenuButton } from "@interactables";
  import type { ContextMenuItem } from "@directives";
  import { Person } from "@icons";
  import { AuthController } from "@controllers";
  import { isLandscape } from "@stores/State";
  import SearchBar from "./SearchBar.svelte";

  const menuItems: ContextMenuItem[] = [
    {
      id: "logout",
      text: "Logout",
      action: AuthController.logout,
    }
  ]
</script>

<div id="desktop-header">
  <div class="branding">
    <img src="/logo.svg" alt="Logo" />
    <div class="m3-font-headline-small">ROM Manager</div>
    {#if $isLandscape}
      <SearchBar />
    {/if}
  </div>
  <div class="account">
    <MenuButton items={menuItems} icon={Person} />
  </div>
</div>

<style>
  #desktop-header {
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
    flex-grow: 1;
    flex-shrink: 0;
  }

  .branding > img {
    width: 2.5rem;
    height: 2.5rem;
  }

  .account {
    margin: 0rem 1rem;
  }
</style>