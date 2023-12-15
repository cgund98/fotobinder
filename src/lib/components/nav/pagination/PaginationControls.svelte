<script lang="ts">
	import IconButton, { Variant } from '$lib/components/button/IconButton.svelte';
	import ChevronDown from '$lib/components/icons/ChevronDown.svelte';

	export let curPage = 0;
	export let pageCount = 10;
	export let pageSize = 10;
	export let maxItems = 100;

	export let scroll: boolean = true;

	const MAX_VISIBLE = 8;

	$: if (pageCount > 0 && curPage >= pageCount) curPage = pageCount - 1;

	$: pages = [...Array(pageCount).keys()]
		.filter((idx) => {
			const withinPageCount = pageCount <= MAX_VISIBLE;
			const isEnd = idx === pageCount - 1 || idx === 0;
			const curIsEnd = curPage === pageCount - 1 || curPage === 0;
			const curIsAlmostEnd = curPage === pageCount - 2 || curPage === 1;
			const curIsMiddle = 2 > Math.ceil(Math.abs(curPage - MAX_VISIBLE / 2)) ? 0 : 1;
			const isNeighbor =
				Math.abs(idx - curPage) <= (curIsEnd ? 3 : curIsAlmostEnd ? 2 : 1) + curIsMiddle;

			return withinPageCount || isEnd || isNeighbor;
		})
		.map((idx) => ({
			active: idx === curPage,
			isPlaceholder: false,
			idx
		}))
		.reduce(
			(prev, cur) => {
				if (prev.length === 0) return [cur];
				if (cur.idx - 1 !== prev[prev.length - 1].idx)
					prev = [...prev, { idx: -1, active: false, isPlaceholder: true }];
				return [...prev, cur];
			},
			[] as { idx: number; active: boolean; isPlaceholder: boolean }[]
		);

	let activeCSS = 'bg-teal-400 text-gray-900';

	const handle = (idx: number) => {
		curPage = idx;

		if (scroll) window.scrollTo(0, 0);
	};
</script>

<div class="flex flex-row justify-between items-center py-1 px-1">
	<p class="text-sm text-gray-400">
		Showing {curPage * pageSize + 1} to {Math.min((curPage + 1) * pageSize, maxItems)} of {maxItems}
		entries.
	</p>
	<div>
		<nav aria-label="Pagination" class="flex items-center text-gray-400 space-x-2">
			<IconButton
				label="Previous page"
				icon={ChevronDown}
				className="rotate-90"
				variant={Variant.Embedded}
				onClick={() => handle(curPage - 1)}
				disabled={curPage === 0}
			/>
			{#each pages as page}
				<button
					class="px-3 py-1 rounded-lg text-sm {page.active ? activeCSS : 'hover:bg-gray-700'}"
					on:click={() => handle(page.idx)}
					disabled={page.isPlaceholder}
				>
					{!page.isPlaceholder ? page.idx + 1 : '...'}
				</button>
			{/each}
			<IconButton
				label="Next page"
				icon={ChevronDown}
				className="-rotate-90"
				variant={Variant.Embedded}
				onClick={() => handle(curPage + 1)}
				disabled={curPage === pageCount - 1}
			/>
		</nav>
	</div>
</div>
