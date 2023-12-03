<script lang="ts">
	import type { FormEventHandler } from 'svelte/elements';
	import Search from '../icons/Search.svelte';

	export let className: string = '';
	export let placeholder: string;
	export let value: string = '';

	const handler: FormEventHandler<HTMLInputElement> = (e) => {
		let t = e.target as HTMLInputElement;
		value = t.value;
	};
</script>

<div class="flex flex-row justify-between border-b-[1px] py-1 px-1 border-gray-700 {className}">
	<input
		{placeholder}
		{value}
		on:input={handler}
		class="text-gray-100 placeholder:text-gray-500 bg-none bg-transparent caret-white appearance-none w-full
    accent-transparent focus:outline-none text-base"
	/>
	<Search className="text-gray-500 w-[20px]" />
</div>
