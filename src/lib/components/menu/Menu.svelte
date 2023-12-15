<script lang="ts">
	import type { ComponentType } from 'svelte';

	import Button, { Variant } from '../button/Button.svelte';
	import ChevronDown from '../icons/ChevronDown.svelte';
	import Separator from '../decoration/Separator.svelte';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

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
		className: '!w-5 !h-5'
	};
</script>

<div class="flex flex-col {className}">
	<div class="relative">
		<Button title={label} variant={Variant.Secondary} onClick={() => (open = !open)}>
			<ChevronDown className="w-[16px] mt-[1px]" />
		</Button>

		{#if open}
			<div
				transition:scale={{ duration: 200, easing: quintOut }}
				class="absolute {positionCSS} z-10 text-base bg-gray-800 rounded-lg divide-y divide-gray-100 shadow my-2 overflow-y-scroll"
			>
				<div class="flex flex-col">
					{#each options as option, idx}
						<button
							class="truncate border-none space-x-2 flex flex-row py-2 px-3 font-medium text-base text-gray-300 cursor-pointer ease-out duration-100 {option.disabled
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
						{#if idx != options.length - 1}
							<Separator className="bg-gray-700 opacity-50" />
						{/if}
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>
