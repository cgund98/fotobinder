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
	import Photo from '$lib/components/icons/Photo.svelte';
	import ImageDetailsCard from '$lib/components/library/image/ImageDetailsCard.svelte';
	import AddToCollectionModal from '$lib/components/collections/AddToCollectionModal.svelte';
	import type { HeaderEntry } from '$lib/components/library/header/PageHeader';

	import type { PageData } from './$types';
	import { catchBad, good } from '$lib/store/alerts';
	import { scan, get as getSource, type Source } from '$lib/api/source';
	import { FileType, generate_missing_thumbnails, list_by_source_id } from '$lib/api/fs_entry';
	import Menu from '$lib/components/menu/Menu.svelte';
	import FolderSolid from '$lib/components/icons/FolderSolid.svelte';
	import Reset from '$lib/components/icons/Reset.svelte';
	import { routeToPage } from '$lib/nav/route';

	// Read inputs
	export let data: PageData;

	$: subpath = $page.url.searchParams.get('subpath') || '';

	interface Folder {
		id: string;
		name: string;
	}
	interface Images {
		id: string;
		src: string;
	}

	// Selection
	let selectedImages = new Set<String>();
	let selectedFolders = writable<Set<String>>(new Set());

	// Fetch entries
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

			// Parse folders
			const newFolders = res.entries
				.filter((e) => e.fs_type == FileType.Directory)
				.map((e) => {
					let parts = e.relative_path.split(path.sep);
					return { name: parts[parts.length - 1], id: e.relative_path };
				});

			// Filter selection
			let folderIds = newFolders.reduce((s, i) => s.add(i.id), new Set<String>());
			selectedFolders.set(
				new Set(...Object.keys($selectedFolders).filter((i) => folderIds.has(i)))
			);

			// Map thumbnail paths to asset url
			const dataDir = await appDataDir();
			const mapFileSrc = async (p: string): Promise<string> => {
				return convertFileSrc(await join(dataDir, 'thumbnails', p));
			};

			// Parse images
			const newImages = await Promise.all(
				res.entries
					.filter((e) => e.fs_type == FileType.File)
					.map(async (e) => ({
						src: await mapFileSrc(e.thumbnail_path),
						id: e.relative_path
					}))
			);

			// Filter selection
			let imageIds = newImages.reduce((s, i) => s.add(i.id), new Set<String>());
			selectedImages = new Set(...Object.keys(selectedImages).filter((i) => imageIds.has(i)));

			// Update values
			folders.set(newFolders);
			images.set(newImages);
			updateNav();
		};
		doFn().catch(catchBad);
	};

	// Initialize fields
	$: fetchSource();
	$: fetchEntries(subpath);
	$: imagesSelected = selectedImages.size > 0;
	$: foldersSelected = $selectedFolders.size > 0;
	$: noSelection = !imagesSelected && !foldersSelected;

	// Menu options
	$: menuOptions = [
		{
			label: 'Add to Collection',
			icon: FolderSolid,
			action: () => (showAddToCollection = true),
			disabled: noSelection
		},
		{
			label: 'Scan for Files',
			icon: Reset,
			action: () =>
				scan(data.sourceId)
					.then((res) => {
						let msg = `Found ${res.entries_created} new entries and \
							deleted ${res.entries_deleted} outdated entries.`;

						if (res.thumbnails_created)
							msg += ` Generating ${res.thumbnails_created} \
							thumbnails in the background...`;
						good(msg);
						fetchEntries(subpath);
					})
					.catch(catchBad),
			disabled: false
		},
		{
			label: 'Fix Thumbnails',
			icon: Photo,
			action: () => {
				generate_missing_thumbnails(data.sourceId)
					.then((res) => {
						if (res > 0) good(`Generating ${res} missing thumbnails in background...`);
						else good(`Found no missing thumbnails!`);
					})
					.catch(catchBad);
			},
			disabled: false
		}
	];

	// Modals
	let showEditTags = false;
	let showNewTag = false;
	let showImageDetails = false;
	let showAddToCollection = false;

	$: console.log($selectedFolders);
</script>

<PathHeader bind:path={$navEntries} />

<div class="flex justify-between pb-1 px-2">
	<div class="flex flex-col justify-end">
		<p class="text-gray-500 text-base">
			{#if noSelection}
				No items selected. <button
					class="text-teal-400 font-medium ml-2"
					on:click={() => (selectedImages = new Set($images.map((i) => i.id)))}>Select all</button
				>
			{:else if foldersSelected}
				{$selectedFolders.size} folders selected.
				<button
					class="text-teal-400 font-medium ml-2"
					on:click={() => selectedFolders.set(new Set())}
					>Deselect all
				</button>
			{:else if imagesSelected}
				{selectedImages.size} images selected.
				<button class="text-teal-400 font-medium ml-2" on:click={() => (selectedImages = new Set())}
					>Deselect all
				</button>
			{/if}
		</p>
	</div>

	<div class="flex flex-row space-x-3 items-center">
		<Button
			title="Modify Tags"
			className="disabled:bg-teal-700"
			onClick={() => {
				showEditTags = true;
				console.log(showEditTags);
			}}
			disabled={noSelection}
		>
			<Tag className="w-[15px] -mt-[1px]" />
		</Button>
		<Menu label="Actions" options={menuOptions} position="right" />
	</div>
</div>

<Separator className="my-2" />

{#if $folders.length === 0 && $images.length === 0}
	<p class="text-left px-2">Folder is empty.</p>
{/if}

<div class="w-full flex flex-wrap">
	{#each $folders as folder (folder.id)}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard
				name={folder.name}
				onClick={() => {
					selectedFolders.update((s) => {
						if (s.has(folder.id)) s.delete(folder.id);
						else s.add(folder.id);

						return s;
					});
					selectedImages = new Set();
				}}
				active={$selectedFolders.has(folder.id)}
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
	{#each $images as image (image.id)}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-1">
			<ImageCard
				onChange={(checked) => {
					if (checked) selectedImages = selectedImages.add(image.id);
					else selectedImages.delete(image.id);

					selectedImages = selectedImages;
					selectedFolders.set(new Set());
				}}
				onView={() => {
					showImageDetails = true;
				}}
				forceHover={imagesSelected}
				checked={selectedImages.has(image.id)}
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
		tags={[]}
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
