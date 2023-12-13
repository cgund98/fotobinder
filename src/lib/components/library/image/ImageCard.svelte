<script lang="ts">
	import Lazy from 'svelte-lazy';

	import IconButton, { Variant } from '$lib/components/button/IconButton.svelte';
	import Photo from '$lib/components/icons/Photo.svelte';
	import CheckBoxRound from './CheckBoxRound.svelte';
	import Expand from '$lib/components/icons/Expand.svelte';

	export let onChange: (checked: boolean) => void;
	export let onView: () => void = () => {};
	export let forceHover: boolean = false;
	export let checked = false;
	export let name: string;

	let hovered = false;

	$: eitherHover = hovered || forceHover;
	$: handleBgClick = forceHover ? () => onChange(!checked) : onView;
</script>

<Lazy height={128} offset={300} fadeOption={{ duration: 200 }}>
	<div
		class="h-32 w-full drop-shadow relative"
		on:mouseenter={() => (hovered = true)}
		on:mouseleave={() => (hovered = false)}
		role="listitem"
	>
		<div
			class="absolute top-0 h-full z-30 w-full ease-out duration-200 justify-around flex bg-gradient-to-b from-gray-900/40 to-gray-900/0 {eitherHover
				? 'opacity-100'
				: 'opacity-0'}"
		>
			<button
				class="h-full w-full bg-no-repeat bg-center absolute top-0 opacity-0"
				on:click={handleBgClick}
			/>
			<div class="px-2 py-2 w-full flex flex-row justify-between">
				<div class={eitherHover ? 'opacity-100' : 'opacity-0'}>
					<CheckBoxRound {onChange} active={checked} />
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
			on:click={handleBgClick}
		/>
		<div class="h-full w-full image-card bg-cover bg-no-repeat bg-center absolute top-0 z-10" />
		<div class="h-full w-full absolute top-0 z-0 flex bg-gray-900 justify-around">
			<div class="flex flex-col justify-around h-full w-full">
				<div class="flex flex-col w-full items-center">
					<Photo className="text-gray-600 " />
					<p class="truncate w-full text-gray-600 px-4 text-sm">{name}</p>
				</div>
			</div>
		</div>
	</div>
</Lazy>

<style>
	.image-card {
		background-image: var(--src);
	}
</style>
