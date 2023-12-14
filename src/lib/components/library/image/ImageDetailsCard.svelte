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
	import { get, type FsEntry, getImage } from '$lib/api/fs_entry';
	import { listByRelativePath, type Tags } from '$lib/api/tag';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { catchBad } from '$lib/store/alerts';
	import path from 'path-browserify';
	import Tooltip from '$lib/components/decoration/Tooltip.svelte';

	export let relativePath: string;
	export let sourceId: string;
	export let loading: boolean;

	let image: string = '';
	let entry: undefined | FsEntry;
	let tags: undefined | Tags;

	export let onClose: () => void = () => {};
	export let onNext: () => void = () => {};
	export let onPrev: () => void = () => {};

	export const fetchDetails = async () => {
		loading = true;
		try {
			entry = await get(relativePath, sourceId);
			tags = await listByRelativePath(relativePath, sourceId);
			image = await getImage(relativePath, sourceId);
		} catch (err) {
			catchBad(err);
		}
		loading = false;
	};

	fetchDetails();

	$: parts = entry?.relative_path.split(path.sep);
	$: name = parts ? parts[parts.length - 1] : '';

	$: additionalFields = entry ? entry.additional_fields : [];
	$: dimensions = additionalFields.reduce(
		(prev, field) =>
			field.label === 'Size' ? field.value.split('x').map((x) => parseInt(x, 10)) : prev,
		[1920, 1080]
	);
	$: tagElements = tags ? tags.tags : [];
</script>

<svelte:window
	on:keydown={(event) => {
		if (event.key === 'Escape') onClose();
		if (event.key === 'a' || event.key === 'ArrowLeft') onPrev();
		if (event.key === 'd' || event.key === 'ArrowRight') onNext();
	}}
/>

<Backdrop>
	{#if entry && image.length > 0}
		<div
			transition:scale={{ duration: 200, easing: quintOut }}
			class="flex w-screen sm:w-[95vw] max-h-[90vh] bg-gray-800 rounded-md overflow-hidden grow-0"
		>
			<div class="flex-1 bg-gray-900">
				<div class="flex flex-col justify-around h-full relative">
					{#if image.length > 0}
						<img
							alt="Full Resolution"
							src={`data:image/${entry?.image_type};base64,${image}`}
							class="w-full drop-shadow z-10"
						/>
					{:else}
						<div style="width: 100%; height: {Math.min(2000, dimensions[1])}px" />
					{/if}
					<div class="absolute w-full flex justify-around bottom-4 z-20 ease-out">
						<div class="flex bg-gray-700 space-x-2 rounded-lg drop-shadow ease-out">
							<IconButton
								icon={ChevronDown}
								label="Previous Image (a / <-)"
								className="rotate-90 text-gray-400"
								variant={Variant.Embedded}
								shadow={false}
								onClick={onPrev}
							/>
							<IconButton
								icon={ChevronDown}
								variant={Variant.Embedded}
								label="Next Image  (d / ->)"
								className="-rotate-90 text-gray-400"
								shadow={false}
								onClick={onNext}
							/>
						</div>
					</div>
				</div>
			</div>
			<div class="w-[250px] grow-0 px-3 py-2 relative">
				<div class="h-full overflow-y-scroll pb-14">
					<div class="flex justify-between items-center mb-2">
						<h2 class="text-lg font-bold truncate">
							<Tooltip title={name} className="truncate">
								{name}
							</Tooltip>
						</h2>
						<div role="button" class="grow-0">
							<IconButton
								onClick={onClose}
								icon={Close}
								variant={Variant.Embedded}
								label="Close Window (Esc)"
							/>
						</div>
					</div>

					<Separator className="bg-gray-700 my-2 mb-3" />

					<div class="">
						{#each additionalFields as field}
							<div class="flex justify-between">
								<span class="text-gray-400 text-sm font-semibold">{field.label}</span>
								<span class="text-gray-400 text-sm font-base">{field.value}</span>
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
						{#if tagElements.length > 0}
							{#each tagElements as tag}
								<div class="px-1 py-1">
									<Badge>{tag.name}</Badge>
								</div>
							{/each}
						{:else}
							<div class="text-sm text-gray-400">
								<p>No tags assigned.</p>
							</div>
						{/if}
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
	{/if}
</Backdrop>
