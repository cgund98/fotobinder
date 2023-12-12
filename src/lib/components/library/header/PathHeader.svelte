<script lang="ts">
	import Ellipsis from '../../icons/Ellipsis.svelte';
	import ChevronDown from '../../icons/ChevronDown.svelte';
	import type { HeaderEntry } from './PageHeader';
	import { routeToPage } from '$lib/nav/route';

	// Parse path
	const MAX_ELEMENTS = 3;
	export let path: HeaderEntry[];
	let active_path: HeaderEntry[];
	$: active_path = path.slice(Math.max(0, path.length - MAX_ELEMENTS), path.length);
</script>

<div class="w-full flex">
	<div class="flex justify-items-center">
		{#if path.length != active_path.length}
			<div class="mx-2 w-">
				<Ellipsis className="h-full w-8" />
			</div>
			<ChevronDown className="-rotate-90 text-gray-600 w-4 h-full" />
		{/if}
		{#each active_path as part, i}
			<button
				class="text-gray-100 text-xl font-bold mx-2 mt-[1px]"
				on:click={() => {
					routeToPage(part.route, part.queryParams);
					if (part.afterRoute) part.afterRoute();
				}}>{part.label}</button
			>
			{#if i != active_path.length - 1}
				<ChevronDown className="-rotate-90 text-gray-600 w-4 h-full" />
			{/if}
		{/each}
	</div>
</div>
