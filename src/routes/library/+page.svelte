<script lang="ts">
	import Button, { Variant } from '../../lib/components/button/Button.svelte';
	import Separator from '../../lib/components/decoration/Separator.svelte';
	import ChevronDown from '../../lib/components/icons/ChevronDown.svelte';
	import FolderCard from '../../lib/components/library/folder/FolderCard.svelte';
	import PathHeader from '../../lib/components/library/header/PathHeader.svelte';
	import NewSourceModal from '../../lib/components/library/source/NewSourceModal.svelte';

	let showNewSource = false;

	const folders = [{ name: 'Folder' }, { name: 'Folder5' }, { name: 'Folder6' }];
</script>

<PathHeader path={['My Library']} />

<div class="flex justify-between py-1 px-2 mt-2">
	<div class="flex flex-col justify-end">
		<p class="text-gray-500 text-base">Select or add an image source.</p>
	</div>

	<div class="flex flex-row space-x-3">
		<Button title="Actions" variant={Variant.Secondary} disabled>
			<ChevronDown className="w-[16px] mt-[1px]" />
		</Button>
	</div>
</div>

<Separator className="my-2" />

<div class="w-full flex flex-wrap">
	<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
		<FolderCard name="Add Source" add />
	</div>
	{#each folders as folder}
		<div class="w-1/2 sm:w-1/3 md:w-1/4 xl:w-1/5 2xl:w-1/6 p-2">
			<FolderCard name={folder.name} />
		</div>
	{/each}
</div>

<a href="/library/local">Local</a>

{#if showNewSource}
	<NewSourceModal
		onClose={() => {
			showNewSource = false;
		}}
	/>
{/if}
