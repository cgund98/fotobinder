<script lang="ts">
	import Button, { Variant } from '../../button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '../../button/IconButton.svelte';
	import Separator from '../../decoration/Separator.svelte';
	import FolderSolid from '../../icons/FolderSolid.svelte';
	import Trash from '../../icons/Trash.svelte';
	import Dropdown from '../../input/Dropdown.svelte';
	import PathInput from '../../input/PathInput.svelte';
	import SearchBox from '../../input/SearchBox.svelte';
	import TextInput from '../../input/TextInput.svelte';
	import Modal from '../../layout/Modal.svelte';
	import Close from '../../icons/Close.svelte';

	export let onClose: () => void = () => {};
</script>

<Modal>
	<div class="flex flex-row justify-between items-center">
		<h1 class="text-lg font-bold pb-1">New Library Source</h1>
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
			<TextInput placeholder="Name" label="Name" />
		</div>

		<div class="w-48">
			<Dropdown options={[{ label: 'Local', value: 'local' }]} value="local" label="Type" />
		</div>
	</div>

	<div class="flex space-x-6">
		<div class="grow">
			<PathInput placeholder="Path" label="Path" />
		</div>
	</div>

	<div class="w-full flex flex-row justify-between content-bottom mt-4">
		<div class="flex flex-col justify-between"></div>
		<div class="flex space-x-4">
			<Button variant={Variant.Warn} title="Discard">
				<Trash className="w-[16px] h-full" />
			</Button>
			<Button variant={Variant.Primary} title="Add Source" disabled>
				<FolderSolid className="w-[16px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</Modal>
