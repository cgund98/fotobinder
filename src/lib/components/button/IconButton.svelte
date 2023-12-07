<script lang="ts" context="module">
	export enum Variant {
		Primary = 'primary',
		Embedded = 'embedded'
	}
</script>

<script lang="ts">
	import type { ComponentType } from 'svelte';
	import type { MouseEventHandler } from 'svelte/elements';
	import Tooltip from '../decoration/Tooltip.svelte';

	export let icon: ComponentType;
	export let variant: Variant = Variant.Primary;
	export let className: string = '';
	export let shadow = true;
	export let label: string;
	export let disabled = false;

	export let onClick: MouseEventHandler<HTMLButtonElement> = () => {};

	$: props = {
		className: `w-[18px] mx-auto ${className}`
	};

	const shadowCSS = shadow ? 'drop-shadow' : '';

	const commonCSS = 'min-h-[28px] min-w-[28px] flex flex-col justify-around rounded-lg';
</script>

<Tooltip title={label}>
	{#if variant == Variant.Primary}
		<button on:click={onClick} class="bg-gray-700 text-gray-400 {shadowCSS} {commonCSS}" {disabled}>
			<svelte:component this={icon} {...props} />
		</button>
	{/if}

	{#if variant == Variant.Embedded}
		<button on:click={onClick} class="text-gray-600 hover:text-gray-100 {commonCSS}" {disabled}>
			<svelte:component this={icon} {...props} />
		</button>
	{/if}
</Tooltip>
