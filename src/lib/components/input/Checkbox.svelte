<script lang="ts">
	import IconButton, { Variant } from '../button/IconButton.svelte';
	import Check from '../icons/Check.svelte';
	import ChevronDown from '../icons/ChevronDown.svelte';

	export let checked: boolean = false;
	export let collapseable: boolean = false;
	export let collapsed: boolean = false;
	export let onClick: () => void;
	export let onCollapse: () => void = () => {};

	export let label: string;

	$: css = checked ? 'bg-teal-400 !border-teal-400' : 'bg-transparent';
	$: iconCSS = checked ? 'text-gray-800' : 'text-transparent';
	$: collapseCSS = collapsed ? 'rotate-180 ' : '';
</script>

<div class="flex items-center">
	<button class="text-left flex items-center" on:click={onClick}>
		<div
			class="cursor-pointer border-2 -mt-[2px] border-gray-600 h-4 w-4 rounded ease-out duration-100 drop-shadow {css}"
		>
			<Check className="w-3 h-3 {iconCSS}" />
		</div>

		<div class="ml-3 text-gray-400 text-base">{label}</div>
	</button>

	{#if collapseable}
		<div class="ml-2 text-gray-500 h-full duration-200 ease-out {collapseCSS}">
			<IconButton
				onClick={onCollapse}
				icon={ChevronDown}
				variant={Variant.Embedded}
				label="Collapse children"
			/>
		</div>
	{/if}
</div>
