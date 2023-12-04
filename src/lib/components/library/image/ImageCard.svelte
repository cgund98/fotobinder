<script lang="ts">
	import Photo from '$lib/components/icons/Photo.svelte';
	import CheckBoxRound from './CheckBoxRound.svelte';

	export let checked: boolean = false;

	export let onView: () => void = () => {};

	let hovered = false;
</script>

<div
	class="h-32 w-full drop-shadow relative"
	on:mouseenter={() => {
		hovered = true;
	}}
	on:mouseleave={() => {
		hovered = false;
	}}
	role="listitem"
>
	<div class="absolute top-0 z-20 w-full h-full ease-out {hovered ? 'bg-gray-900/20' : ''}">
		<div role="button" on:click={onView} class="absolute top-2 left-2">
			<CheckBoxRound active={checked} show={hovered} />
		</div>
	</div>
	<div class="h-full w-full image-card bg-cover bg-no-repeat bg-center absolute top-0 z-10" />
	<div class="h-full w-full absolute top-0 z-0 flex bg-gray-900 justify-around">
		<div class="flex flex-col justify-around h-full">
			<Photo className="text-gray-600" />
		</div>
	</div>
</div>

<style>
	.image-card {
		background-image: var(--src);
	}
</style>
