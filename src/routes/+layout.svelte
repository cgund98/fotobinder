<script>
	import '../app.css';
	import AlertsWrapper from '$lib/components/alert/AlertsWrapper.svelte';

	import Navbar from '$lib/components/nav/navbar/Navbar.svelte';
</script>

<div class="bg-bg-gray min-h-screen flex text-gray-100">
	<div class="flex-none w-[250px] bg-dark-gray">
		<Navbar />
	</div>
	<div class="flex-1 px-2 pt-4">
		<slot />
	</div>
</div>

<AlertsWrapper />
