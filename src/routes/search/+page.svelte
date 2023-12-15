<script lang="ts">
	import { v4 } from 'uuid';

	import Button, { Variant } from '$lib/components/button/Button.svelte';
	import IconButton, { Variant as IconVariant } from '$lib/components/button/IconButton.svelte';
	import Separator from '$lib/components/decoration/Separator.svelte';
	import Adjustments from '$lib/components/icons/Adjustments.svelte';
	import Trash from '$lib/components/icons/Trash.svelte';
	import Plus from '$lib/components/icons/Plus.svelte';
	import TagIcon from '$lib/components/icons/Tag.svelte';
	import Dropdown from '$lib/components/input/Dropdown.svelte';
	import SearchBox from '$lib/components/input/SearchBox.svelte';
	import PathHeader from '$lib/components/library/header/PathHeader.svelte';
	import Search from '$lib/components/icons/Search.svelte';
	import Reset from '$lib/components/icons/Reset.svelte';
	import { list, type Tag } from '$lib/api/tag';
	import { catchBad } from '$lib/store/alerts';
	import { type SearchRule, save, load } from '$lib/store/search';
	import { routeToPage } from '$lib/nav/route';
	import PageTransitionWrapper from '$lib/components/layout/PageTransitionWrapper.svelte';

	// Fetch tags
	let tagOptions: { label: string; value: string }[] = [];

	const refreshTags = () => {
		const doFn = async () => {
			const res = await list();

			// Parse search options
			const names = res.tags.reduce(
				(m, t) => {
					m[t.id] = t.name;
					return m;
				},
				{} as { [id: string]: string }
			);

			tagOptions = res.tags.map((t) => {
				let label = t.name;

				if (t.parent_id) label = `${names[t.parent_id]} | ${label}`;

				return {
					label,
					value: t.id
				};
			});
		};
		doFn().catch(catchBad);
	};

	refreshTags();

	const ruleTypeOptions = [
		{ label: 'Include', value: 'include' },
		{ label: 'Exclude', value: 'exclude' }
	];

	let rules: SearchRule[] = load();
	if (rules.length === 0) rules = [{ id: v4(), ruleType: ruleTypeOptions[0].value }];

	// let additionalOptions = {
	// 	overlapIncluded: { label: 'Included tags intersect one another', value: false }
	// };

	$: valid = rules.reduce(
		(prev, rule) => prev && rule.tagId !== undefined && rule.tagId.length > 0,
		true
	);
</script>

<PageTransitionWrapper>
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
						<TagIcon className="text-gray-500 w-[18px]" />
					</div>
				</div>
			</div>

			<div class="flex flex-row items-end pr-2">
				<div class="flex flex-col space-y-1">
					<h3 class="font-bold mb-2">Action</h3>
				</div>
			</div>
		</div>
		{#each rules as r, idx (r.id)}
			<div class="flex hover:bg-gray-900 rounded-lg flex-row justify-between">
				<div class="flex flex-row space-x-8">
					<div class="flex flex-col space-y-1 w-[120px]">
						<Dropdown
							options={ruleTypeOptions}
							value={r.ruleType}
							onChange={(value) => {
								rules[idx] = { ...r, ruleType: value };
								rules = [...rules];
							}}
						/>
					</div>
					<div class="flex flex-col space-y-1 w-64">
						<SearchBox
							options={tagOptions}
							placeholder="Search Tag"
							value={r.tagId}
							onChange={(value) => {
								rules[idx] = { ...r, tagId: value };
								rules = [...rules];
							}}
						/>
					</div>
				</div>

				<div class="flex flex-row items-end pr-2">
					<div class="flex flex-col space-y-1">
						<div class="flex flex-col justify-around h-[48px]">
							<IconButton
								icon={Trash}
								variant={IconVariant.Embedded}
								label="Remove Rule"
								disabled={rules.length < 2}
								onClick={() => {
									rules.splice(idx, 1);
									rules = [...rules];
								}}
							/>
						</div>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<div class="flex flex-row space-x-3 px-1 mt-3">
		<Button
			className=""
			title="Add Rule"
			variant={Variant.Secondary}
			onClick={() => {
				rules = [...rules, { id: v4(), ruleType: ruleTypeOptions[0].value }];
			}}
			disabled={rules.length > 9}
		>
			<Plus className="w-[15px]" />
		</Button>
	</div>

	<!-- <div class="my-4">
	<Separator />
</div>

<div class="flex flex-col mt-4 px-2 space-y-2">
	<div>
		<h3 class="font-bold">Extra Options</h3>
	</div>

	<div class="flex flex-col space-y-2">
		{#each Object.entries(additionalOptions) as [id, option]}
			<Checkbox
				label={option.label}
				checked={option.value}
				onClick={() => {
					additionalOptions = {
						...additionalOptions,
						[id]: { ...option, value: !option.value }
					};
				}}
			/>
		{/each}
	</div>
</div> -->

	<div class="my-4">
		<Separator />
	</div>

	<div class="flex justify-between py-1 px-2 mt-2">
		<div class="flex flex-row space-x-3">
			<Button
				title="Reset"
				variant={Variant.Warn}
				onClick={() => {
					rules = [{ id: v4(), ruleType: ruleTypeOptions[0].value }];
					save(rules);
				}}
			>
				<Reset className="w-[15px]" />
			</Button>
		</div>

		<div class="flex flex-row space-x-3">
			<Button
				title="Search"
				variant={Variant.Primary}
				className=""
				disabled={!valid}
				onClick={() => {
					save(rules);
					routeToPage('/search/results');
				}}
			>
				<Search className="w-[15px] -mt-[1px]" />
			</Button>
		</div>
	</div>
</PageTransitionWrapper>
