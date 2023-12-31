<script lang="ts">
	import path, { relative } from 'path-browserify';
	import { appDataDir, join } from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	import { writable } from 'svelte/store';

	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import FolderCard from '$lib/components/library/folder/FolderCard.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import ImageCard from '$lib/components/library/image/ImageCard.svelte';
	import EditTagsModal from '$lib/components/tags/EditTagsModal.svelte';
	import Button from '$lib/components/button/Button.svelte';
	import Tag from '$lib/components/icons/Tag.svelte';
	import Photo from '$lib/components/icons/Photo.svelte';
	import ImageDetailsCard from '$lib/components/library/image/ImageDetailsCard.svelte';
	import AddToCollectionModal from '$lib/components/collections/AddToCollectionModal.svelte';
	import type { HeaderEntry } from '$lib/components/library/header/PageHeader';

	import type { PageData } from './$types';
	import { catchBad, good } from '$lib/store/alerts';
	import Menu from '$lib/components/menu/Menu.svelte';
	import { routeToPage } from '$lib/nav/route';
	import { get, listByParentId, remove } from '$lib/api/collection';
	import { remove as removeCollectionImage } from '$lib/api/collection_image';
	import { FileType, listbyCollectionId } from '$lib/api/fs_entry';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';
	import ConfirmModal from '$lib/components/layout/ConfirmModal.svelte';
	import Plus from '$lib/components/icons/Plus.svelte';
	import Trash from '$lib/components/icons/Trash.svelte';
	import ProgressWrapper from '$lib/components/progress/ProgressWrapper.svelte';
	import Pencil from '$lib/components/icons/Pencil.svelte';
	import EditCollectionModal from '$lib/components/collections/EditCollectionModal.svelte';
	import PaginationControls from '$lib/components/nav/pagination/PaginationControls.svelte';

	let loading = false;

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

	// Image details
	let refreshImageDetails: (relativePath: string, sourceId: string) => Promise<void>;
	let selSourceId = '';
	let selRelativePath = '';
	let selIdx = 0;

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

	const fetchEntries = async (collectionId: string) => {
		loading = true;
		try {
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
							id: `${e.source_id}<|>${e.relative_path}`,
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
		} catch (err) {
			catchBad(err);
		}
		loading = false;
	};

	// Initialize fields
	fetchEntries(data.collectionId);
	$: imagesSelected = selectedImages.size > 0;
	$: foldersSelected = $selectedFolders.size > 0;
	$: noSelection = !imagesSelected && !foldersSelected;

	let showEditCollection = false;

	// Menu options
	$: menuOptions = [
		{
			label: 'Edit Collection',
			icon: Pencil,
			action: () => (showEditCollection = true),
			disabled: $selectedFolders.size !== 1
		},
		{
			label: 'Add to Collection',
			icon: Plus,
			action: () => (showAddToCollection = true),
			disabled: selectedImages.size === 0
		},
		{
			label: 'Remove Images',
			icon: Photo,
			action: () => {
				confirmTitle = 'Confirm Delete';
				confirmMessage = `Are you sure you want to remove ${selectedImages.size} images from this collection?`;
				onConfirmAccept = async () => {
					try {
						for (let id of selectedImages) {
							const [sourceId, relativePath] = id.split('<|>');
							await removeCollectionImage(data.collectionId, relativePath, sourceId);
						}

						good(`Removed ${selectedImages.size} images from this collection.`);

						selectedImages = new Set();
						fetchEntries(data.collectionId);
						showConfirm = false;
					} catch (err) {
						catchBad(err);
					}
				};
				showConfirm = true;
			},
			disabled: selectedImages.size === 0
		},
		{
			label: 'Delete Collections',
			icon: Trash,
			action: () => {
				confirmTitle = 'Confirm Delete';
				confirmMessage = `Are you sure you want to delete ${$selectedFolders.size} subcollections?`;
				onConfirmAccept = async () => {
					try {
						for (let collectionId of $selectedFolders) {
							await remove(collectionId);
						}

						good(`Deleted ${$selectedFolders.size} subcollection.`);

						selectedFolders.set(new Set());
						fetchEntries(data.collectionId);
						showConfirm = false;
					} catch (err) {
						catchBad(err);
					}
				};
				showConfirm = true;
			},
			disabled: !foldersSelected
		}
	];

	// Modals
	let showEditTags = false;
	let showImageDetails = false;
	let showAddToCollection = false;

	$: relativePaths = $images.filter((i) => selectedImages.has(i.id)).map((i) => i.relativePath);
	$: sourceIds = $images.filter((i) => selectedImages.has(i.id)).map((i) => i.sourceId);

	// Confirm modal
	let showConfirm = false;
	let confirmTitle = '';
	let confirmMessage = '';
	let onConfirmAccept = () => {};
	const onConfirmReject: () => void = () => (showConfirm = false);

	let headerHeight: number;
	let headerWidth: number;

	// Pagination
	const PAGE_SIZE = 150;
	let curPage: number = 0;
	$: pageCount = Math.ceil($images.length / PAGE_SIZE);
</script>

{#if loading}
	<ProgressWrapper />
{/if}

<PageTransitionWrapper>
	<div
		class="fixed w-full top-0 pt-4 bg-bg-gray z-10"
		style:width="{headerWidth}px"
		bind:clientHeight={headerHeight}
	>
		<PathHeader bind:path={$navEntries} />

		<div class="flex justify-between pb-1 px-2">
			<div class="flex flex-col justify-end">
				<p class="text-gray-500 text-base">
					{#if noSelection}
						No items selected. <button
							class="text-teal-400 font-medium ml-2"
							on:click={() => (selectedImages = new Set($images.map((i) => i.id)))}
							>Select all</button
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
	</div>

	<div style="padding-top: {headerHeight}px" class="w-full -mt-4" bind:clientWidth={headerWidth} />

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
						routeToPage(`/collections/${folder.id}`);
					}}
				/>
			</div>
		{/each}
	</div>

	{#if $images.length && $folders.length}
		<Separator className="my-2" />
	{/if}

	{#if pageCount > 1}
		<PaginationControls bind:curPage {pageCount} pageSize={PAGE_SIZE} maxItems={$images.length} />
	{/if}

	<div class="w-full flex flex-wrap mt-1">
		{#each $images.slice(curPage * PAGE_SIZE, (curPage + 1) * PAGE_SIZE) as image, idx (image.id)}
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

	{#if pageCount > 1}
		<Separator className="my-2" />
		<PaginationControls bind:curPage {pageCount} pageSize={PAGE_SIZE} maxItems={$images.length} />
		<div class="py-2" />
	{/if}
</PageTransitionWrapper>

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

{#if showConfirm}
	<ConfirmModal
		title={confirmTitle}
		message={confirmMessage}
		onClose={onConfirmReject}
		onConfirm={onConfirmAccept}
	/>
{/if}

{#if showEditCollection}
	<EditCollectionModal
		collectionId={[...$selectedFolders][0] || ''}
		onClose={() => {
			showEditCollection = false;
			fetchEntries(data.collectionId);
		}}
	/>
{/if}
