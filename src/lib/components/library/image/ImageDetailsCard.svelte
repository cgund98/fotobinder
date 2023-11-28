<script lang="ts">
	import Badge from '../../decoration/Badge.svelte';
	import Separator from '../../decoration/Separator.svelte';
	import Close from '../../icons/Close.svelte';
	import Backdrop from '../../layout/Backdrop.svelte';
	import Pencil from '../../icons/Pencil.svelte';
	import IconButton, { Variant } from '../../button/IconButton.svelte';
	import FolderSolid from '../../icons/FolderSolid.svelte';
	import Tag from '../../icons/Tag.svelte';
	import Clipboard from '../../icons/Clipboard.svelte';
	import ChevronDown from '../../icons/ChevronDown.svelte';

	export let name: string;
	export let src: string;

	export let onClose: () => void = () => {};

	const details = [
		{ label: 'Date Taken', value: 'Aug. 26, 2023' },
		{ label: 'Size', value: '1920x1080' }
	];

	const tags = ['Slovenia', 'Grass', 'Forest', 'Water', 'Mountain'];
</script>

<Backdrop>
	<div
		class="flex w-screen sm:w-[90vw] lg:w-[900px] max-h-[80vh] bg-gray-800 rounded-md overflow-hidden grow-0"
	>
		<div class="flex-1 bg-gray-900">
			<div class="flex flex-col justify-around h-full relative">
				<img alt="Highres" {src} class="w-full drop-shadow z-10" />
				<div class="absolute w-full flex justify-around bottom-4 z-20 ease-out">
					<div class="flex bg-gray-700 space-x-2 rounded-lg drop-shadow ease-out">
						<IconButton
							icon={ChevronDown}
							label="Previous Image"
							className="rotate-90 text-gray-400"
							variant={Variant.Embedded}
							shadow={false}
						/>
						<IconButton
							icon={ChevronDown}
							variant={Variant.Embedded}
							label="Next Image"
							className="-rotate-90 text-gray-400"
							shadow={false}
						/>
					</div>
				</div>
			</div>
		</div>
		<div class="w-[250px] grow-0 px-3 py-2 relative">
			<div class="h-full overflow-y-scroll pb-14">
				<div class="flex justify-between items-center mb-2">
					<h2 class="text-lg font-bold truncate">
						{name}
					</h2>
					<div role="button" class="">
						<IconButton
							onClick={onClose}
							icon={Close}
							variant={Variant.Embedded}
							label="Close Window"
						/>
					</div>
				</div>

				<Separator className="bg-gray-700 my-2 mb-3" />

				<div class="">
					{#each details as detail}
						<div class="flex justify-between">
							<span class="text-gray-400 text-sm font-semibold">{detail.label}</span>
							<span class="text-gray-400 text-sm font-base">{detail.value}</span>
						</div>
					{/each}
				</div>

				<Separator className="bg-gray-700 my-3" />

				<div class="flex space-x-1">
					<div class="flex flex-col justify-center">
						<h3 class="font-bold">Tags</h3>
					</div>
					<div role="button" class="text-gray-600 hover:text-gray-200 pointer">
						<IconButton
							icon={Pencil}
							variant={Variant.Embedded}
							className="h-[16px]"
							label="Edit Tags"
						/>
					</div>
				</div>

				<div class="flex flex-wrap mt-1">
					{#each tags as tag}
						<div class="px-1 py-1">
							<Badge>{tag}</Badge>
						</div>
					{/each}
				</div>
			</div>

			<div
				class="absolute bottom-0 pb-4 left-0 w-full flex justify-center space-x-4 gradient bg-gradient-to-b from-gray-800/0 to-gray-800"
			>
				<IconButton icon={FolderSolid} label="Save to Collection" />
				<IconButton icon={Tag} label="Edit Tags" />
				<IconButton icon={Clipboard} label="Copy Path" />
			</div>
		</div>
	</div>
</Backdrop>
