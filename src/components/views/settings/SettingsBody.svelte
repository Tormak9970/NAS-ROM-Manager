<script lang="ts">
  import { Icon } from "@component-utils";
  import { isScrolled, scrollShadow } from "@directives";
  import { BackArrow } from "@icons";
  import { Button } from "@interactables";

  type Props = {
    title: string;
    children?: any;
  }

  let { title, children }: Props = $props();

  let highlight = $state(false);
</script>

<div class="settings-body"> <!-- transition:sharedAxisTransition={{ direction: "Z", leaving: false }}-->
  <div class="header" class:highlight>
    <Button type="text" iconType="full" on:click={() => window.history.back()}>
      <Icon icon={BackArrow} width="20px" height="20px" />
    </Button>
    <div class="font-headline" style="margin-left: 10px; font-weight: bold;">{title}</div>
  </div>
  <div class="scroll-wrapper">
    <div class="content" use:isScrolled={{ callback: (isScrolled) => { highlight = isScrolled }}} use:scrollShadow={{ background: "--m3-scheme-background" }}>
      <div class="inner-content">
        {@render children?.()}
      </div>
    </div>
  </div>
</div>

<style>
  .settings-body {
    width: 100%;
    height: 100%;
  }

  .header {
    width: calc(100% - 0.5rem);

    padding-left: 0.5rem;
    
    height: 50px;

    display: flex;
    align-items: center;
    
    background-color: rgb(var(--m3-scheme-background));

    transition: background-color 0.2s ease-in-out;
  }

  .highlight {
    background-color: rgb(var(--m3-scheme-surface-container));
  }

  .scroll-wrapper {
    width: 100%;
    height: calc(100% - 50px);

    position: relative;
  }

  .content {
    width: 100%;
    height: 100%;

    overflow-y: scroll;
    overflow-x: hidden;
  }

  .inner-content {
    width: 100%;
    height: fit-content;

    display: flex;
    flex-direction: column;
    align-items: center;

    gap: 0.5rem;

    padding-top: 0.5rem;

    padding-bottom: 0.5rem;
  }
</style>