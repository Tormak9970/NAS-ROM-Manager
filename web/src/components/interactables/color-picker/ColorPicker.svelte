<script lang="ts">
	import ColorPicker, { type RgbaColor } from 'svelte-awesome-color-picker';
	import Input from './Input.svelte';
	import TextInput from './TextInput.svelte';
	import Wrapper from './Wrapper.svelte';

  let {
    hex = $bindable(),
    rgb = $bindable(),
  }: { hex?: string; rgb?: RgbaColor } = $props();
</script>

<div class="wrapper">
  <ColorPicker
    components={{
      wrapper: Wrapper,
      input: Input,
      textInput: TextInput
    }}
    label=""
    bind:hex={hex}
    bind:rgb={rgb}
    isAlpha={false}
  />
</div>

<style>
  .wrapper {
		--cp-text-color: rgb(var(--m3-scheme-on-background));
		--cp-input-color: rgb(var(--m3-scheme-background));
		--cp-button-hover-color: rgb(var(--m3-scheme-surface-container-lowest));
    --slider-width: 15px;
    --input-size: 36px;
    --focus-color: rgb(var(--m3-scheme-primary));

    ---thumb-border: 1px solid transparent;
  }
</style>