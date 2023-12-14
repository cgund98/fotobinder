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
	import { FileType, generateMissingThumbnails, listBySourceId } from '$lib/api/fs_entry';
	import Menu from '$lib/components/menu/Menu.svelte';
	import FolderSolid from '$lib/components/icons/FolderSolid.svelte';
	import Reset from '$lib/components/icons/Reset.svelte';
	import { routeToPage } from '$lib/nav/route';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';
	import ThumbnailQueueProgress from '$lib/components/progress/ThumbnailQueueProgress.svelte';
	import ProgressWrapper from '$lib/components/progress/ProgressWrapper.svelte';

	let loading = false;

	// Read inputs
	export let data: PageData;

	$: subpath = $page.url.searchParams.get('subpath') || '';

	interface Folder {
		id: string;
		relativePath: string;
		name: string;
	}
	interface Image {
		id: string;
		relativePath: string;
		sourceId: string;
		name: string;
		src: string;
	}

	// Selection
	let selectedImages = new Set<string>();
	let selectedFolders = writable<Set<string>>(new Set());

	let refreshImageDetails: (relativePath: string, sourceId: string) => Promise<void>;
	let selSourceId = '';
	let selRelativePath = '';
	let selIdx = 0;

	// Fetch entries
	const folders = writable<Folder[]>([]);
	const images = writable<Image[]>([]);
	const source = writable<Source>();
	const navEntries = writable<HeaderEntry[]>([]);

	const updateNav = () => {
		let parts = subpath.split(path.sep);

		let partsPaths = parts.map((_, idx) => parts.slice(0, idx + 1).join(path.sep));

		navEntries.set([
			{ label: 'My Library', route: '/library' },
			{ label: $source.name, route: `/library/${$source.id}`, afterRoute: () => fetchSource() },
			...partsPaths
				.filter((p) => p)
				.map((p, idx) => ({
					label: parts[idx],
					route: `/library/${$source.id}`,
					queryParams: { subpath: p },
					afterRoute: () => fetchSource()
				}))
		]);
	};

	// Fetch data
	const fetchSource = async () => {
		loading = true;
		try {
			const res = await getSource(data.sourceId);
			source.set(res);
			updateNav();
			fetchEntries(subpath);
		} catch (err) {
			catchBad(err);
		}
		loading = false;
	};

	const fetchEntries = async (subpath: string) => {
		loading = true;

		try {
			const res = await listBySourceId(data.sourceId, subpath);

			// Parse folderslistBySourceid
			const newFolders = res.entries
				.filter((e) => e.fs_type == FileType.Directory)
				.map((e) => {
					let parts = e.relative_path.split(path.sep);
					return {
						name: parts[parts.length - 1],
						id: e.relative_path,
						relativePath: e.relative_path
					};
				})
				.sort((a, b) => a.name.localeCompare(b.name));

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
			let newImages = await Promise.all(
				res.entries
					.filter((e) => e.fs_type == FileType.File)
					.map(async (e) => {
						const parts = e.relative_path.split(path.sep);
						return {
							src: await mapFileSrc(e.thumbnail_path),
							id: `${e.source_id}/${e.relative_path}`,
							name: parts[parts.length - 1],
							relativePath: e.relative_path,
							sourceId: e.source_id
						};
					})
			);
			newImages = newImages.sort((a, b) => a.relativePath.localeCompare(b.relativePath));

			// Filter selection
			let imageIds = newImages.reduce((s, i) => s.add(i.id), new Set<String>());
			selectedImages = new Set(...Object.keys(selectedImages).filter((i) => imageIds.has(i)));

			// Update values
			folders.set(newFolders);
			images.set(newImages);
			updateNav();
		} catch (err) {
			catchBad(err);
		}
		loading = false;
	};

	// Initialize fields
	fetchSource();
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
			action: async () => {
				loading = true;
				try {
					const res = await scan(data.sourceId);

					let msg = `Found ${res.entries_created} new entries and \
							deleted ${res.entries_deleted} outdated entries.`;

					if (res.thumbnails_created)
						msg += ` Generating ${res.thumbnails_created} \
							thumbnails in the background...`;
					good(msg);
					fetchEntries(subpath);
				} catch (err) {
					catchBad(err);
				}
				loading = false;
			},
			disabled: false
		},
		{
			label: 'Fix Thumbnails',
			icon: Photo,
			action: () => {
				generateMissingThumbnails(data.sourceId)
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
	let showImageDetails = false;
	let showAddToCollection = false;

	$: relativePaths = imagesSelected
		? $images.filter((i) => selectedImages.has(i.id)).map((i) => i.relativePath)
		: $folders.filter((i) => $selectedFolders.has(i.id)).map((i) => i.relativePath);
</script>

{#if loading}
	<ProgressWrapper />
{/if}

<PageTransitionWrapper>
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
					<button
						class="text-teal-400 font-medium ml-2"
						on:click={() => (selectedImages = new Set())}
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
					onDoubleClick={() => {
						routeToPage(`/library/${data.sourceId}`, { subpath: path.join(subpath, folder.name) });
						fetchSource();
					}}
				/>
			</div>
		{/each}
	</div>

	{#if $images.length && $folders.length}
		<Separator className="my-2" />
	{/if}

	<div class="w-full flex flex-wrap mt-1">
		{#each $images as image, idx (image.id)}
			<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-1">
				<ImageCard
					onChange={(checked) => {
						if (checked) selectedImages = selectedImages.add(image.id);
						else selectedImages.delete(image.id);

						selectedImages = selectedImages;
						selectedFolders.set(new Set());
					}}
					onView={() => {
						selIdx = idx;
						selSourceId = image.sourceId;
						selRelativePath = image.relativePath;
						showImageDetails = true;
					}}
					forceHover={imagesSelected}
					checked={selectedImages.has(image.id)}
					name={image.name}
					--src="url('{image.src}')"
					--color="red"
				/>
			</div>
		{/each}
	</div>
</PageTransitionWrapper>

<ThumbnailQueueProgress />

{#if showEditTags}
	<EditTagsModal
		{relativePaths}
		sourceIds={$images.filter((i) => selectedImages.has(i.id)).map((i) => i.sourceId)}
		isImage={imagesSelected}
		onClose={() => {
			showEditTags = false;
		}}
	/>
{/if}

{#if showImageDetails}
	<ImageDetailsCard
		onClose={() => {
			showImageDetails = false;
		}}
		relativePath={selRelativePath}
		sourceId={selSourceId}
		onPrev={() => {
			selIdx = selIdx - 1;
			if (selIdx < 0) selIdx = $images.length - 1;
			selRelativePath = $images[selIdx].relativePath;
			selSourceId = $images[selIdx].sourceId;
			refreshImageDetails(selRelativePath, selSourceId);
		}}
		onNext={() => {
			selIdx = selIdx + 1;
			if (selIdx === $images.length) selIdx = 0;
			selRelativePath = $images[selIdx].relativePath;
			selSourceId = $images[selIdx].sourceId;
			refreshImageDetails(selRelativePath, selSourceId);
		}}
		bind:loading
		bind:fetchDetails={refreshImageDetails}
	/>
{/if}

{#if showAddToCollection}
	<AddToCollectionModal
		relativePaths={$images.filter((i) => selectedImages.has(i.id)).map((i) => i.relativePath)}
		sourceIds={$images.filter((i) => selectedImages.has(i.id)).map((i) => i.sourceId)}
		onClose={() => {
			showAddToCollection = false;
		}}
	/>
{/if}
