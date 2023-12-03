<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import FolderSolid from '../icons/FolderSolid.svelte';

	export let className: string = '';
	export let label: string = '';
	export let value: string = '';
	export let placeholder: string = 'Placeholder';
</script>

<div class="flex flex-col py-1 px-1 {className}">
	{#if label != ''}
		<h3 class="text-base font-bold text-gray-100 mb-1 px-3">
			{label}
		</h3>
	{/if}
	<div class="relative">
		<div class="flex flex-row text-gray-100 bg-gray-700 px-3 py-2 rounded-lg">
			<input
				on:click={async () => {
					const newPath = await open({
						directory: true,
						multiple: false
					});

					if (typeof newPath === 'string') value = newPath;
				}}
				{placeholder}
				readonly
				{value}
				class="placeholder:text-gray-500 cursor-pointer bg-transparent caret-gray-100 appearance-none w-full accent-transparent focus:outline-none text-base"
			/>
			<div><FolderSolid className="text-gray-500 w-[20px]" /></div>
		</div>
	</div>
</div>
