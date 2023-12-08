<script lang="ts">
	import Modal from '../layout/Modal.svelte';
	import Separator from '../decoration/Separator.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import TagIcon from '../icons/Tag.svelte';
	import Trash from '../icons/Trash.svelte';
	import Plus from '../icons/Plus.svelte';
	import Checkbox from '../input/Checkbox.svelte';
	import TagBlock from './TagBlock.svelte';
	import Close from '../icons/Close.svelte';
	import SearchInput from '../input/SearchInput.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';
	import { listByBasePath, assign as assignPathTags } from '$lib/api/path_tag';
	import { catchBad, good } from '$lib/store/alerts';
	import {
		assign as assignImageTags,
		listByRelativePath,
		type TagAssignments
	} from '$lib/api/image_tag';
	import { writable } from 'svelte/store';
	import { list, type Tag } from '$lib/api/tag';
	import NewTagModal from './NewTagModal.svelte';
	import Warn from '../icons/Warn.svelte';

	export let onClose: () => void = () => {};

	export let sourceId: string;
	export let selectedImages: Set<string> = new Set();
	export let selectedFolders: Set<string> = new Set();
	let showNewTag = false;

	let tags = writable<TagsState>({});
	let tagsBlock = writable<TagsBlock>({});
	let filteredTagsBlock = writable<TagsBlock>({});

	interface TagsBlock {
		[key: string]: {
			tag: Tag;
			children: Tag[];
			collapsed: boolean;
		};
	}

	interface TagsState {
		[id: string]: {
			name: string;
			checked: boolean;
			partial: boolean;
		};
	}

	// Search functionality
	let search = '';

	const filterSearch = (search: string) => {
		if (!search) filteredTagsBlock.set({ ...$tagsBlock });
		const filteredIds = new Set(
			Object.entries($tags)
				.filter(([_, tag]) => tag.name.toLowerCase().includes(search.toLowerCase()))
				.map(([id, _]) => id)
		);

		let filteredBlock = Object.entries($tagsBlock).reduce((block, [id, entry]) => {
			if (filteredIds.has(id)) {
				block[id] = entry;
			} else {
				let filteredChildren = entry.children.filter((child) => filteredIds.has(child.id));
				if (filteredChildren.length > 0) {
					let e = { ...entry };
					e.children = filteredChildren;
					block[id] = e;
				}
			}

			return block;
		}, {} as TagsBlock);

		filteredTagsBlock.set(filteredBlock);
	};

	$: filterSearch(search);

	$: console.log($filteredTagsBlock);
	$: console.log($tagsBlock);

	// 🤨🤨 - I know this is kind of spaghetti but it works
	const refreshTags = () => {
		const doFn = async () => {
			const res = await list();

			let state = res.tags.reduce((b, t) => {
				b[t.id] = { checked: false, partial: false, name: t.name };
				return b;
			}, {} as TagsState);

			// Fetch current tags for all selected elements
			let tagFreqs: { [key: string]: number } = {};
			if (selectedImages.size > 0) {
				for (var relPath of selectedImages) {
					const { image_tags } = await listByRelativePath(relPath, sourceId);
					image_tags.forEach(
						(t) => (tagFreqs[t.tag_id] = tagFreqs[t.tag_id] ? tagFreqs[t.tag_id] + 1 : 1)
					);
				}
			} else {
				for (var basePath of selectedFolders) {
					const { path_tags } = await listByBasePath(basePath, sourceId);
					path_tags.forEach(
						(t) => (tagFreqs[t.tag_id] = tagFreqs[t.tag_id] ? tagFreqs[t.tag_id] + 1 : 1)
					);
				}
			}
			const selSize = selectedImages.size ? selectedImages.size : selectedFolders.size;
			Object.entries(tagFreqs).forEach(([id, count]) => {
				state[id] = { checked: true, partial: count != selSize, name: state[id]?.name };
			});
			tags.set(state);

			// Set tagsBlock
			tagsBlock.set(
				res.tags
					.sort((a, b) => {
						if (a.parent_id === null && b.parent_id !== null) return -1;
						else if (a.parent_id !== null && b.parent_id === null) return 1;
						return 0;
					})
					.reduce((cur, tag) => {
						if (tag.parent_id === null) cur[tag.id] = { tag, children: [], collapsed: false };
						else if (cur[tag.parent_id]) cur[tag.parent_id].children.push(tag);

						return cur;
					}, {} as TagsBlock)
			);

			filteredTagsBlock.set({ ...$tagsBlock });
		};

		doFn().catch(catchBad);
	};

	const submit = () => {
		const doFn = async () => {
			let added = 0;
			let removed = 0;
			if (selectedImages.size > 0) {
				const assignments: TagAssignments = {
					add: Object.keys($tags).filter((t) => $tags[t] && $tags[t].checked && !$tags[t].partial),
					remove: Object.keys($tags).filter(
						(t) => $tags[t] && !$tags[t].checked && !$tags[t].partial
					)
				};
				await assignImageTags([...selectedImages], sourceId, assignments);
				added = assignments.add.length;
				removed = assignments.add.length;
			} else {
				const assignments: TagAssignments = {
					add: Object.keys($tags).filter((t) => $tags[t] && $tags[t].checked && !$tags[t].partial),
					remove: Object.keys($tags).filter(
						(t) => $tags[t] && !$tags[t].checked && !$tags[t].partial
					)
				};
				await assignPathTags([...selectedFolders], sourceId, assignments);
				added = assignments.add.length;
				removed = assignments.add.length;
			}

			good(`Saved tag assignments.`);
			onClose();
		};

		doFn().catch(catchBad);
	};

	refreshTags();
</script>

<Modal>
	<div class="overflow-y-scroll h-full">
		<div class="flex flex-row justify-between items-center">
			<h1 class="text-lg font-bold pb-1">Edit Tags</h1>
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
		<div class="w-full flex flex-row justify-between mt-2">
			<SearchInput bind:value={search} placeholder="Search" className="basis-[250px]" />
			<div>
				<Button variant={Variant.Secondary} onClick={() => (showNewTag = true)} title="Create Tag">
					<Plus className="w-[15px] h-full" />
				</Button>
			</div>
		</div>
		<div class="flex mt-2 mb-4">
			<div class="w-1/2 pr-4 relative">
				<h3 class="text-md font-bold">All Tags</h3>
				<div
					class="mt-2 flex-col space-y-1.5 overflow-scroll min-h-[200px] max-h-[300px] pb-4 relative"
				>
					{#each Object.values($filteredTagsBlock) as { tag, children, collapsed } (tag.id)}
						<Checkbox
							label={tag.name}
							collapseable={children.length > 0}
							{collapsed}
							onClick={() => {
								tags.update((s) => {
									let isChecked = !s[tag.id]?.checked;
									if (s[tag.id]) s[tag.id].checked = isChecked;
									if (!isChecked)
										children.forEach((t) => {
											if (s[t.id]) s[t.id].checked = false;
											if (s[t.id]) s[t.id].partial = false;
										});
									return s;
								});
							}}
							onCollapse={() =>
								tagsBlock.update((b) => {
									if (b[tag.id]) b[tag.id].collapsed = !b[tag.id].collapsed;
									return b;
								})}
							checked={$tags[tag.id]?.checked}
						/>
						{#if children.length > 0 && !collapsed}
							<TagBlock>
								{#each children as child}
									<Checkbox
										label={child.name}
										onClick={() => {
											tags.update((s) => {
												let isChecked = !s[child.id]?.checked;
												if (s[tag.id] && isChecked) {
													s[tag.id].checked = true;
													s[tag.id].partial = false;
												}

												if (s[child.id]) {
													s[child.id].checked = isChecked;
													s[child.id].partial = false;
												}

												return s;
											});
										}}
										checked={$tags[child.id]?.checked}
									/>
								{/each}
							</TagBlock>
						{/if}
					{/each}
					{#if Object.keys($tagsBlock).length === 0}
						<p class="text-gray-400 italic">No tags available.</p>
					{:else if Object.keys($filteredTagsBlock).length === 0}
						<p class="text-gray-400 italic">No matching tags available.</p>
					{/if}
				</div>

				<div
					class="absolute bottom-0 left-0 right-0 h-4 bg-gradient-to-b from-gray-800/0 to-gray-800"
				/>
			</div>
			<div class="w-1/2 border-solid border-l-[1px] border-gray-700 pl-6">
				<h3 class="text-md font-bold">Selected Tags</h3>
				<div class="mt-2 flex-col space-y-1.5">
					{#each Object.values($tagsBlock) as { tag, children, collapsed } (tag.id)}
						{#if $tags[tag.id]?.checked}
							<div class="flex flex-row space-x-2">
								<Checkbox
									label={tag.name}
									collapseable={children.length > 0}
									{collapsed}
									onClick={() => {
										tags.update((s) => {
											if (s[tag.id]) {
												s[tag.id].checked = false;
												s[tag.id].partial = false;
											}
											children.forEach((t) => {
												if (s[t.id]) s[t.id].checked = false;
												if (s[t.id]) s[t.id].partial = false;
											});

											return s;
										});
									}}
									onCollapse={() =>
										tagsBlock.update((b) => {
											if (b[tag.id]) b[tag.id].collapsed = !b[tag.id].collapsed;
											return b;
										})}
									checked
								/>
								{#if $tags[tag.id]?.partial}
									<IconButton
										label="Only some items have this tag selected. Click to fix this."
										icon={Warn}
										onClick={() => {
											tags.update((s) => {
												if (s[tag.id]) s[tag.id].partial = false;
												return s;
											});
										}}
									/>
								{/if}
							</div>
							{#if children.length > 0 && !collapsed}
								<TagBlock>
									{#each children as child}
										<div class="flex flex-row space-x-2">
											<Checkbox
												label={child.name}
												onClick={() => {
													tags.update((s) => {
														if (s[child.id]) s[child.id].checked = !s[child.id].checked;
														if (s[child.id]) s[child.id].partial = false;

														return s;
													});
												}}
												checked={$tags[child.id]?.checked}
											/>
											{#if $tags[child.id]?.partial}
												<IconButton
													label="Only some items have this tag selected. Click to fix this."
													icon={Warn}
													variant={IconVariant.Embedded}
													onClick={() => {
														tags.update((s) => {
															if (s[tag.id]) s[tag.id].partial = false;
															if (s[child.id]) s[child.id].partial = false;
															return s;
														});
													}}
												/>
											{/if}
										</div>
									{/each}
								</TagBlock>
							{/if}
						{/if}
					{/each}
					{#if Object.keys($tagsBlock).filter((id) => $tags[id] && $tags[id].checked).length == 0}
						<p class="text-gray-400 italic">No tags selected.</p>
					{/if}
				</div>
			</div>
		</div>
		<div class="w-full flex flex-row justify-between content-bottom mt-2">
			<div class="flex flex-col justify-between">
				<p class="text-gray-500 inline-block my-auto text-base">Editing 8 Images</p>
			</div>
			<div class="flex space-x-4">
				<Button variant={Variant.Warn} title="Discard Changes" onClick={onClose}>
					<Trash className="w-[16px] h-full" />
				</Button>
				<Button variant={Variant.Primary} title="Save Changes" onClick={submit}>
					<TagIcon className="w-[16px] h-full" />
				</Button>
			</div>
		</div>
	</div>
</Modal>

{#if showNewTag}
	<NewTagModal
		onClose={() => {
			showNewTag = false;
			refreshTags();
		}}
	/>
{/if}
