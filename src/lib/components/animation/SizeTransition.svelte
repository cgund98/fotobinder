<script lang="ts">
	import { quintOut } from 'svelte/easing';
	import { scale } from 'svelte/transition';
</script>

<div transition:scale={{ duration: 200, easing: quintOut }}>
	<slot />
</div>
