<script lang="ts">
	import path from 'path-browserify';
	import { appDataDir, join } from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	import { writable } from 'svelte/store';
	import { page } from '$app/stores';

	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import FolderCard from '$lib/components/library/folder/FolderCard.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import ImageCard from '$lib/components/library/image/ImageCard.svelte';
	import EditTagsModal from '$lib/components/tags/EditTagsModal.svelte';
	import NewTagModal from '$lib/components/tags/NewTagModal.svelte';
	import Button from '$lib/components/button/Button.svelte';
	import Tag from '$lib/components/icons/Tag.svelte';
	import ImageDetailsCard from '$lib/components/library/image/ImageDetailsCard.svelte';
	import AddToCollectionModal from '$lib/components/collections/AddToCollectionModal.svelte';
	import type { HeaderEntry } from '$lib/components/library/header/PageHeader';

	import type { PageData } from './$types';
	import { catchBad, good } from '$lib/store/alerts';
	import { scan, get as getSource, type Source } from '$lib/api/source';
	import { FileType, list_by_source_id } from '$lib/api/fs_entry';
	import Menu from '$lib/components/menu/Menu.svelte';
	import FolderSolid from '$lib/components/icons/FolderSolid.svelte';
	import Reset from '$lib/components/icons/Reset.svelte';
	import { routeToPage } from '$lib/nav/route';

	export let data: PageData;

	$: subpath = $page.url.searchParams.get('subpath') || '';

	interface Folder {
		name: string;
	}
	interface Images {
		src: string;
	}

	const folders = writable<Folder[]>([]);
	const images = writable<Images[]>([]);
	const source = writable<Source>();
	const navEntries = writable<HeaderEntry[]>([]);

	const updateNav = () => {
		let parts = subpath.split(path.sep);

		let partsPaths = parts.map((_, idx) => parts.slice(0, idx + 1).join(path.sep));

		navEntries.set([
			{ label: 'My Library', route: '/library' },
			{ label: $source.name, route: `/library/${$source.id}` },
			...partsPaths
				.filter((p) => p)
				.map((p, idx) => ({
					label: parts[idx],
					route: `/library/${$source.id}`,
					queryParams: { subpath: p }
				}))
		]);
	};

	// Fetch data
	const fetchSource = () => {
		getSource(data.sourceId)
			.then((res) => {
				source.set(res);
				updateNav();
			})
			.catch(catchBad);
	};

	const fetchEntries = (subpath: string) => {
		const doFn = async () => {
			const res = await list_by_source_id(data.sourceId, subpath);

			const newFolders = res.entries
				.filter((e) => e.fs_type == FileType.Directory)
				.map((e) => ({ name: e.name }));

			const dataDir = await appDataDir();
			const mapFileSrc = async (p: string): Promise<string> => {
				return convertFileSrc(await join(dataDir, 'thumbnails', p));
			};

			const newImages = await Promise.all(
				res.entries
					.filter((e) => e.fs_type == FileType.File)
					.map(async (e) => ({
						src: await mapFileSrc(e.thumbnail_path)
					}))
			);

			folders.set(newFolders);
			images.set(newImages);
			updateNav();
		};
		doFn().catch(catchBad);
	};

	// Initialize fields
	$: fetchSource();
	$: fetchEntries(subpath);

	// Menu options
	$: menuOptions = [
		{
			label: 'Add to Collection',
			icon: FolderSolid,
			action: () => (showAddToCollection = true),
			disabled: false
		},
		{
			label: 'Scan for Files',
			icon: Reset,
			action: () =>
				scan(data.sourceId)
					.then(() => {
						good('Successfully scanned for new files.');
						fetchEntries(subpath);
					})
					.catch(catchBad),
			disabled: false
		}
	];

	// Modals
	let showEditTags = false;
	let showNewTag = false;
	let showImageDetails = false;
	let showAddToCollection = false;
</script>

<PathHeader bind:path={$navEntries} />

<div class="flex justify-between py-1 px-2 mt-2">
	<div class="flex flex-col justify-end">
		<p class="text-gray-500 text-base">8 Items Selected</p>
	</div>

	<div class="flex flex-row space-x-3 items-center">
		<Button
			title="Modify Tags"
			className="disabled:bg-teal-700"
			onClick={() => {
				showEditTags = true;
				console.log(showEditTags);
			}}
		>
			<Tag className="w-[15px] -mt-[1px]" />
		</Button>
		<Menu label="Actions" options={menuOptions} position="right" />
	</div>
</div>

<Separator className="my-2" />

<div class="w-full flex flex-wrap">
	{#each $folders as folder}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard
				name={folder.name}
				onClick={() => console.log('click')}
				onDoubleClick={() =>
					routeToPage(`/library/${data.sourceId}`, { subpath: path.join(subpath, folder.name) })}
			/>
		</div>
	{/each}
</div>

{#if $images.length && $folders.length}
	<Separator className="my-2" />
{/if}

<div class="w-full flex flex-wrap mt-1">
	{#each $images as image}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-1">
			<ImageCard
				onView={() => {
					showImageDetails = true;
				}}
				--src="url('{image.src}')"
				--color="red"
			/>
		</div>
	{/each}
</div>

{#if showEditTags}
	<EditTagsModal
		onClose={() => {
			showEditTags = false;
		}}
		onNewTag={() => {
			showNewTag = true;
		}}
	/>
{/if}

{#if showNewTag}
	<NewTagModal
		onClose={() => {
			showNewTag = false;
		}}
	/>
{/if}

{#if showImageDetails}
	<ImageDetailsCard
		onClose={() => {
			showImageDetails = false;
		}}
		name="Mountain.jpg"
		src="/image/mountain.jpg"
	/>
{/if}

{#if showAddToCollection}
	<AddToCollectionModal
		onClose={() => {
			showAddToCollection = false;
		}}
	/>
{/if}
