<script lang="ts" context="module">
	export enum Variant {
		Primary = 'primary',
		Secondary = 'secondary',
		Warn = 'warn',
		Gray = 'gray'
	}
</script>

<script lang="ts">
	import type { MouseEventHandler } from 'svelte/elements';

	export let className: string = '';
	export let title: string;
	export let variant: Variant;
	export let disabled = false;
	export let shadow = true;

	export let onClick: MouseEventHandler<HTMLButtonElement> = () => {};

	const primaryCSS = 'bg-teal-400 text-gray-800';
	const secondaryCSS = 'bg-gray-700 text-teal-500';
	const warnCSS = 'bg-red-400 text-gray-800';
	const grayCSS = 'bg-gray-700 text-gray-300';

	let css = primaryCSS;
	if (variant == Variant.Secondary) css = secondaryCSS;
	else if (variant == Variant.Warn) css = warnCSS;
	else if (variant == Variant.Gray) css = grayCSS;

	const shadowCSS = shadow ? 'drop-shadow' : '';
</script>

<button
	{disabled}
	on:click={onClick}
	class="h-9 items-center px-3 flex {shadowCSS} font-medium rounded-md disabled:opacity-50 disabled:shadow-none space-x-2 {css} {className}"
>
	<span class="text-[14px] -mt-[1px]">{title}</span>
	<slot />
</button>
