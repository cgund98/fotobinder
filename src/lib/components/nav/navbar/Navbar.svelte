<script lang="ts">
	import Entry from './Entry.svelte';
	import Separator from '../../decoration/Separator.svelte';

	import Search from '../../icons/Search.svelte';
	import Collection from '../../icons/FolderSolid.svelte';
	import Library from '../../icons/Library.svelte';

	const iconClassNames = 'text-gray-500 h-[20px] w-[20px]';
	const sepClassNames = 'px-2';
</script>

<div class="mr-2 bg-bg-gray rounded-b">
	<div class="flex flex-col">
		<div class="mx-4 mb-1 mt-4">
			<img src="/logo-name.svg" class="w-40" alt="Logo Photobinder" />
		</div>

		<Entry title="Search" href="/search">
			<Search className={iconClassNames} />
		</Entry>

		<Separator className={sepClassNames} />

		<Entry title="Library" href="/library" collapseable>
			<Library className={iconClassNames + ' !text-teal-500'} />
		</Entry>

		<Separator className={sepClassNames} />

		<Entry title="Collections" href="/collections" collapseable>
			<Collection className={iconClassNames} />
		</Entry>
	</div>
</div>
