<script lang="ts">
	import { listByParentId, remove } from '$lib/api/collection';
	import Button, { Variant } from '$lib/components/button/Button.svelte';
	import NewCollectionModal from '$lib/components/collections/NewCollectionModal.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import ChevronDown from '$lib/components/icons/ChevronDown.svelte';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';
	import FolderCard from '$lib/components/library/folder/FolderCard.svelte';
	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import Trash from '$lib/components/icons/Trash.svelte';
	import { routeToPage } from '$lib/nav/route';
	import { catchBad, good } from '$lib/store/alerts';
	import ConfirmModal from '$lib/components/layout/ConfirmModal.svelte';
	import Menu from '$lib/components/menu/Menu.svelte';
	import { collections } from '$lib/store/nav';

	let folders: { id: string; name: string }[] = [];

	const fetchCollections = () => {
		const doFn = async () => {
			const res = await listByParentId(null);
			folders = res.collections.map((col) => ({ id: col.id, name: col.name }));
			collections.set(res.collections.sort((a, b) => a.name.localeCompare(b.name)));
		};

		doFn().catch(catchBad);
	};

	fetchCollections();

	let showNewCollection = false;

	let selectedCollection: string | undefined;

	// Menu options
	$: menuOptions = [
		{
			label: 'Delete Collections',
			icon: Trash,
			action: () => {
				const collectionName = folders.filter((f) => f.id === selectedCollection)[0].name;
				confirmTitle = 'Confirm Delete';
				confirmMessage = `Are you sure you want to delete '${collectionName}'?`;
				onConfirmAccept = async () => {
					if (!selectedCollection) return;

					try {
						remove(selectedCollection);

						good(`Deleted '${collectionName}'.`);

						selectedCollection = undefined;
						fetchCollections();
						showConfirm = false;
					} catch (err) {
						catchBad(err);
					}
				};
				showConfirm = true;
			},
			disabled: !selectedCollection
		}
	];

	// Confirm modal
	let showConfirm = false;
	let confirmTitle = '';
	let confirmMessage = '';
	let onConfirmAccept = () => {};
	const onConfirmReject: () => void = () => (showConfirm = false);
</script>

<PageTransitionWrapper>
	<PathHeader path={[{ label: 'My Collections', route: '/collections' }]} />

	<div class="flex justify-between pb-1 px-2">
		<div class="flex flex-col justify-end">
			<p class="text-gray-500 text-base">No Items Selected</p>
		</div>

		<div class="flex flex-row space-x-3">
			<Menu label="Actions" options={menuOptions} position="right" />
		</div>
	</div>

	<Separator className="my-2" />

	<div class="w-full flex flex-wrap">
		{#each folders as folder}
			<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
				<FolderCard
					name={folder.name}
					onClick={() => {
						if (selectedCollection === folder.id) selectedCollection = undefined;
						else selectedCollection = folder.id;
					}}
					onDoubleClick={() => routeToPage(`/collections/${folder.id}`)}
					active={selectedCollection === folder.id}
				/>
			</div>
		{/each}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard name="New Collection" add onClick={() => (showNewCollection = true)} />
		</div>
	</div>
</PageTransitionWrapper>

{#if showConfirm}
	<ConfirmModal
		title={confirmTitle}
		message={confirmMessage}
		onClose={onConfirmReject}
		onConfirm={onConfirmAccept}
	/>
{/if}

{#if showNewCollection}
	<NewCollectionModal
		onClose={() => {
			showNewCollection = false;
			fetchCollections();
		}}
	/>
{/if}
