export interface SearchRule {
	id: string;
	ruleType: string;
	tagId?: string;
}

const KEY = 'search-rules';

export const save = (rules: SearchRule[]) => {
	localStorage.setItem(KEY, JSON.stringify(rules));
};

export const load = () => {
	return JSON.parse(localStorage.getItem(KEY) || '[]') as SearchRule[];
};
