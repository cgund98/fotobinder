<script lang="ts">
	import Entry from './Entry.svelte';
	import Separator from '../../decoration/Separator.svelte';

	import Search from '../../icons/Search.svelte';
	import FolderSolid from '../../icons/FolderSolid.svelte';
	import Library from '../../icons/Library.svelte';
	import Tag from '$lib/components/icons/Tag.svelte';
	import { listByParentId } from '$lib/api/collection';
	import { catchBad } from '$lib/store/alerts';
	import { list } from '$lib/api/source';
	import EntryBlock from './EntryBlock.svelte';
	import Subentry from './Subentry.svelte';
	import { collections, sources } from '$lib/store/nav';

	const sepClassNames = 'px-2';

	let showSources = false;
	let showCollections = false;

	const fetchCollections = async () => {
		try {
			const res = await listByParentId(null);
			collections.set(res.collections.sort((a, b) => a.name.localeCompare(b.name)));
		} catch (err) {
			catchBad(err);
		}
	};

	const fetchSources = async () => {
		try {
			const res = await list();
			sources.set(res.sources.sort((a, b) => a.name.localeCompare(b.name)));
		} catch (err) {
			catchBad(err);
		}
	};

	fetchCollections();
	fetchSources();
</script>

<div class="mr-2 bg-bg-gray rounded-b">
	<div class="flex flex-col">
		<div class="mx-4 mb-1 mt-4">
			<img src="/logo-name.svg" class="w-40" alt="Logo Photobinder" />
		</div>

		<Entry title="Search" href="/search" icon={Search}></Entry>

		<Separator className={sepClassNames} />

		<Entry
			title="Library"
			href="/library"
			icon={Library}
			collapseable={$sources.length > 0}
			bind:showChildren={showSources}
		>
			{#if showSources && $sources.length > 0}
				<EntryBlock>
					{#each $sources as source}
						<Subentry label={source.name} href={`/library/${source.id}`} />
					{/each}
				</EntryBlock>
			{/if}
		</Entry>

		<Separator className={sepClassNames} />

		<Entry title="Tags" href="/tags" icon={Tag}></Entry>

		<Separator className={sepClassNames} />

		<Entry
			title="Collections"
			href="/collections"
			icon={FolderSolid}
			collapseable={$collections.length > 0}
			bind:showChildren={showCollections}
		>
			{#if showCollections && $collections.length > 0}
				<EntryBlock>
					{#each $collections as collection}
						<Subentry label={collection.name} href={`/collections/${collection.id}`} />
					{/each}
				</EntryBlock>
			{/if}</Entry
		>
	</div>
</div>
