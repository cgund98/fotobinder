<script lang="ts">
	import FolderSolid from '../../icons/FolderSolid.svelte';

	export let name: string;
</script>

<div
	class="rounded-md flex h-full justify-items-center bg-gray-700 text-gray-300 fill-gray-400 py-2 px-4 drop-shadow"
>
	<FolderSolid className="flex-none h-full w-[16px]" />
	<p class="ml-3 grow text-[14px] font-medium truncate">{name}</p>
</div>
