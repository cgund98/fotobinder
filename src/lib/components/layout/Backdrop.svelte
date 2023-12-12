<script lang="ts">
	import { fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
</script>

<div
	transition:fade={{ duration: 150, easing: quintOut }}
	class="fixed top-0 left-0 h-screen w-screen flex bg-black/70"
>
	<div class="h-full mx-auto flex flex-col justify-center">
		<slot />
	</div>
</div>
