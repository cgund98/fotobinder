import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	console.log('Page loading...');
	return {
		sourceId: params.source
	};
};
