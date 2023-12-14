<script lang="ts">
	import { alerts } from '../../store/alerts';
	import Alert from './Alert.svelte';
</script>

<div class="fixed top-0 w-full mt-3 px-8 z-[151]">
	<div class="max-w-[800px] w-full mx-auto flex flex-col space-y-2">
		{#each $alerts as alert (alert.id)}
			<Alert {alert} time={5000} />
		{/each}
	</div>
</div>
