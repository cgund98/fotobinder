import type { QueryParams } from '$lib/nav/route';

export interface HeaderEntry {
	label: string;
	route: string;
	afterRoute?: () => void;
	queryParams?: QueryParams;
}
