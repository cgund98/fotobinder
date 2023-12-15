<script lang="ts">
	import { scale } from 'svelte/transition';
	import ChevronDown from '../icons/ChevronDown.svelte';
	import { quintOut } from 'svelte/easing';

	export let className: string = '';
	export let label: string = '';
	export let placeholder: string = '';
	export let options: { label: string; value: string }[];
	export let value: string;
	export let onChange: (value: string) => void = () => {};

	let open = false;

	// Parse label from current set of options
	$: curLabel = options.filter((o) => o.value === value)[0]?.label;
</script>

<div class="flex flex-col py-1 px-1 {className}">
	{#if label != ''}
		<h3 class="text-base font-bold text-gray-100 mb-1 px-3">
			{label}
		</h3>
	{/if}
	<div class="relative">
		<button
			on:click={() => (open = !open)}
			class="w-full text-left flex flex-row items-center text-gray-100 bg-gray-700 px-3 py-2 rounded-lg"
		>
			<p
				class="placeholder:text-gray-500 bg-transparent caret-gray-100 appearance-none w-full accent-transparent focus:outline-none text-base"
			>
				{curLabel ? curLabel : placeholder}
			</p>
			<div class="flex flex-col justify-around">
				<ChevronDown className="text-gray-500 w-[20px]" />
			</div>
		</button>

		{#if open}
			<div
				transition:scale={{ duration: 200, easing: quintOut }}
				class="z-10 inset-x-0 border-2 border-gray-700 text-base list-none bg-gray-800 rounded-lg divide-y divide-gray-100 shadow-xl absolute my-2 max-h-32 overflow-y-auto"
			>
				<ul class="">
					{#each options as option}
						<li
							role="option"
							aria-selected={value == option.value}
							class="block py-2 px-4 text-base text-gray-300 hover:bg-gray-600 cursor-pointer"
							on:click|preventDefault={() => {
								value = option.value;
								open = false;
								onChange(option.value);
							}}
							on:keydown|preventDefault={() => {}}
						>
							{option.label}
						</li>
					{/each}
				</ul>
			</div>
		{/if}
	</div>
</div>
