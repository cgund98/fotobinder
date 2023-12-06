<script lang="ts">
	import Entry from './Entry.svelte';
	import Separator from '../../decoration/Separator.svelte';

	import Search from '../../icons/Search.svelte';
	import Collection from '../../icons/FolderSolid.svelte';
	import Library from '../../icons/Library.svelte';
	import Tag from '$lib/components/icons/Tag.svelte';

	const sepClassNames = 'px-2';
</script>

<div class="mr-2 bg-bg-gray rounded-b">
	<div class="flex flex-col">
		<div class="mx-4 mb-1 mt-4">
			<img src="/logo-name.svg" class="w-40" alt="Logo Photobinder" />
		</div>

		<Entry title="Search" href="/search" icon={Search}></Entry>

		<Separator className={sepClassNames} />

		<Entry title="Library" href="/library" icon={Library} collapseable></Entry>

		<Separator className={sepClassNames} />

		<Entry title="Tags" href="/tags" icon={Tag}></Entry>

		<Separator className={sepClassNames} />

		<Entry title="Collections" href="/collections" icon={Collection} collapseable></Entry>
	</div>
</div>
