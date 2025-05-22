<script lang="ts">
  import { onMount } from "svelte";

  type Props = {
    query: string;
    /**
     * DO NOT MODIFY OUTSIDE OF COMPONENT.
     * Initial value must be false
     */
    matches?: boolean;
  }

  let {
    query,
    matches = $bindable(false),
  }: Props = $props();

  let mql: MediaQueryList | undefined = $state();
  let queryListener: ((event: any) => void) | undefined = $state();
  let wasMounted = $state(false);

  function addNewListener(query:string) {
    mql = window.matchMedia(query);

    queryListener = (event: MediaQueryList) => {
      matches = event.matches;
    };

    mql.addEventListener("change", queryListener);
    matches = mql.matches;
  }

  function removeActiveListener() {
    if (mql && queryListener) mql.removeEventListener('change', queryListener);
  }
  
  onMount(() => {
    wasMounted = true;

    removeActiveListener();
    addNewListener(query);

    return () => {
      removeActiveListener();
    }
  });
</script>