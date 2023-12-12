<script lang="ts">
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	import Backdrop from './Backdrop.svelte';
</script>

<Backdrop>
	<div
		transition:scale={{ delay: 100, duration: 200, easing: quintOut }}
		class="w-screen sm:w-[500px] md:w-[600px] lg:w-[750px] max-h-[90vh] bg-gray-800 rounded-md px-4 py-2 pb-4 grow-0"
	>
		<slot />
	</div>
</Backdrop>
