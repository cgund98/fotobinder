<script lang="ts">
	import type { ComponentType } from 'svelte';

	import Button, { Variant } from '../button/Button.svelte';
	import ChevronDown from '../icons/ChevronDown.svelte';

	export let className: string = '';
	export let label: string = 'Menu';
	export let options: {
		label: string;
		icon: ComponentType;
		action: () => void;
		disabled: boolean;
	}[];

	export let open = false;

	export let position: 'left' | 'right' = 'left';
	let positionCSS = '';
	if (position == 'left') positionCSS = 'left-0';
	if (position == 'right') positionCSS = 'right-0';

	const iconProps = {
		className: ''
	};
</script>

<div class="flex flex-col py-1 px-1 {className}">
	<div class="relative">
		<Button title={label} variant={Variant.Secondary} onClick={() => (open = !open)}>
			<ChevronDown className="w-[16px] mt-[1px]" />
		</Button>

		{#if open}
			<div
				class="absolute {positionCSS} z-10 text-base bg-gray-800 rounded-lg divide-y divide-gray-100 shadow my-2 max-h-32 overflow-y-scroll"
			>
				{#each options as option}
					<button
						class="w-36 truncate space-x-2 flex flex-row py-2 px-3 text-base text-gray-300 cursor-pointer {option.disabled
							? '!text-gray-500'
							: 'hover:text-teal-400'}"
						on:click={() => {
							open = false;
							option.action();
						}}
						disabled={option.disabled}
					>
						<svelte:component this={option.icon} {...iconProps} />
						<span>{option.label}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>
