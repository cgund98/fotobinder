<script lang="ts">
	export let label: string;
	export let href: string;
</script>

<div>
	<a {href} class="text-base font-regular text-gray-300">{label}</a>
</div>
