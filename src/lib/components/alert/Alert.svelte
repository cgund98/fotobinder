<script lang="ts">
	import { onDestroy } from 'svelte';

	import { alerts, type Alert, Severity } from '../../store/alerts';

	import IconButton, { Variant } from '../button/IconButton.svelte';
	import Close from '../icons/Close.svelte';

	export let alert: Alert;

	let severityCSS = 'text-green-900 bg-green-300';
	if (alert.severity == Severity.Warn) severityCSS = 'text-yellow-900 bg-yellow-300';
	if (alert.severity == Severity.Bad) severityCSS = 'text-red-900 bg-red-300';

	const removeItem = () => alerts.update((a) => a.filter((x) => x.id != alert.id));

	onDestroy(removeItem);

	// Destroy element after a timer
	export let time: number;
	setTimeout(removeItem, time);
</script>

<div
	class="w-full {severityCSS} flex flex-row items-center justify-between box-shadow rounded-lg px-3 py-3"
>
	<p class="ml-1">{alert.message}</p>
	<IconButton
		onClick={removeItem}
		variant={Variant.Embedded}
		label="close"
		icon={Close}
		className={severityCSS}
	/>
</div>
