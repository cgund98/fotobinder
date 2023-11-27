<script lang="ts">
	import Check from '../../icons/Check.svelte';

	export let active: boolean = false;
	export let show: boolean = true;
</script>

<div
	role="button"
	class="border-2 border-gray-300 p-[1px] rounded-full ease-out duration-100 drop-shadow {active
		? 'bg-teal-400 border-teal-400'
		: ''} {show ? '' : 'opacity-0'}"
>
	<Check className="fill-bg-gray h-[15px] w-[15px] {active ? '' : 'opacity-0'}" />
</div>
