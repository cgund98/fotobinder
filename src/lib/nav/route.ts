import { goto } from '$app/navigation';

export const routeToPage = (route: string) => goto(`/${route}`, { replaceState: true });
