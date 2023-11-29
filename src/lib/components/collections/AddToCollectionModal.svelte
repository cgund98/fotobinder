<script lang="ts">
	import Modal from '../layout/Modal.svelte';
	import Separator from '../decoration/Separator.svelte';
	import Button, { Variant } from '../button/Button.svelte';
	import Collection from '../icons/FolderSolid.svelte';
	import Trash from '../icons/Trash.svelte';
	import Plus from '../icons/Plus.svelte';
	import Checkbox from '../input/Checkbox.svelte';
	import Close from '../icons/Close.svelte';
	import SearchInput from '../input/SearchInput.svelte';
	import IconButton, { Variant as IconVariant } from '../button/IconButton.svelte';
	import TagBlock from '../tags/TagBlock.svelte';

	export let onClose: () => void = () => {};
	export let onNewTag: () => void = () => {};
</script>

<Modal>
	<div class="overflow-y-scroll h-full">
		<div class="flex flex-row justify-between items-center">
			<h1 class="text-lg font-bold pb-1">Save to Collections</h1>
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
				<Button variant={Variant.Secondary} onClick={onNewTag} title="Create Collection">
					<Plus className="w-[15px] h-full" />
				</Button>
			</div>
		</div>
		<div class="flex mt-2 mb-4">
			<div class="w-1/2 pr-4 relative">
				<h3 class="text-md font-bold">All Collections</h3>
				<div
					class="mt-2 flex-col space-y-1.5 overflow-scroll min-h-[200px] max-h-[300px] pb-4 relative"
				>
					<Checkbox label="Collection" collapseable></Checkbox>
					<TagBlock>
						<Checkbox label="Collection"></Checkbox>
						<Checkbox label="Collection"></Checkbox>
						<Checkbox label="Collection"></Checkbox>
					</TagBlock>
					<Checkbox label="Collection" checked collapseable></Checkbox>
					<TagBlock>
						<Checkbox label="Collection" checked></Checkbox>
						<Checkbox label="Collection"></Checkbox>
					</TagBlock>
					<Checkbox label="Collection"></Checkbox>
				</div>

				<div
					class="absolute bottom-0 left-0 right-0 h-4 bg-gradient-to-b from-gray-800/0 to-gray-800"
				></div>
			</div>
			<div class="w-1/2 border-solid border-l-[1px] border-gray-700 pl-6">
				<h3 class="text-md font-bold">Selected Collections</h3>
				<div class="mt-2 flex-col space-y-1.5">
					<Checkbox label="Collection" checked></Checkbox>
					<Checkbox label="Collection2" checked></Checkbox>
					<Checkbox label="Collection3" checked></Checkbox>
				</div>
			</div>
		</div>
		<div class="w-full flex flex-row justify-between content-bottom mt-2">
			<div class="flex flex-col justify-between">
				<p class="text-gray-500 inline-block my-auto text-base">Saving 8 Images</p>
			</div>
			<div class="flex space-x-4">
				<Button variant={Variant.Warn} title="Discard">
					<Trash className="w-[16px] h-full" />
				</Button>
				<Button variant={Variant.Primary} title="Save">
					<Collection className="w-[16px] h-full" />
				</Button>
			</div>
		</div>
	</div>
</Modal>
