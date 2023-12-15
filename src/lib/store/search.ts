export interface SearchOptions {
	rules: SearchRule[];
	overlapIncludes: boolean;
}
export interface SearchRule {
	id: string;
	ruleType: string;
	tagId?: string;
}

const KEY = 'search-options';

export const save = (options: SearchOptions) => {
	localStorage.setItem(KEY, JSON.stringify(options));
};

export const load = () => {
	const defaultStr = '{"rules":[],"overlapIncludes":false}';
	return JSON.parse(localStorage.getItem(KEY) || defaultStr) as SearchOptions;
};
