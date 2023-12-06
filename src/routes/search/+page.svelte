<script lang="ts">
	import Button, { Variant } from '$lib/components/button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '$lib/components/button/IconButton.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import Adjustments from '$lib/components/icons/Adjustments.svelte';
	import Trash from '$lib/components/icons/Trash.svelte';
	import Plus from '$lib/components/icons/Plus.svelte';
	import Tag from '$lib/components/icons/Tag.svelte';
	import Dropdown from '$lib/components/input/Dropdown.svelte';
	import SearchBox from '$lib/components/input/SearchBox.svelte';
	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import Search from '$lib/components/icons/Search.svelte';
	import Checkbox from '$lib/components/input/Checkbox.svelte';
	import Reset from '$lib/components/icons/Reset.svelte';

	const rules = [
		{ ruleType: 'include', tag: 'Season / Winter' },
		{ ruleType: 'exclude', tag: 'Mountain' },
		{ ruleType: 'exclude', tag: 'Coast' }
	];

	const ruleTypeOptions = [
		{ label: 'Exclude', value: 'exclude' },
		{ label: 'Include', value: 'include' }
	];

	const additionalOptions = [{ label: 'Overlap Included Tags', value: false }];
</script>

<PathHeader path={[{ label: 'Query Builder', route: '/search' }]} />

<div class="my-2">
	<Separator />
</div>

<div class="flex flex-col w-full justify-between space-y-1">
	<div class="flex flex-row justify-between">
		<div class="flex flex-row space-x-8">
			<div class="flex flex-row justify-between px-3 mb-2 w-[120px]">
				<h3 class="font-bold">Rule</h3>
				<Adjustments className="text-gray-500 w-[20px]" />
			</div>
			<div class="flex flex-col space-y-1 w-64">
				<div class="flex flex-row justify-between px-3 mb-2">
					<h3 class="font-bold">Tag</h3>
					<Tag className="text-gray-500 w-[18px]" />
				</div>
			</div>
		</div>

		<div class="flex flex-row items-end pr-2">
			<div class="flex flex-col space-y-1">
				<h3 class="font-bold mb-2">Action</h3>
			</div>
		</div>
	</div>
	{#each rules as r}
		<div class="flex hover:bg-gray-900 rounded-lg flex-row justify-between">
			<div class="flex flex-row space-x-8">
				<div class="flex flex-col space-y-1 w-[120px]">
					<Dropdown options={ruleTypeOptions} value={r.ruleType} />
				</div>
				<div class="flex flex-col space-y-1 w-64">
					<SearchBox options={[]} placeholder="Search Tag" />
				</div>
			</div>

			<div class="flex flex-row items-end pr-2">
				<div class="flex flex-col space-y-1">
					<div class="flex flex-col justify-around h-[48px]">
						<IconButton icon={Trash} variant={IconVariant.Embedded} label="Remove Rule" />
					</div>
				</div>
			</div>
		</div>
	{/each}
</div>

<div class="flex flex-row space-x-3 px-1 mt-3">
	<Button className="" title="Add Rule" variant={Variant.Secondary}>
		<Plus className="w-[15px]" />
	</Button>
</div>

<div class="my-4">
	<Separator />
</div>

<div class="flex flex-col mt-4 px-2 space-y-2">
	<div>
		<h3 class="font-bold">Extra Options</h3>
	</div>

	<div class="flex flex-col space-y-2">
		{#each additionalOptions as option}
			<Checkbox label={option.label} checked={option.value} />
		{/each}
	</div>
</div>

<div class="my-4">
	<Separator />
</div>

<div class="flex justify-between py-1 px-2 mt-2">
	<div class="flex flex-row space-x-3">
		<Button title="Reset" variant={Variant.Warn}>
			<Reset className="w-[15px]" />
		</Button>
	</div>

	<div class="flex flex-row space-x-3">
		<Button title="Search" variant={Variant.Primary} className="" disabled>
			<Search className="w-[15px] -mt-[1px]" />
		</Button>
	</div>
</div>
