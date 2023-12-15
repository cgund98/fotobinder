<script lang="ts">
	import { writable } from 'svelte/store';

	import Button, { Variant } from '$lib/components/button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '$lib/components/button/IconButton.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import Trash from '$lib/components/icons/Trash.svelte';
	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import ChevronDown from '$lib/components/icons/ChevronDown.svelte';
	import Pencil from '$lib/components/icons/Pencil.svelte';

	import { list, remove, type Tag } from '$lib/api/tag';
	import { catchBad, good } from '$lib/store/alerts';
	import Plus from '$lib/components/icons/Plus.svelte';
	import NewTagModal from '$lib/components/tags/NewTagModal.svelte';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';
	import EditTagModal from '$lib/components/tags/EditTagModal.svelte';
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { save as saveSearch } from '$lib/store/search';
	import Search from '$lib/components/icons/Search.svelte';
	import { routeToPage } from '$lib/nav/route';

	let tags = writable<Tag[]>([]);

	interface TagsBlock {
		[key: string]: {
			tag: Tag;
			children: Tag[];
			collapsed: boolean;
		};
	}
	$: tagsBlock = $tags
		.sort((a, b) => {
			if (a.parent_id === null && b.parent_id !== null) return -1;
			else if (a.parent_id !== null && b.parent_id === null) return 1;
			return 0;
		})
		.reduce((cur, tag) => {
			if (tag.parent_id === null) cur[tag.id] = { tag, children: [], collapsed: false };
			else if (cur[tag.parent_id]) cur[tag.parent_id].children.push(tag);

			return cur;
		}, {} as TagsBlock);

	const refreshTags = () => {
		list()
			.then((res) => tags.set(res.tags))
			.catch(catchBad);
	};

	refreshTags();

	let showNewTag = false;
	let showEditTag = false;

	let selTagId = '';
</script>

<PageTransitionWrapper>
	<PathHeader path={[{ label: 'Tags', route: '/tags' }]} />

	<div class="flex justify-between pb-1 px-2">
		<div class="flex flex-col justify-end">
			<p class="text-gray-500 text-base">Manage your library's tags.</p>
		</div>

		<div class="flex flex-row space-x-3 items-center">
			<Button title="Add Tag" variant={Variant.Secondary} onClick={() => (showNewTag = true)}>
				<Plus className="w-[15px] -mt-[1px]" />
			</Button>
		</div>
	</div>

	<Separator className="my-2" />

	<div class="px-2">
		{#each Object.values(tagsBlock) as { tag, children, collapsed }, i (tag.id)}
			<div class="hover:bg-gray-800 rounded flex flex-row align-center w-full justify-between">
				<div class="flex-grow flex flex-row">
					<div class="text-left w-8 flex flex-col justify-around">
						{#if children.length > 0}
							<IconButton
								icon={ChevronDown}
								variant={IconVariant.Embedded}
								label="Collapse Children"
								className="duration-200 ease-out {collapsed ? 'rotate-180' : ''}"
								onClick={() => {
									tagsBlock = {
										...tagsBlock,
										[tag.id]: { tag, children, collapsed: !collapsed }
									};
								}}
							/>
						{/if}
					</div>
					<div class="py-1.5">
						{tag.name}
					</div>
				</div>
				<div class="flex flex-row justify-end space-x-2">
					<div class="flex flex-col justify-around">
						<IconButton
							icon={Search}
							variant={IconVariant.Embedded}
							onClick={() => {
								const rules = [{ id: tag.id, ruleType: 'include', tagId: tag.id }];
								saveSearch(rules);
								routeToPage('/search/results');
							}}
							label="View Labeled Images"
						/>
					</div>
					<div class="flex flex-col justify-around">
						<IconButton
							icon={Pencil}
							variant={IconVariant.Embedded}
							onClick={() => {
								selTagId = tag.id;
								showEditTag = true;
							}}
							label="Edit Tag"
						/>
					</div>
					<div class="flex flex-col justify-around">
						<IconButton
							icon={Trash}
							onClick={() =>
								remove(tag.id)
									.then(() => {
										refreshTags();
										good(`Deleted tag '${tag.name}'`);
									})
									.catch(catchBad)}
							variant={IconVariant.Embedded}
							label="Delete Tag"
						/>
					</div>
				</div>
			</div>
			{#if children.length > 0 && !collapsed}
				<div transition:slide={{ duration: 200, easing: quintOut }}>
					<Separator className="my-1 opacity-50" />
					<div class="pl-[13px]">
						<div class="border-l-2 border-teal-800 pl-2">
							{#each children as child, idx}
								<div
									class="hover:bg-gray-800 rounded pl-6 flex flex-row align-center w-full justify-between"
								>
									<div class="flex-grow flex flex-row">
										<div class="py-1.5">
											{child.name}
										</div>
									</div>
									<div class="flex flex-row justify-end space-x-2">
										<div class="flex flex-col justify-around">
											<IconButton
												icon={Search}
												variant={IconVariant.Embedded}
												onClick={() => {
													const rules = [{ id: child.id, ruleType: 'include', tagId: child.id }];
													saveSearch(rules);
													routeToPage('/search/results');
												}}
												label="View Labeled Images"
											/>
										</div>
										<div class="flex flex-col justify-around">
											<IconButton
												icon={Pencil}
												variant={IconVariant.Embedded}
												onClick={() => {
													selTagId = child.id;
													showEditTag = true;
												}}
												label="Edit Tag"
											/>
										</div>
										<div class="flex flex-col justify-around">
											<IconButton
												icon={Trash}
												onClick={() =>
													remove(child.id)
														.then(() => {
															refreshTags();
															good(`Deleted tag '${child.name}'`);
														})
														.catch(catchBad)}
												variant={IconVariant.Embedded}
												label="Delete Tag"
											/>
										</div>
									</div>
								</div>
								{#if idx !== children.length - 1}
									<Separator className="my-1 opacity-50" />
								{/if}
							{/each}
						</div>
					</div>
				</div>
			{/if}

			{#if i !== Object.keys(tagsBlock).length - 1}
				<Separator className="my-1 opacity-50" />
			{/if}
		{/each}
	</div>

	{#if $tags.length === 0}
		<p class="text-left px-2">You have not created any tags yet. Try creating some!</p>
	{/if}
</PageTransitionWrapper>

{#if showNewTag}
	<NewTagModal
		onClose={() => {
			showNewTag = false;
			refreshTags();
		}}
	/>
{/if}

{#if showEditTag}
	<EditTagModal
		tagId={selTagId}
		onClose={() => {
			showEditTag = false;
			refreshTags();
		}}
	/>
{/if}
