<script>
	import '../app.css';

	import Navbar from '../lib/components/nav/navbar/Navbar.svelte';
</script>

<div class="bg-bg-gray min-h-screen flex text-gray-100 px-4">
	<div class="flex-none w-[250px] bg-dark-gray">
		<Navbar />
	</div>
	<div class="flex-1">
		<slot />
	</div>
</div>
