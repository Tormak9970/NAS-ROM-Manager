<script lang="ts">
  import { Icon } from "@component-utils";
  import { Search, Tune } from "@icons";

  let searchValue = $state("");

  function onInput(e: Event) {

  }

  function onFilterClick() {

  }
</script>

<div class="m3-container" >
  <Icon icon={Search} class="leading" />
  <input
    class="m3-font-body-large"
    autocomplete="off"
    placeholder="Search your ROM library"
    bind:value={searchValue}
    oninput={onInput}
  />
  <div class="layer"></div>
  <button onclick={onFilterClick} class="trailing">
    <Icon icon={Tune} />
  </button>
</div>

<style>
  :root {
    --m3-textfield-outlined-shape: var(--m3-util-rounding-medium);
  }
  .m3-container {
    margin-left: 5rem;
    display: inline-flex;
    align-items: center;
    
    position: relative;

    height: 3rem;
    width: 100%;
    max-width: 50rem;
  }

  input {
    position: absolute;
    z-index: 2;
    inset: 0;
    left: 2rem;
    width: calc(100% - 4.5rem);
    height: 100%;
    border: none;
    outline: none;
    padding: 1rem;
    border-radius: var(--m3-textfield-outlined-shape);
    background-color: transparent;
    color: rgb(var(--m3-scheme-on-surface));
  }

  .layer {
    position: absolute;
    z-index: 1;

    inset: 0;
    border: 0.125rem solid rgb(var(--error, var(--m3-scheme-outline)));
    background-color: rgb(var(--m3-scheme-surface-container));
    border-radius: var(--m3-textfield-outlined-shape);
    pointer-events: none;
    transition: all 200ms;
  }

  input:hover ~ .layer {
    border-color: rgb(var(--error, var(--m3-scheme-on-surface)));
  }
  input:focus ~ .layer {
    border-color: rgb(var(--error, var(--m3-scheme-primary)));
    border-width: 0.125rem;
  }

  .m3-container :global(svg) {
    width: 1.4rem;
    height: 1.4rem;
    color: rgb(var(--m3-scheme-on-surface-variant));
    pointer-events: none;
  }
  .m3-container > :global(.leading) {
    position: relative;
    z-index: 2;
    margin-left: 0.75rem;
  }
  
  .trailing {
    position: absolute;
    z-index: 2;

    padding-left: 0.425rem;
    padding-right: 0.425rem;
    height: 2.25rem;
    right: 0.4rem;

    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background-color: transparent;
    border-radius: 1.125rem;

    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    transition: all 200ms;
  }
  button:hover {
    background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.08);
  }
  button:focus-visible,
  button:active {
    background-color: rgb(var(--m3-scheme-on-surface-variant) / 0.12);
  }
</style>