<script lang="ts">
	import ChevronDown from '../icons/ChevronDown.svelte';

	export let className: string = '';
	export let label: string = '';
	export let placeholder: string = '';
	export let open: boolean = false;
	export let options: { label: string; value: string }[];
	export let value: string;

	// Parse label from current set of options
	const curLabel = options.filter((o) => o.value === value)[0]?.label;
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
				{placeholder}
				value={curLabel}
				class="placeholder:text-gray-500 bg-transparent caret-gray-100 appearance-none w-full accent-transparent focus:outline-none text-base"
			/>
			<div class="flex flex-col justify-around">
				<ChevronDown className="text-gray-500 w-[20px]" />
			</div>
		</div>

		{#if open}
			<div
				class="z-10 inset-x-0 text-base list-none bg-gray-700 rounded-lg divide-y divide-gray-100 shadow-xl absolute my-2 max-h-32 overflow-y-scroll"
			>
				<ul class="">
					{#each options as option}
						<li class="block py-2 px-4 text-base text-gray-300 hover:bg-gray-600 cursor-pointer">
							{option.label}
						</li>
					{/each}
				</ul>
			</div>
		{/if}
	</div>
</div>
