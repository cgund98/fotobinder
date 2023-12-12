<script lang="ts">
	import { fly } from 'svelte/transition';
	import IndeterminateBar from './IndeterminateBar.svelte';
	import { quintOut } from 'svelte/easing';
</script>

<div transition:fly={{ duration: 200, easing: quintOut }} class="fixed top-0 left-0 right-0 z-90">
	<IndeterminateBar />
</div>
