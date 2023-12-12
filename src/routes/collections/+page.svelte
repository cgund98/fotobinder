<script lang="ts">
	import { listByParentId } from '$lib/api/collection';
	import Button, { Variant } from '$lib/components/button/Button.svelte';
	import NewCollectionModal from '$lib/components/collections/NewCollectionModal.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import ChevronDown from '$lib/components/icons/ChevronDown.svelte';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';
	import FolderCard from '$lib/components/library/folder/FolderCard.svelte';
	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import { routeToPage } from '$lib/nav/route';
	import { catchBad } from '$lib/store/alerts';

	let folders: { id: string; name: string }[] = [];

	const fetchCollections = () => {
		const doFn = async () => {
			const res = await listByParentId(null);
			folders = res.collections.map((col) => ({ id: col.id, name: col.name }));
		};

		doFn().catch(catchBad);
	};

	fetchCollections();

	let showNewCollection = false;
</script>

<PageTransitionWrapper>
	<PathHeader path={[{ label: 'My Collections', route: '/collections' }]} />

	<div class="flex justify-between pb-1 px-2">
		<div class="flex flex-col justify-end">
			<p class="text-gray-500 text-base">No Items Selected</p>
		</div>

		<div class="flex flex-row space-x-3">
			<Button title="Actions" variant={Variant.Secondary} disabled>
				<ChevronDown className="w-[16px] h-full" />
			</Button>
		</div>
	</div>

	<Separator className="my-2" />

	<div class="w-full flex flex-wrap">
		{#each folders as folder}
			<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
				<FolderCard
					name={folder.name}
					onDoubleClick={() => routeToPage(`/collections/${folder.id}`)}
				/>
			</div>
		{/each}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard name="New Collection" add onClick={() => (showNewCollection = true)} />
		</div>
	</div>
</PageTransitionWrapper>

{#if showNewCollection}
	<NewCollectionModal
		onClose={() => {
			showNewCollection = false;
			fetchCollections();
		}}
	/>
{/if}
