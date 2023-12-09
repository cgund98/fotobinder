import { invoke } from '@tauri-apps/api';

export interface Collection {
	id: string;
	parent_id: string | null;
	name: string;
}

export interface Collections {
	collections: Collection[];
}

export const create = async (name: string, parentId: string | null): Promise<Collection> => {
	const collection = (await invoke('create_collection', { name, parentId })) as Collection;

	return collection;
};

export const list = async (): Promise<Collections> => {
	const collections = (await invoke('list_collections', {})) as Collections;

	return collections;
};

export const listByParentId = async (parentId: string | null): Promise<Collections> => {
	const collections = (await invoke('list_collections_by_parent_id', { parentId })) as Collections;

	return collections;
};

export const get = async (id: string): Promise<Collection> => {
	const collection = (await invoke('get_collection', { id })) as Collection;

	return collection;
};

export const remove = async (id: string): Promise<Collection> => {
	const collection = (await invoke('delete_collection', { id })) as Collection;

	return collection;
};
