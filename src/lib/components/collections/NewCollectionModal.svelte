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
	import Dropdown from '../input/Dropdown.svelte';
	import PathInput from '../input/PathInput.svelte';

	export let onClose: () => void = () => {};
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
			<TextInput placeholder="Name" label="Name" />
		</div>

		<div class="grow">
			<SearchBox placeholder="Collection" label="Parent Collection" />
		</div>
	</div>

	<div class="w-full flex flex-row justify-between content-bottom mt-4">
		<div class="flex flex-col justify-between"></div>
		<div class="flex space-x-4">
			<Button variant={Variant.Warn} title="Discard">
				<Trash className="w-[16px] h-full" />
			</Button>
			<Button variant={Variant.Primary} title="Create" disabled>
				<Plus className="w-[16px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</Modal>
