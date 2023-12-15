<script lang="ts">
	import { update, get } from '../../../api/source';
	import type { Source } from '../../../api/source';

	import Button, { Variant } from '../../button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '../../button/IconButton.svelte';
	import Separator from '../../decoration/Separator.svelte';
	import Check from '../../icons/Check.svelte';
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
	export let sourceId: string;

	const fetchSource = async () => {
		try {
			const source = await get(sourceId);
			name = source.name;
		} catch (err) {
			catchBad(err);
		}
	};

	fetchSource();

	const typeOptions = [{ label: 'Local', value: 'Local' }];

	let name: string = '';

	const handleSubmit = async () => {
		loading = true;
		try {
			const source = await update(sourceId, name);
			good(`Updated source '${name}'. `);

			onClose(source);
		} catch (err: any) {
			catchBad(err);
		}
	};

	// Validate inputs
	const validate = (name: string) => {
		let isValid = true;

		// Validate name
		isValid = isValid && name.length > 0;

		return isValid;
	};
	$: valid = validate(name);
</script>

<svelte:window
	on:keydown={(event) => {
		if (!loading && event.key === 'Escape') onClose();
	}}
/>

<Modal>
	<div class="flex flex-row justify-between items-center">
		<h1 class="text-lg font-bold pb-1">Edit Library Source</h1>
		<div class="-mt-1">
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

	<div class="">
		<div class="w-full">
			<TextInput bind:value={name} placeholder="Name" label="Name" />
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
					title="Save"
					disabled={!valid || loading}
				>
					<Check className="w-[16px] -mt-[1px]" />
				</Button>
			</div>
		</div>
	</div></Modal
>

{#if loading}
	<ProgressWrapper />
{/if}
