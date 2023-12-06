<script lang="ts">
	import type { MouseEventHandler } from 'svelte/elements';
	import FolderSolid from '../../icons/FolderSolid.svelte';
	import Plus from '../../icons/Plus.svelte';

	export let name: string;
	export let add: boolean = false;
	export let active: boolean = false;
	export let onClick: MouseEventHandler<HTMLButtonElement> = () => {};
	export let onDoubleClick: (() => void) | undefined = undefined;

	let addCSS = '';
	if (add) addCSS = 'text-teal-400 bg-gray-700';
	let activeCSS: string;
	$: if (active) {
		activeCSS = 'bg-teal-400 text-gray-900';
	} else {
		activeCSS = '';
	}

	let lastClicked = 0;

	const DOUBLE_CLICK_THRESHOLD = 200;
</script>

<button
	class="w-full text-left cursor-pointer rounded-md flex h-full justify-items-center bg-gray-700 text-gray-300 fill-gray-400 py-3 px-4 drop-shadow ease-out duration-300 {addCSS} {activeCSS}"
	on:click={(e) => {
		const now = new Date().getTime();

		if (now - lastClicked < DOUBLE_CLICK_THRESHOLD && onDoubleClick) onDoubleClick();
		else onClick(e);

		lastClicked = now;
	}}
>
	{#if add}
		<Plus className="flex-none h-full w-[16px]" />
	{:else}
		<FolderSolid className="flex-none h-full w-[16px]" />
	{/if}
	<p class="ml-3 grow text-[14px]font-medium truncate">{name}</p>
</button>
