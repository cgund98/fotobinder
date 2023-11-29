<script lang="ts">
	import FolderSolid from '../../icons/FolderSolid.svelte';
	import Plus from '../../icons/Plus.svelte';

	export let name: string;
	export let add: boolean = false;

	let addCSS = '';
	if (add) addCSS = 'text-teal-400 bg-gray-700';
</script>

<div
	role="button"
	class="cursor-pointer rounded-md flex h-full justify-items-center bg-gray-700 text-gray-300 fill-gray-400 py-3 px-4 drop-shadow {addCSS}"
>
	{#if add}
		<Plus className="flex-none h-full w-[16px]" />
	{:else}
		<FolderSolid className="flex-none h-full w-[16px]" />
	{/if}
	<p class="ml-3 grow text-[14px] font-medium truncate">{name}</p>
</div>
