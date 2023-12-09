<script lang="ts">
	import Separator from '../decoration/Separator.svelte';
	import Modal from '../layout/Modal.svelte';
	import Close from '../icons/Close.svelte';
	import TextInput from '../input/TextInput.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';
	import Plus from '../icons/Plus.svelte';
	import Trash from '../icons/Trash.svelte';
	import SearchBox from '../input/SearchBox.svelte';
	import { catchBad, good } from '$lib/store/alerts';
	import { list, type Collection, create } from '$lib/api/collection';

	export let onClose: () => void = () => {};

	// Fetch parent collections
	const MAX_LEVELS = 10;
	let parentIdOptions: { label: string; value?: string }[] = [];

	const fetchCollections = () => {
		const doFn = async () => {
			const res = await list();

			let map = res.collections.reduce(
				(m, col) => ({ ...m, [col.id]: col }),
				{} as { [id: string]: Collection }
			);

			const assignNames = (input: Collection[]) =>
				input.map((col) => {
					let name = col.name;
					let parent_id = col.parent_id;

					if (col.parent_id && map[col.parent_id]) {
						name = `${map[col.parent_id].name} | ${name}`;
						parent_id = map[col.parent_id].parent_id;
					}

					return { id: col.id, parent_id, name };
				});

			let filteredCollections = res.collections;
			for (let i = 0; i < MAX_LEVELS - 2; i++) {
				filteredCollections = assignNames(filteredCollections);
			}
			filteredCollections = filteredCollections.filter((col) => col.parent_id === null);

			parentIdOptions = filteredCollections.map((col) => ({ value: col.id, label: col.name }));
			parentIdOptions = [{ label: 'No Parent', value: undefined }, ...parentIdOptions];
		};

		doFn().catch(catchBad);
	};

	fetchCollections();

	// Form values
	let name = '';
	let parentId: string | undefined = undefined;

	// Form validation
	const validate = (name: string, parentId: string | undefined) => {
		// Validate name
		const validName = name.length > 0;

		// Validate parent tag ID
		const validParentTag = parentId === undefined || parentId.length > 0;

		return validName && validParentTag;
	};

	$: valid = validate(name, parentId);
</script>

<Modal>
	<div class="flex flex-row justify-between items-center">
		<h1 class="text-lg font-bold pb-1">New Collection</h1>
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
				options={parentIdOptions}
				bind:value={parentId}
				placeholder="Collection"
				label="Parent Collection"
			/>
		</div>
	</div>

	<div class="w-full flex flex-row justify-between content-bottom mt-4">
		<div class="flex flex-col justify-between"></div>
		<div class="flex space-x-4">
			<Button variant={Variant.Warn} title="Discard" onClick={onClose}>
				<Trash className="w-[16px] h-full" />
			</Button>
			<Button
				variant={Variant.Primary}
				title="Create"
				disabled={!valid}
				onClick={() => {
					create(name, parentId || null)
						.then(() => {
							good(`Created collection '${name}'.`);
							onClose();
						})
						.catch(catchBad);
				}}
			>
				<Plus className="w-[16px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</Modal>
