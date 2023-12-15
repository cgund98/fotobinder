<script lang="ts">
	import { page } from '$app/stores';
	import type { ComponentType } from 'svelte';
	import ChevronDown from '../../icons/ChevronDown.svelte';
	import IconButton, { Variant } from '$lib/components/button/IconButton.svelte';

	export let title: string;
	export let href: string;
	export let collapseable: boolean = false;
	export let icon: ComponentType;
	export let showChildren = false;

	$: active = $page.url.pathname.startsWith(href);

	$: iconProps = {
		className: `h-[20px] w-[20px] duration-200 ease-out ${
			active ? 'text-teal-500' : 'text-gray-500'
		}`
	};
</script>

<div class="flex flex-row justify-between pr-3 pl-5 py-4">
	<a {href} class="flex">
		<div>
			<svelte:component this={icon} {...iconProps} />
		</div>
		<p class="ml-3 text-base font-medium text-gray-300">{title}</p>
	</a>
	<div>
		{#if collapseable}
			<IconButton
				label={showChildren ? 'Collapse children' : 'Expand children'}
				icon={ChevronDown}
				variant={Variant.Embedded}
				className="text-gray-600 duration-200 ease-out {showChildren ? 'rotate-180' : ''}"
				onClick={() => (showChildren = !showChildren)}
			/>
		{/if}
	</div>
</div>

<slot />
