<script lang="ts">
	import Check from '../icons/Check.svelte';
	import ChevronDown from '../icons/ChevronDown.svelte';

	export let checked: boolean = false;
	export let collapseable: boolean = false;

	export let label: string;

	let css = '';
	if (checked) css = 'bg-teal-400 !border-teal-400';
</script>

<div class="flex items-center">
	<div role="button" class="flex items-center">
		<div
			class="cursor-pointer border-2 -mt-[2px] bg-gray-800 border-gray-600 h-4 w-4 rounded ease-out duration-100 drop-shadow {css}"
		>
			<Check className="w-3 h-3 text-gray-800" />
		</div>

		<div class="ml-3 text-gray-400 text-base">{label}</div>
	</div>

	{#if collapseable}
		<ChevronDown className="ml-3 text-gray-500 h-full" />
	{/if}
</div>
