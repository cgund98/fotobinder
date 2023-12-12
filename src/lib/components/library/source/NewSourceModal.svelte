<script lang="ts">
	import { create, scan } from '../../../api/source';
	import type { Source } from '../../../api/source';

	import Button, { Variant } from '../../button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '../../button/IconButton.svelte';
	import Separator from '../../decoration/Separator.svelte';
	import FolderSolid from '../../icons/FolderSolid.svelte';
	import Trash from '../../icons/Trash.svelte';
	import Dropdown from '../../input/Dropdown.svelte';
	import PathInput from '../../input/PathInput.svelte';
	import TextInput from '../../input/TextInput.svelte';
	import Modal from '../../layout/Modal.svelte';
	import Close from '../../icons/Close.svelte';
	import { catchBad, good } from '../../../store/alerts';
	import ProgressWrapper from '$lib/components/progress/ProgressWrapper.svelte';

	let loading = false;

	export let onClose: (source?: Source) => void = () => {};

	const typeOptions = [{ label: 'Local', value: 'Local' }];

	let name: string = '';
	let path: string = '';
	let type: string = typeOptions[0].value;

	const handleSubmit = async () => {
		loading = true;
		try {
			const source = await create(name, type, path);
			let scanRes = await scan(source.id);
			good(
				`Created source '${source.name}'. Found ${scanRes.entries_created} file entries when scanned.`
			);
			onClose(source);
		} catch (err: any) {
			catchBad(err);
		}
		loading = false;
	};

	// Validate inputs
	const validate = (name: string, path: string, type: string) => {
		let isValid = true;

		// Validate name
		isValid = isValid && name.length > 0;

		// Validate path
		isValid = isValid && path.length > 0;

		// Validate type
		isValid = isValid && type.length > 0;

		return isValid;
	};
	$: valid = validate(name, path, type);
</script>

<Modal>
	<div class="flex flex-row justify-between items-center">
		<h1 class="text-lg font-bold pb-1">New Library Source</h1>
		<div role="button" class="-mt-1">
			<IconButton
				onClick={() => onClose()}
				icon={Close}
				variant={IconVariant.Embedded}
				label="Close Window"
				disabled={loading}
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

		<div class="w-48">
			<Dropdown bind:value={type} options={typeOptions} label="Type" />
		</div>
	</div>

	<div class="flex space-x-6">
		<div class="grow">
			<PathInput bind:value={path} placeholder="Path" label="Path" />
		</div>
	</div>

	<div class="w-full flex flex-row justify-between content-bottom mt-4">
		<div class="flex flex-col justify-between"></div>
		<div class="flex space-x-4">
			<Button onClick={() => onClose()} disabled={loading} variant={Variant.Warn} title="Discard">
				<Trash className="w-[16px] h-full" />
			</Button>
			<Button
				onClick={() => {
					if (!loading) handleSubmit();
				}}
				variant={Variant.Primary}
				title="Add Source"
				disabled={!valid}
			>
				<FolderSolid className="w-[16px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</Modal>

{#if loading}
	<ProgressWrapper />
{/if}
