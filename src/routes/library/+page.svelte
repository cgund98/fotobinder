<script lang="ts">
	import { routeToPage } from '$lib/nav/route';

	import { list } from '$lib/api/source';
	import { writable } from 'svelte/store';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import FolderCard from '$lib/components/library/folder/FolderCard.svelte';
	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import NewSourceModal from '$lib/components/library/source/NewSourceModal.svelte';
	import { type Sources, remove } from '$lib/api/source';
	import { catchBad, good } from '$lib/store/alerts';
	import Menu from '$lib/components/menu/Menu.svelte';
	import Trash from '$lib/components/icons/Trash.svelte';
	import ConfirmModal from '$lib/components/layout/ConfirmModal.svelte';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';
	import ProgressWrapper from '$lib/components/progress/ProgressWrapper.svelte';
	import ThumbnailQueueProgress from '$lib/components/progress/ThumbnailQueueProgress.svelte';
	import SizeTransition from '$lib/components/animation/SizeTransition.svelte';

	let showNewSource = false;
	let loading = false;

	/* Fetch sources */
	let sources: Sources = { sources: [] };
	let selected: undefined | string;

	const updateSources = () => {
		loading = true;
		list()
			.then((res) => {
				// Update sources
				sources = { sources: res.sources.sort((a, b) => a.name.localeCompare(b.name)) };

				// Update selection
				const newIds = new Set(sources.sources.map((s) => s.id));
				let newSelected = !selected || !newIds.has(selected) ? undefined : selected;

				loading = false;
			})
			.catch((err) => {
				catchBad(err);
				loading = false;
			});
	};

	updateSources();

	/* Menu Options */
	$: selectedName = sources.sources.filter((s) => s.id === selected)[0]?.name;
	const deleteSources = async () => {
		if (!selected) return;

		loading = true;
		try {
			await remove(selected);
			good(`Successfully deleted '${selectedName}''.`);
			updateSources();
		} catch (err) {
			catchBad(err);
		}
		loading = false;
	};

	$: menuOptions = [
		{
			label: 'Delete',
			icon: Trash,
			action: () => (showConfirmDelete = true),
			disabled: selected === undefined
		}
	];

	// Confirm
	let showConfirmDelete = false;
</script>

{#if loading}
	<ProgressWrapper />
{/if}

<PageTransitionWrapper>
	<PathHeader path={[{ label: 'My Library', route: '/library' }]} />

	<div class="flex justify-between pb-1 px-2">
		<div class="flex flex-col justify-end">
			<p class="text-gray-500 text-base">
				Select or add an image source. Double-click to view a source's contents.
			</p>
		</div>

		<div class="flex flex-row space-x-3">
			<Menu label="Actions" options={menuOptions} position="right" />
		</div>
	</div>

	<Separator className="my-2" />

	<div class="w-full flex flex-wrap">
		{#each sources.sources as source}
			<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
				<FolderCard
					active={selected === source.id}
					onClick={() => {
						if (selected === source.id) selected = undefined;
						else selected = source.id;
					}}
					onDoubleClick={() => routeToPage(`/library/${source.id}`, { subpath: '' })}
					name={source.name}
				/>
			</div>
		{/each}

		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard onClick={() => (showNewSource = true)} name="Add Source" add />
		</div>
	</div>
</PageTransitionWrapper>

<ThumbnailQueueProgress />

{#if showNewSource}
	<NewSourceModal
		onClose={() => {
			showNewSource = false;
			updateSources();
		}}
	/>
{/if}

{#if showConfirmDelete}
	<ConfirmModal
		title="Delete Source"
		message={`Are you sure you want to delete '${selectedName}'`}
		onClose={() => (showConfirmDelete = false)}
		onConfirm={() => {
			deleteSources();
			showConfirmDelete = false;
		}}
	/>
{/if}
