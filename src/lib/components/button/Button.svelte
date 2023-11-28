<script lang="ts" context="module">
	export enum Variant {
		Primary = 'primary',
		Secondary = 'secondary',
		Warn = 'warn'
	}

	export let disabled = false;
</script>

<script lang="ts">
	export let className: string = '';
	export let title: string;

	export let variant: Variant;

	const primaryCSS = 'bg-teal-400 text-gray-800';
	const secondaryCSS = 'bg-gray-700 text-teal-400';
	const warnCSS = 'bg-red-400 text-gray-800';

	let css = primaryCSS;
	if (variant == Variant.Secondary) css = secondaryCSS;
	else if (variant == Variant.Warn) css = warnCSS;

	console.log('Variant:');
	console.log(css);
</script>

<button
	{disabled}
	class="py-[6px] px-3 flex shadow font-medium rounded-md disabled:opacity-75 disabled:shadow-none space-x-2 {css} {className}"
>
	<span class="text-[14px]">{title}</span>
	<slot />
</button>
