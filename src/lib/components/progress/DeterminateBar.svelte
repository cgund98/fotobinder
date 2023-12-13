<script lang="ts">
	export let progress = 0;
	export let label: string;

	$: roundedProgress = (Math.round(progress * 100) / 100).toFixed(1);
</script>

<div class="flex justify-between mb-1">
	<span class="text-base font-medium text-gray-300">{label}</span>
	<span class="text-sm font-medium text-gray-300">{roundedProgress}%</span>
</div>
<div class="w-full bg-gray-700 rounded-full h-1">
	<div class="bg-teal-600 h-1 rounded-full" style="width: {progress}%"></div>
</div>
