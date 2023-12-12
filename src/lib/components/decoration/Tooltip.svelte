<script lang="ts">
	import { quintOut } from 'svelte/easing';
	import { scale } from 'svelte/transition';

	export let title = '';
	export let delay = 750;
	let show = false;
	let isHovered = false;
	let x: number = 0;
	let y: number = 0;

	const HORIZ_THRESHOLD = 200;
	const VERT_THRESHOLD = 100;

	let horCSS = '';
	let vertCSS = '';

	const setCSS = () => {
		horCSS = `left: ${x}px;`;
		const fromRight = window.innerWidth - x;
		if (fromRight < HORIZ_THRESHOLD) horCSS = `right: ${fromRight}px;`;

		vertCSS = `top: ${y}px;`;
		const fromBottom = window.innerHeight - y;
		if (fromBottom < VERT_THRESHOLD) vertCSS = `bottom: ${fromBottom}px;`;
	};

	const mouseOver = (event: MouseEvent) => {
		isHovered = true;
		x = event.pageX;
		y = event.pageY;

		setCSS();

		// Wait to see if user holds mouse before showing the tooltip
		if (!show)
			setTimeout(() => {
				show = isHovered;
			}, delay);
	};
	const mouseMove = (event: MouseEvent) => {
		x = event.pageX;
		y = event.pageY;

		setCSS();
	};
	const mouseLeave = () => {
		show = false;
		isHovered = false;
	};
</script>

<div
	role="tooltip"
	on:focus={mouseLeave}
	on:mouseover={mouseOver}
	on:mouseleave={mouseLeave}
	on:mousemove={mouseMove}
	class="relative"
>
	<slot />
</div>

{#if show}
	<div
		transition:scale={{ duration: 100, easing: quintOut }}
		style="{vertCSS}; {horCSS}"
		class="fixed box-shadow rounded-md px-2 py-1 text-sm bg-teal-900 z-50 text-gray-100"
	>
		{title}
	</div>
{/if}
