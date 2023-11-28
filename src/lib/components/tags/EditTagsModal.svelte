<script lang="ts">
	import Modal from '../layout/Modal.svelte';
	import Separator from '../decoration/Separator.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import Tag from '../icons/Tag.svelte';
	import Trash from '../icons/Trash.svelte';
	import Plus from '../icons/Plus.svelte';
	import Checkbox from '../input/Checkbox.svelte';
	import TagBlock from './TagBlock.svelte';
	import Close from '../icons/Close.svelte';
	import SearchInput from '../input/SearchInput.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';

	export let onClose: () => void = () => {};
	export let onNewTag: () => void = () => {};
</script>

<Modal>
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
		<SearchInput placeholder="Search" className="basis-[250px]" />
		<div>
			<Button variant={Variant.Secondary} onClick={onNewTag} title="Create Tag">
				<Plus className="w-[15px] h-full" />
			</Button>
		</div>
	</div>
	<div class="flex mt-2 mb-4">
		<div class="w-1/2 pr-4">
			<h3 class="text-md font-bold">All Tags</h3>
			<div
				class="mt-2 flex-col space-y-1.5 overflow-scroll min-h-[200px] max-h-[300px] pb-8 relative"
			>
				<Checkbox label="Tag" collapseable></Checkbox>
				<TagBlock>
					<Checkbox label="Tag"></Checkbox>
					<Checkbox label="Tag"></Checkbox>
					<Checkbox label="Tag"></Checkbox>
				</TagBlock>
				<Checkbox label="Tag" checked collapseable></Checkbox>
				<TagBlock>
					<Checkbox label="Tag" checked></Checkbox>
					<Checkbox label="Tag"></Checkbox>
				</TagBlock>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<Checkbox label="Tag"></Checkbox>
				<div
					class="absolute bottom-0 left-0 right-0 h-8 bg-gradient-to-b from-gray-800/0 to-gray-800"
				></div>
			</div>
		</div>
		<div class="w-1/2 border-solid border-l-[1px] border-gray-700 pl-6">
			<h3 class="text-md font-bold">Selected Tags</h3>
			<div class="mt-2 flex-col space-y-1.5">
				<Checkbox label="Tag" checked></Checkbox>
				<Checkbox label="Tag2" checked></Checkbox>
				<Checkbox label="Tag3" checked></Checkbox>
			</div>
		</div>
	</div>
	<div class="w-full flex flex-row justify-between content-bottom mt-2">
		<div class="flex flex-col justify-between">
			<p class="text-gray-500 inline-block my-auto text-base">Editing 8 Images</p>
		</div>
		<div class="flex space-x-4">
			<Button variant={Variant.Warn} title="Discard Changes">
				<Trash className="w-[16px] h-full" />
			</Button>
			<Button variant={Variant.Primary} title="Save Changes">
				<Tag className="w-[16px] h-full" />
			</Button>
		</div>
	</div>
</Modal>
