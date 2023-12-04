import type { QueryParams } from '$lib/nav/route';

export interface HeaderEntry {
	label: string;
	route: string;
	queryParams?: QueryParams;
}
