<script lang="ts">
	import { writable } from 'svelte/store';

	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import { scan, get as getSource, type Source } from '$lib/api/source';
	import { catchBad } from '$lib/store/alerts';
	import Button from '$lib/components/button/Button.svelte';
	import FolderSolid from '$lib/components/icons/FolderSolid.svelte';
	import Menu from '$lib/components/menu/Menu.svelte';
	import EditTagsModal from '$lib/components/tags/EditTagsModal.svelte';
	import ImageDetailsCard from '$lib/components/library/image/ImageDetailsCard.svelte';
	import AddToCollectionModal from '$lib/components/collections/AddToCollectionModal.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import { load } from '$lib/store/search';
	import { listByTags } from '$lib/api/fs_entry';
	import path from 'path-browserify';
	import ImageCard from '$lib/components/library/image/ImageCard.svelte';
	import { appDataDir, join } from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';

	interface Image {
		id: string;
		relativePath: string;
		sourceId: string;
		name: string;
		src: string;
	}

	// Selection
	let selectedImages = new Set<string>();

	// Fetch entries
	const images = writable<Image[]>([]);

	$: imagesSelected = selectedImages.size > 0;

	// Modals
	let showImageDetails = false;
	let showAddToCollection = false;

	// Menu options
	$: menuOptions = [
		{
			label: 'Add to Collection',
			icon: FolderSolid,
			action: () => (showAddToCollection = true),
			disabled: !imagesSelected
		}
	];

	// Fetch data
	const fetchResults = () => {
		const doFn = async () => {
			const rules = load();

			// Parse included and excluded tags
			const includes = rules
				.filter((rule) => rule.ruleType === 'include' && rule.tagId !== undefined)
				.map((rule) => rule.tagId || '');
			const excludes = rules
				.filter((rule) => rule.ruleType === 'exclude' && rule.tagId !== undefined)
				.map((rule) => rule.tagId || '');

			// Fetch tags
			const res = await listByTags(includes, excludes);

			console.log(includes, excludes);

			// Map thumbnail paths to asset url
			const dataDir = await appDataDir();
			const mapFileSrc = async (p: string): Promise<string> => {
				return convertFileSrc(await join(dataDir, 'thumbnails', p));
			};

			// Parse images
			const newImages = await Promise.all(
				res.entries.map(async (e) => {
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
			images.set(newImages);
		};

		doFn().catch(catchBad);
	};

	fetchResults();

	$: relativePaths = $images.filter((i) => selectedImages.has(i.id)).map((i) => i.relativePath);
	$: sourceIds = $images.filter((i) => selectedImages.has(i.id)).map((i) => i.sourceId);
</script>

<PageTransitionWrapper>
	<PathHeader
		path={[
			{ label: 'Query Builder', route: '/search' },
			{ label: 'Results', route: '/search/results' }
		]}
	/>

	<div class="flex justify-between pb-1 px-2">
		<div class="flex flex-col justify-end">
			<p class="text-gray-500 text-base">
				{#if !imagesSelected}
					{$images.length} images found.
					<button
						class="text-teal-400 font-medium ml-2"
						on:click={() => (selectedImages = new Set($images.map((i) => i.id)))}>Select all</button
					>
				{:else}
					{selectedImages.size}/{$images.length} images selected.
					<button
						class="text-teal-400 font-medium ml-2"
						on:click={() => (selectedImages = new Set())}
						>Deselect all
					</button>
				{/if}
			</p>
		</div>

		<div class="flex flex-row space-x-3 items-center">
			<Menu label="Actions" options={menuOptions} position="right" />
		</div>
	</div>

	<Separator className="my-2" />

	<div class="w-full flex flex-wrap mt-1">
		{#each $images as image (image.id)}
			<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-1">
				<ImageCard
					onChange={(checked) => {
						if (checked) selectedImages = selectedImages.add(image.id);
						else selectedImages.delete(image.id);

						selectedImages = selectedImages;
					}}
					onView={() => {
						showImageDetails = true;
					}}
					forceHover={imagesSelected}
					checked={selectedImages.has(image.id)}
					name={image.name}
					--src="url('{image.src}')"
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
		{relativePaths}
		{sourceIds}
		onClose={() => {
			showAddToCollection = false;
		}}
	/>
{/if}
