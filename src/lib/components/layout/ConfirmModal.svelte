<script lang="ts">
	import Separator from '../decoration/Separator.svelte';
	import Modal from '../layout/Modal.svelte';
	import Close from '../icons/Close.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';
	import Cancel from '../icons/Cancel.svelte';
	import Check from '../icons/Check.svelte';

	export let onClose: () => void = () => {};
	export let onConfirm: () => void = () => {};

	export let title: string;
	export let message: string;
</script>

<Modal>
	<div class="flex flex-row justify-between items-center">
		<h1 class="text-lg font-bold pb-1">{title}</h1>
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
		<p>{message}</p>
	</div>

	<div class="w-full flex flex-row justify-between content-bottom mt-4">
		<div class="flex flex-col justify-between"></div>
		<div class="flex space-x-4">
			<Button variant={Variant.Warn} title="Cancel" onClick={onClose}>
				<Cancel className="w-[16px] h-full" />
			</Button>
			<Button variant={Variant.Primary} title="Confirm" onClick={onConfirm}>
				<Check className="w-[16px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</Modal>
