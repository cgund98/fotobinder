<script lang="ts">
	import Check from '../icons/Check.svelte';
	import ChevronDown from '../icons/ChevronDown.svelte';

	export let checked: boolean = false;
	export let collapseable: boolean = false;

	export let label: string;

	let css = 'bg-transparent';
	if (checked) css = 'bg-teal-400 !border-teal-400';
</script>

<div class="flex items-center">
	<button class="text-left flex items-center" on:click={() => (checked = !checked)}>
		<div
			class="cursor-pointer border-2 -mt-[2px] border-gray-600 h-4 w-4 rounded ease-out duration-100 drop-shadow {css}"
		>
			<Check className="w-3 h-3 {checked ? 'text-gray-800' : 'text-transparent'}" />
		</div>

		<div class="ml-3 text-gray-400 text-base">{label}</div>
	</button>

	{#if collapseable}
		<ChevronDown className="ml-3 text-gray-500 h-full" />
	{/if}
</div>
