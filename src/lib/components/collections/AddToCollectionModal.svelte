<script lang="ts">
	import Modal from '../layout/Modal.svelte';
	import Separator from '../decoration/Separator.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import FolderSolid from '../icons/FolderSolid.svelte';
	import Trash from '../icons/Trash.svelte';
	import Plus from '../icons/Plus.svelte';
	import Close from '../icons/Close.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';
	import SearchBox from '../input/SearchBox.svelte';
	import { list, type Collection } from '$lib/api/collection';
	import { catchBad, good } from '$lib/store/alerts';
	import { assignMany } from '$lib/api/collection_image';
	import NewCollectionModal from './NewCollectionModal.svelte';

	export let onClose: () => void = () => {};

	export let relativePaths: string[];
	export let sourceIds: string[];

	// Fetch collections
	let collectionIdOptions: { label: string; value?: string }[] = [];

	const fetchCollections = () => {
		const doFn = async () => {
			const res = await list();

			let map = res.collections.reduce(
				(m, col) => ({ ...m, [col.id]: col }),
				{} as { [id: string]: Collection }
			);

			const assignNames = (input: Collection[]) =>
				input.map((col) => {
					let name = col.name;
					let parent_id = col.parent_id;

					// Append parent name to name
					if (col.parent_id && map[col.parent_id]) {
						name = `${map[col.parent_id].name} | ${name}`;
						parent_id = map[col.parent_id].parent_id;
					}

					return { id: col.id, parent_id, name };
				});

			let filteredCollections = res.collections;
			while (true) {
				filteredCollections = assignNames(filteredCollections);

				// Count number of substitutions remaining
				const subCount = filteredCollections.reduce(
					(count, col) => count + (col.parent_id === null ? 0 : 1),
					0
				);
				console.log('count', subCount);
				if (subCount === 0) break;
			}
			filteredCollections = filteredCollections.filter((col) => col.parent_id === null);

			collectionIdOptions = filteredCollections.map((col) => ({ value: col.id, label: col.name }));
			collectionIdOptions = [...collectionIdOptions];
		};

		doFn().catch(catchBad);
	};

	fetchCollections();

	// Form values
	let collectionId: string | undefined = undefined;

	$: collectionName = collectionId
		? collectionIdOptions.filter((op) => op.value === collectionId)[0].label
		: '';

	// Modal
	let showNewCollection = false;
</script>

<Modal>
	<div class="h-full">
		<div class="flex flex-row justify-between items-center">
			<h1 class="text-lg font-bold pb-1">Save to Collection</h1>
			<div role="button" class="-mt-1">
				<IconButton
					onClick={onClose}
					icon={Close}
					variant={IconVariant.Embedded}
					label="Close Window"
				/>
			</div>
		</div>
		<div class="w-full h-2">
			<Separator className="bg-gray-700" />
		</div>
		<div class="w-full flex flex-row justify-between mt-2 items-center space-x-4 py-2">
			<SearchBox
				bind:value={collectionId}
				options={collectionIdOptions}
				placeholder="Search"
				className="flex-grow"
			/>
			<Button
				variant={Variant.Secondary}
				onClick={() => (showNewCollection = true)}
				title="Create Collection"
			>
				<Plus className="w-[15px] h-full" />
			</Button>
		</div>
		<div class="w-full flex flex-row justify-between content-bottom mt-2">
			<div class="flex flex-col justify-between">
				<p class="text-gray-500 inline-block my-auto text-base">
					Saving {relativePaths.length} images to collection
				</p>
			</div>
			<div class="flex space-x-4">
				<Button variant={Variant.Warn} title="Discard" onClick={onClose}>
					<Trash className="w-[16px] h-full" />
				</Button>
				<Button
					variant={Variant.Primary}
					title="Save"
					disabled={collectionId === undefined}
					onClick={() => {
						assignMany(collectionId || '', relativePaths, sourceIds)
							.then(() => {
								good(`Saved ${relativePaths.length} images to collection '${collectionName}'.`);
								onClose();
							})
							.catch(catchBad);
					}}
				>
					<FolderSolid className="w-[16px] h-full" />
				</Button>
			</div>
		</div>
	</div>
</Modal>

{#if showNewCollection}
	<NewCollectionModal
		onClose={() => {
			showNewCollection = false;
			fetchCollections();
		}}
	/>
{/if}
