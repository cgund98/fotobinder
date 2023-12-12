<script lang="ts">
	import { fly } from 'svelte/transition';
	import { quintIn, quintOut } from 'svelte/easing';
</script>

<div
	in:fly={{ delay: 100, duration: 100, easing: quintIn }}
	out:fly={{ duration: 100, easing: quintOut }}
>
	<slot />
</div>
