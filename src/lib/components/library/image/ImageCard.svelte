<script lang="ts">
	import IconButton, { Variant } from '$lib/components/button/IconButton.svelte';
	import Photo from '$lib/components/icons/Photo.svelte';
	import CheckBoxRound from './CheckBoxRound.svelte';
	import Expand from '$lib/components/icons/Expand.svelte';

	export let onChange: (checked: boolean) => void;
	export let onView: () => void = () => {};
	export let forceHover: boolean = false;
	export let checked = false;

	let hovered = false;

	$: eitherHover = hovered || forceHover;
</script>

<div
	class="h-32 w-full drop-shadow relative"
	on:mouseenter={() => (hovered = true)}
	on:mouseleave={() => (hovered = false)}
	role="listitem"
>
	<div
		class="absolute top-0 h-full z-30 w-full ease-out duration-300 justify-around flex bg-gradient-to-b from-gray-900/70 to-gray-900/0 {eitherHover
			? 'opacity-100'
			: 'opacity-0'}"
	>
		<button
			class="h-full w-full bg-no-repeat bg-center absolute top-0 opacity-0"
			on:click={onView}
			disabled={forceHover}
		/>
		<div class="px-2 py-2 w-full flex flex-row justify-between">
			<div>
				<CheckBoxRound {onChange} active={checked} show={eitherHover} />
			</div>

			<div class="ease-out duration-300 {hovered && !forceHover ? 'opacity-100' : 'opacity-0'}">
				<IconButton
					label="View Details"
					icon={Expand}
					variant={Variant.Embedded}
					onClick={onView}
					className="text-gray-200 !w-6 !h-6"
					disabled={forceHover}
				/>
			</div>
		</div>
	</div>
	<button
		class="h-full w-full bg-no-repeat bg-center absolute top-0 z-20 opacity-0"
		on:click={onView}
		disabled={forceHover}
	/>
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
