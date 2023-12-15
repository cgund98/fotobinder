import { goto } from '$app/navigation';

export interface QueryParams {
	[key: string]: string;
}

export const routeToPage = (route: string, queryParams?: QueryParams) => {
	const params = new URLSearchParams();

	// Set query params
	if (queryParams)
		Object.keys(queryParams).forEach((key) => {
			params.set(key, queryParams[key]);
		});

	goto(`${route}?${params.toString()}`, { replaceState: true });
};
