import type { Collection } from '$lib/api/collection';
import type { Source } from '$lib/api/source';
import { writable } from 'svelte/store';

export const collections = writable<Collection[]>([]);

export const sources = writable<Source[]>([]);
