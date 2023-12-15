<script lang="ts">
	import Separator from '../decoration/Separator.svelte';
	import Modal from '../layout/Modal.svelte';
	import Close from '../icons/Close.svelte';
	import TextInput from '../input/TextInput.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';
	import Trash from '../icons/Trash.svelte';
	import SearchBox from '../input/SearchBox.svelte';
	import { get, list, update, type Tag } from '$lib/api/tag';
	import { catchBad, good } from '$lib/store/alerts';
	import { writable } from 'svelte/store';
	import Check from '../icons/Check.svelte';

	export let onClose: () => void = () => {};
	let tags = writable<Tag[]>([]);

	export let tagId: string;

	const refreshTags = () => {
		list()
			.then((res) => tags.set(res.tags))
			.catch(catchBad);
	};

	refreshTags();

	// Form values
	let name = '';
	let parentId: string | undefined = undefined;

	// Build search options
	const buildOptions = (tags: Tag[]) => {
		const options = tags
			.filter((tag) => tag.parent_id === null)
			.filter((tag) => tag.id !== tagId)
			.map((tag) => ({
				label: tag.name,
				value: tag.id
			}));

		return [{ label: 'No Parent', value: undefined }, ...options];
	};

	// Form validation
	const validate = (name: string, parentId: string | undefined) => {
		// Validate name
		const validName = name.length > 0;

		// Validate parent tag ID
		const validParentTag = parentId === undefined || parentId.length > 0;

		return validName && validParentTag;
	};

	// Fetch tag
	const fetchTag = async () => {
		const tag = await get(tagId);
		name = tag.name;
		parentId = tag.parent_id || undefined;
	};

	fetchTag();

	// Reactive variables
	$: searchOptions = buildOptions($tags);
	$: valid = validate(name, parentId);
</script>

<svelte:window
	on:keydown={(event) => {
		if (event.key === 'Escape') onClose();
	}}
/>

<Modal>
	<div class="flex flex-row justify-between items-center">
		<h1 class="text-lg font-bold pb-1">Edit Tag</h1>
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

	<div class="flex space-x-6">
		<div class="grow">
			<TextInput bind:value={name} placeholder="Name" label="Name" />
		</div>

		<div class="grow">
			<SearchBox
				bind:value={parentId}
				options={searchOptions}
				placeholder="Parent Tag"
				label="Parent Tag"
			/>
		</div>
	</div>

	<div class="w-full flex flex-row justify-between content-bottom mt-3">
		<div class="flex flex-col justify-between">
			<Button variant={Variant.Warn} title="Discard" onClick={onClose}>
				<Trash className="w-[16px] h-full" />
			</Button>
		</div>
		<div class="flex space-x-4">
			<Button
				variant={Variant.Primary}
				title="Save"
				disabled={!valid}
				onClick={() => {
					update(tagId, name, parentId || null)
						.then(() => {
							good(`Updated tag '${name}'.`);
							onClose();
						})
						.catch(catchBad);
				}}
			>
				<Check className="w-[16px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</Modal>
