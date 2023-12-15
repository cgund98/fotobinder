<script lang="ts">
	import { onMount } from 'svelte';
	import { useZoomImageWheel } from '@zoom-image/svelte';
	import { fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	export let src: string;
	export let height: number;
	export let width: number;

	let container: HTMLDivElement;
	const { createZoomImage } = useZoomImageWheel();

	onMount(() => {
		createZoomImage(container, { maxZoom: 32, wheelZoomRatio: 0.25 });
	});

	let imgHeight = '95vh';
	let imgWidth = '100%';

	const setDimensions = () => {
		let windowX = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
		let windowY = Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0);

		const parentWidth = windowX * 0.95 - 250;
		const parentHeight = windowY * 0.95;

		const scaledWidth = Math.ceil((width * parentHeight) / height);
		const scaledHeight = Math.ceil((height * parentWidth) / width);

		if (height > width) {
			if (height > windowY) imgWidth = `${scaledWidth}px`;
		} else {
			if (parentHeight / parentWidth < height / width) {
				imgWidth = `${scaledWidth}px`;
			} else {
				if (width > parentWidth) imgHeight = `${scaledHeight}px`;
			}
		}
	};

	$: setDimensions();
</script>

<div transition:fade={{ duration: 150, easing: quintOut }}>
	<div bind:this={container}>
		<img class="block" style:width={imgWidth} style:height={imgHeight} alt="Large Pic" {src} />
	</div>
</div>
