<script lang="ts">
	import Search from '../icons/Search.svelte';

	export let className: string = '';

	export let placeholder: string;
</script>

<div class="flex flex-row justify-between border-b-[1px] py-1 px-1 border-gray-700 {className}">
	<input
		{placeholder}
		class="text-gray-100 placeholder:text-gray-500 bg-none bg-transparent caret-white appearance-none w-full
    accent-transparent focus:outline-none text-[15px]"
	/>
	<Search className="text-gray-500 w-[20px]" />
</div>
