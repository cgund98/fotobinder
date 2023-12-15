<script lang="ts">
	import { quintOut } from 'svelte/easing';
	import { slide } from 'svelte/transition';
</script>

<div
	transition:slide={{ duration: 200, easing: quintOut }}
	class="border-solid border-l-[1px] -mt-2 border-teal-700 flex-col ml-[30px] pl-4 space-y-1 mb-4"
>
	<slot />
</div>
