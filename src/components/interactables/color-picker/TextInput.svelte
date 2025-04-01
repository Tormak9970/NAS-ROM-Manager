<script lang="ts">
	import Button from "../Button.svelte";

	type Props = {
		/** if set to false, disables the alpha channel */
		isAlpha: boolean;
		/** rgb color */
		rgb: any;
		/** hsv color */
		hsv: any;
		/** hex color */
		hex: string;
		/** configure which hex, rgb and hsv inputs will be visible and in which order. If overridden, it is necessary to provide at least one value */
		textInputModes: Array<'hex' | 'rgb' | 'hsv'>;
		/** all translation tokens used in the library; can be partially overridden; see [full object type](https://github.com/Ennoriel/svelte-awesome-color-picker/blob/master/src/lib/utils/texts.ts) */
		texts: any;

    oninput: (value: any) => void
	}

	let {
		isAlpha,
		rgb = $bindable(),
		hsv = $bindable(),
		hex = $bindable(),
		textInputModes,
		texts,
    oninput
	}: Props = $props();

	const HEX_COLOR_REGEX = /^#?([A-F0-9]{6}|[A-F0-9]{8})$/i;

	let mode: 'hex' | 'rgb' | 'hsv' = $state(textInputModes[0] || 'hex');

	let nextMode = $derived(textInputModes[(textInputModes.indexOf(mode) + 1) % textInputModes.length]);

	let h = $derived(Math.round(hsv.h));
	let s = $derived(Math.round(hsv.s));
	let v = $derived(Math.round(hsv.v));
	let a = $derived(hsv.a === undefined ? 1 : Math.round(hsv.a * 100) / 100);

	type InputEvent = Event & { currentTarget: EventTarget & HTMLInputElement };

	function updateHex(e: InputEvent) {
		const target = e.target as HTMLInputElement;
		if (HEX_COLOR_REGEX.test(target.value)) {
			hex = target.value;
			oninput(hex);
		}
	}

	function updateRgb(property: string) {
		return function (e: InputEvent) {
			rgb = { ...rgb, [property]: parseFloat((e.target as HTMLInputElement).value) };
			oninput(rgb);
		};
	}

	function updateHsv(property: string) {
		return function (e: InputEvent) {
			hsv = { ...hsv, [property]: parseFloat((e.target as HTMLInputElement).value) };
			oninput(hsv);
		};
	}
</script>

<div class="text-input" class:alpha={isAlpha}>
	<div class="input-container">
		{#if mode === 'hex'}
			<input autocomplete="off" aria-label={texts.label.hex} value={hex} oninput={updateHex} style:flex={3} />
		{:else if mode === 'rgb'}
			<input autocomplete="off" aria-label={texts.label.r} value={rgb.r} type="number" min="0" max="255" oninput={updateRgb('r')} />
			<input autocomplete="off" aria-label={texts.label.g} value={rgb.g} type="number" min="0" max="255" oninput={updateRgb('g')} />
			<input autocomplete="off" aria-label={texts.label.b} value={rgb.b} type="number" min="0" max="255" oninput={updateRgb('b')} />
		{:else}
			<input autocomplete="off" aria-label={texts.label.h} value={h} type="number" min="0" max="360" oninput={updateHsv('h')} />
			<input autocomplete="off" aria-label={texts.label.s} value={s} type="number" min="0" max="100" oninput={updateHsv('s')} />
			<input autocomplete="off" aria-label={texts.label.v} value={v} type="number" min="0" max="100" oninput={updateHsv('v')} />
		{/if}
	</div>

	<Button
    type="tonal"
    onclick={() => (mode = nextMode)}
    extraOptions={{ style: "width: 100%; margin-top: 8px;" }}
  >
    {texts.color[mode]}
  </Button>
</div>

<style>
	.text-input {
		margin: var(--text-input-margin, 5px 0 0);
	}
	.input-container {
		display: flex;
		flex: 1;
		gap: 10px;
	}
	input {
		flex: 1;
		border: none;
		background-color: var(--cp-input-color, #eee);
		color: var(--cp-text-color, var(--cp-border-color));
		padding: 0;
		border-radius: 5px;
		height: 30px;
		line-height: 30px;
		text-align: center;
    
		width: 5px;
		font-family: inherit;
		outline: 2px solid transparent;
    transition: outline-color 0.2s ease-in-out; 
	}

  input:focus {
		outline: 2px solid transparent;
	}

	input:focus-visible {
		outline: 2px solid var(--focus-color, red);
	}
</style>