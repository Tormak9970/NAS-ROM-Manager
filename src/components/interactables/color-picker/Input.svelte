<script lang="ts">
  import { preventDefault } from "@utils";

	type Props = {
		/** DOM element of the label wrapper */
		labelElement: HTMLLabelElement;
		/** hex color */
		hex: string | undefined;
		/** input label */
		label: string;
		/** input name, useful in a native form */
		name?: string | undefined;
		/* svelte-ignore unused-export-let /** indicator of the popup state */
		isOpen: boolean;
	}

	let {
		labelElement = $bindable(),
		hex,
		label,
		name = undefined,
		isOpen
	}: Props = $props();

	function noop() {
		/* prevent browser color picker from opening unless javascript is broken */
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions, a11y_click_events_have_key_events -->
<label bind:this={labelElement} onclick={preventDefault(noop)} onmousedown={preventDefault(noop)}>
	<div class="container">
		<input
			type="color"
      autocomplete="off"
			{name}
			value={hex}
			onclick={preventDefault(noop)}
			onmousedown={preventDefault(noop)}
			aria-haspopup="dialog"
		/>
		<div class="alpha"></div>
		<div class="color" style:background={hex}></div>
	</div>
	{label}
</label>

<style>
	label {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		border-radius: 3px;
		margin: 4px;
		height: var(--input-size, 25px);
		user-select: none;
	}

	.container {
		position: relative;
		display: block;
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--input-size, 25px);
	}

	input {
		margin: 0;
		padding: 0;
		border: none;
		width: 1px;
		height: 1px;
		flex-shrink: 0;
		opacity: 0;
	}

	.alpha {
		clip-path: circle(50%);
		background: var(--alpha-grid-bg);
	}

	.alpha,
	.color {
		position: absolute;
		width: var(--input-size, 25px);
		height: var(--input-size, 25px);
		border-radius: 50%;
    border: 2px solid rgb(var(--m3-scheme-outline));
		user-select: none;
	}

	input:focus-visible ~ .color {
		outline: 2px solid var(--focus-color, red);
		outline-offset: 2px;
	}
</style>