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
	import Menu from '$lib/components/menu/Menu.svelte';
	import FolderSolid from '$lib/components/icons/FolderSolid.svelte';
	import { routeToPage } from '$lib/nav/route';
	import { get, listByParentId } from '$lib/api/collection';
	import { FileType, listbyCollectionId } from '$lib/api/fs_entry';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';

	// Read inputs
	export let data: PageData;

	interface Folder {
		id: string;
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

	// Fetch entries
	const folders = writable<Folder[]>([]);
	const images = writable<Image[]>([]);
	const navEntries = writable<HeaderEntry[]>([]);

	const updateNav = async (collectionId: string) => {
		let entries: HeaderEntry[] = [];
		let pId: string | null = collectionId;
		do {
			let col = await get(pId);
			entries = [{ label: col.name, route: `/collections/${col.id}` }, ...entries];
			pId = col.parent_id;
		} while (pId !== null);

		navEntries.set([{ label: 'My Collections', route: '/collections' }, ...entries]);
	};

	const fetchEntries = (collectionId: string) => {
		const doFn = async () => {
			const subcollections = await listByParentId(collectionId);

			// Parse folders
			const newFolders = subcollections.collections.map((e) => {
				return { name: e.name, id: e.id };
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
			const res = await listbyCollectionId(collectionId);
			const newImages = await Promise.all(
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

			// Update values
			folders.set(newFolders);
			images.set(newImages);
			await updateNav(collectionId);
		};
		doFn().catch(catchBad);
	};

	// Initialize fields
	$: fetchEntries(data.collectionId);
	$: imagesSelected = selectedImages.size > 0;
	$: foldersSelected = $selectedFolders.size > 0;
	$: noSelection = !imagesSelected && !foldersSelected;

	// Menu options
	$: menuOptions = [
		{
			label: 'Add to Collection',
			icon: FolderSolid,
			action: () => (showAddToCollection = true),
			disabled: !imagesSelected
		}
	];

	// Modals
	let showEditTags = false;
	let showImageDetails = false;
	let showAddToCollection = false;

	$: relativePaths = $images.filter((i) => selectedImages.has(i.id)).map((i) => i.relativePath);
	$: sourceIds = $images.filter((i) => selectedImages.has(i.id)).map((i) => i.sourceId);
</script>

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
				disabled={!imagesSelected}
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
						console.log(`/collections/${folder.id}`);
						routeToPage(`/collections/${folder.id}`);
					}}
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
					name={image.name}
					--src="url('{image.src}')"
					--color="red"
				/>
			</div>
		{/each}
	</div>
</PageTransitionWrapper>

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
		relativePaths={$images.filter((i) => selectedImages.has(i.id)).map((i) => i.relativePath)}
		sourceIds={$images.filter((i) => selectedImages.has(i.id)).map((i) => i.sourceId)}
		onClose={() => {
			showAddToCollection = false;
		}}
	/>
{/if}

{#if showEditTags}
	<EditTagsModal
		{relativePaths}
		{sourceIds}
		isImage
		onClose={() => {
			showEditTags = false;
		}}
	/>
{/if}
