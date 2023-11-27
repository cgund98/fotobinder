<script lang="ts">
	import ChevronDown from '../../icons/ChevronDown.svelte';

	export let title: string;
	export let href: string;
</script>

<div class="flex flex-row justify-between pr-3 pl-5 py-4">
	<a {href} class="flex">
		<div>
			<slot />
		</div>
		<p class="ml-3 text-[15px] font-medium text-gray-300">{title}</p>
	</a>
	<div>
		<ChevronDown className="stroke-gray-600 w-[20px]" />
	</div>
</div>
