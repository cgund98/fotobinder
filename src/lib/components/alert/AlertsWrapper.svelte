<script lang="ts">
	import { alerts } from '../../store/alerts';
	import Alert from './Alert.svelte';
</script>

<div class="fixed top-0 w-full mt-3 px-8 flex flex-col space-y-2">
	{#each $alerts as alert}
		<Alert {alert} time={10000} />
	{/each}
</div>
