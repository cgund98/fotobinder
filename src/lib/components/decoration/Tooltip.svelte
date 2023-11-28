<script lang="ts">
	export let title = '';
	let isHovered = false;
	let x: number;
	let y: number;

	const mouseOver = (event: MouseEvent) => {
		isHovered = true;
		x = event.pageX + 5;
		y = event.pageY + 5;
	};
	function mouseMove(event: MouseEvent) {
		x = event.pageX + 5;
		y = event.pageY + 5;
	}
	function mouseLeave() {
		isHovered = false;
	}
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

{#if isHovered}
	<div
		style="top: {y}px; left: {x}px;"
		class="fixed box-shadow rounded-md px-2 py-1 text-sm bg-teal-900 z-50"
	>
		{title}
	</div>
{/if}
