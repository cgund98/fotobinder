<script lang="ts">
	import Search from '../icons/Search.svelte';

	export let className: string = '';
	export let label: string;
	export let placeholder: string;
</script>

<div class="flex flex-col py-1 px-1 {className}">
	<h3 class="text-base font-bold text-gray-100 mb-1 px-3">
		{label}
	</h3>
	<div class="text-gray-100 bg-gray-700 px-3 py-2 rounded-lg">
		<input
			{placeholder}
			class="placeholder:text-gray-500 bg-transparent caret-gray-100 appearance-none w-full
        accent-transparent focus:outline-none text-base"
		/>
	</div>
</div>
