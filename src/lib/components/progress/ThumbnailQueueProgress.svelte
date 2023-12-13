<script lang="ts">
	import { onDestroy } from 'svelte';
	import DeterminateBar from './DeterminateBar.svelte';
	import { getThumbnailQueueSize } from '$lib/api/fs_entry';
	import { catchBad } from '$lib/store/alerts';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	let poller: number | undefined;
	let total = 0;
	let cur = 0;

	// Load progress from local storage
	const TOTAL_KEY = 'thumbnail.total';
	const CUR_KEY = 'thumbnail.cur';

	const get = () => {
		total = parseInt(localStorage.getItem(TOTAL_KEY) || '0');
		cur = parseInt(localStorage.getItem(CUR_KEY) || '0');
	};

	const save = () => {
		localStorage.setItem(TOTAL_KEY, JSON.stringify(total));
		localStorage.setItem(CUR_KEY, JSON.stringify(cur));
	};

	// Polling
	const poll = async () => {
		if (poller) clearInterval(poller);
		get();
		try {
			console.log('ping!');
			const size = await getThumbnailQueueSize();

			if (size === 0) {
				total = 0;
				cur = 0;
				poller = setTimeout(poll, 5000);
				save();
				return;
			} else if (size > total) {
				total = size;
			} else {
				cur = total - size;
			}
		} catch (err) {
			catchBad(err);
		}

		save();

		poller = setTimeout(poll, 1000);
	};

	poll();
	onDestroy(() => clearTimeout(poller));
</script>

{#if total > 0}
	<div class="fixed bottom-2 left-2 right-2 bg-gray-800 shadow-md rounded-lg p-2">
		<DeterminateBar label="Creating thumbnails..." progress={(100 * cur) / total} />
	</div>
{/if}
