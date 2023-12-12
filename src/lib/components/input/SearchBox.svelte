<script lang="ts">
	import type { FormEventHandler } from 'svelte/elements';
	import Search from '../icons/Search.svelte';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	export let className: string = '';
	export let label: string = '';
	export let placeholder: string = 'Placeholder';
	export let open: boolean = false;
	export let value: string | undefined = '';
	export let inputValue: string = '';
	export let onChange: (value: string | undefined) => void = () => {};

	interface Option {
		label: string;
		value?: string;
	}

	export let options: Option[];

	const handler: FormEventHandler<HTMLInputElement> = (e) => {
		let t = e.target as HTMLInputElement;

		// Update displayed value
		inputValue = t.value;

		// Update actual value
		const hasMatch = options.reduce((cur, prev) => cur || prev.label == t.value, true);
		if (hasMatch)
			value = options.reduce(
				(cur, prev) => (cur ? cur : prev.label == t.value ? prev.value : ''),
				'' as string | undefined
			);
		else value = '';

		onChange(value);
	};

	$: inputValue = options.reduce(
		(prev, cur) => (cur.value == value ? cur.label : prev),
		inputValue
	);

	$: filteredOptions = inputValue
		? options.filter((option) =>
				option.label.toLocaleLowerCase().includes(inputValue.toLocaleLowerCase())
		  )
		: options;
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
				value={inputValue}
				on:input={handler}
				on:focus={() => (open = true)}
				on:blur={() => (open = false)}
				class="placeholder:text-gray-500 bg-transparent caret-gray-100 appearance-none w-full accent-transparent focus:outline-none text-base"
			/>
			<div><Search className="text-gray-500 w-[20px]" /></div>
		</div>

		{#if open}
			<div
				transition:scale={{ duration: 200, easing: quintOut }}
				class="z-10 inset-x-0 border-2 border-gray-700 text-base list-none bg-gray-800 rounded-lg divide-y divide-gray-100 shadow-xl absolute my-2 max-h-32 overflow-y-scroll"
			>
				<ul class="">
					{#each filteredOptions as option (option.value)}
						<button
							class="text-left block w-full py-2 px-4 text-base text-gray-300 hover:bg-gray-600 cursor-pointer {option.value
								? ''
								: 'italic'}"
							on:mousedown={() => {
								value = option.value;
								inputValue = option.label;
								open = false;
								onChange(value);
							}}
						>
							{option.label}
						</button>
					{/each}
				</ul>
			</div>
		{/if}
	</div>
</div>
