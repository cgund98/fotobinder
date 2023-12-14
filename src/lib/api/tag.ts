import { invoke } from '@tauri-apps/api';

export interface Tag {
	id: string;
	parent_id: string | null;
	name: string;
}

export interface Tags {
	tags: Tag[];
}

export const create = async (name: string, parentId: string | null): Promise<Tag> => {
	const tag = (await invoke('create_tag', { name, parentId })) as Tag;

	return tag;
};

export const list = async (): Promise<Tags> => {
	const tags = (await invoke('list_tags', {})) as Tags;

	return tags;
};

export const listByRelativePath = async (relativePath: string, sourceId: string): Promise<Tags> => {
	const tags = (await invoke('list_tags_by_relative_path', { relativePath, sourceId })) as Tags;

	return tags;
};

export const get = async (id: string): Promise<Tag> => {
	const tag = (await invoke('get_tag', { id })) as Tag;

	return tag;
};

export const remove = async (id: string): Promise<Tag> => {
	const tag = (await invoke('delete_tag', { id })) as Tag;

	return tag;
};
