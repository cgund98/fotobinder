<script lang="ts">
	import PathHeader from '../../../lib/components/library/header/PathHeader.svelte';
	import FolderCard from '../../../lib/components/library/folder/FolderCard.svelte';
	import Separator from '../../../lib/components/decoration/Separator.svelte';
	import ImageCard from '../../../lib/components/library/image/ImageCard.svelte';
	import EditTagModal from '../../../lib/components/tags/EditTagModal.svelte';

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
</script>

<PathHeader path={['Local Files', 'Downloads', 'Mountains', 'Peaks']} />

<Separator className="mt-2" />

<div class="w-full flex flex-wrap mt-1">
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
			<ImageCard --src="url('{image.src}')" --color="red" />
		</div>
	{/each}
</div>

<EditTagModal />
