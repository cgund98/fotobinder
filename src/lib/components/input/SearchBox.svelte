<script lang="ts">
	import Search from '../icons/Search.svelte';

	export let className: string = '';
	export let label: string;
	export let placeholder: string;
	export let open: boolean = false;

	const items = [
		{ label: 'Item' },
		{ label: 'Item' },
		{ label: 'Item' },
		{ label: 'Item' },
		{ label: 'Item' }
	];
</script>

<div class="flex flex-col py-1 px-1 {className}">
	<h3 class="text-base font-bold text-gray-100 mb-1 px-3">
		{label}
	</h3>
	<div class="relative">
		<div class="flex flex-row text-gray-100 bg-gray-700 px-3 py-2 rounded-lg">
			<input
				{placeholder}
				class="placeholder:text-gray-500 bg-transparent caret-gray-100 appearance-none w-full accent-transparent focus:outline-none text-base"
			/>
			<div><Search className="text-gray-500 w-[20px]" /></div>
		</div>

		{#if open}
			<div
				class="z-10 inset-x-0 text-base list-none bg-gray-700 rounded-lg divide-y divide-gray-100 shadow-xl absolute my-2 max-h-32 overflow-y-scroll"
			>
				<ul class="">
					{#each items as item}
						<li class="block py-2 px-4 text-base text-gray-300 hover:bg-gray-600 cursor-pointer">
							{item.label}
						</li>
					{/each}
				</ul>
			</div>
		{/if}
	</div>
</div>
