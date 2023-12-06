<script lang="ts">
	import Check from '../../icons/Check.svelte';

	export let show: boolean = true;
	export let onChange: (checked: boolean) => void = () => {};
	export let active: boolean = false;
</script>

<button
	class="border-2 border-gray-300 p-[1px] rounded-full ease-out duration-200 drop-shadow {active
		? 'bg-teal-400 border-teal-400'
		: ''} {show ? '' : 'opacity-0'}"
	on:click={() => {
		onChange(!active);
	}}
>
	<Check
		className="fill-bg-gray ease-out duration-200  h-[15px] w-[15px] {active ? '' : 'opacity-0'}"
	/>
</button>
