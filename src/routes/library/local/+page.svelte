<script lang="ts">
	import PathHeader from '../../../lib/components/library/header/PathHeader.svelte';
	import FolderCard from '../../../lib/components/library/folder/FolderCard.svelte';
	import Separator from '../../../lib/components/decoration/Separator.svelte';
	import ImageCard from '../../../lib/components/library/image/ImageCard.svelte';
	import EditTagsModal from '../../../lib/components/tags/EditTagsModal.svelte';
	import NewTagModal from '../../../lib/components/tags/NewTagModal.svelte';
	import Button, { Variant } from '../../../lib/components/button/Button.svelte';
	import Tag from '../../../lib/components/icons/Tag.svelte';
	import ChevronDown from '../../../lib/components/icons/ChevronDown.svelte';
	import ImageDetailsCard from '../../../lib/components/library/image/ImageDetailsCard.svelte';
	import AddToCollectionModal from '../../../lib/components/collections/AddToCollectionModal.svelte';

	const folders = [
		{ name: 'Folder' },
		{ name: 'Folder1 with an extra long name' },
		{ name: 'Folder2' },
		{ name: 'Folder3' },
		{ name: 'Folder4' },
		{ name: 'Folder5' },
		{ name: 'Folder6' }
	];

	const images = [
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' },
		{ src: '/image/mountain.jpg' }
	];

	let showEditTags = false;
	let showNewTag = false;
	let showImageDetails = false;
	let showAddToCollection = false;
</script>

<PathHeader path={['Local Files', 'Downloads', 'Mountains', 'Peaks']} />

<div class="flex justify-between py-1 px-2 mt-2">
	<div class="flex flex-col justify-end">
		<p class="text-gray-500 text-base">8 Items Selected</p>
	</div>

	<div class="flex flex-row space-x-3">
		<Button
			title="Modify Tags"
			variant={Variant.Primary}
			className="disabled:bg-teal-700"
			onClick={() => {
				showEditTags = true;
				console.log(showEditTags);
			}}
		>
			<Tag className="w-[15px] -mt-[1px]" />
		</Button>
		<Button
			title="Actions"
			onClick={() => {
				showAddToCollection = true;
			}}
			variant={Variant.Secondary}
		>
			<ChevronDown className="w-[16px] mt-[1px]" />
		</Button>
	</div>
</div>

<Separator className="my-2" />

<div class="w-full flex flex-wrap">
	{#each folders as folder}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard name={folder.name} />
		</div>
	{/each}
</div>

<Separator className="my-2" />

<div class="w-full flex flex-wrap mt-1">
	{#each images as image}
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
