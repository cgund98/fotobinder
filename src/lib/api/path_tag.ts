import { invoke } from '@tauri-apps/api';
import type { TagAssignments } from './image_tag';

export interface PathTag {
	base_path: string;
	source_id: string;
	tag_id: string;
}

export interface PathTags {
	path_tags: PathTag[];
}

export const create = async (name: string, parentId: string): Promise<PathTag> => {
	const pathTag = (await invoke('create_path_tag', { name, parentId })) as PathTag;

	return pathTag;
};

export const list = async (): Promise<PathTags> => {
	const pathTags = (await invoke('list_path_tags', {})) as PathTags;

	return pathTags;
};

export const listByBasePath = async (basePath: string, sourceId: string): Promise<PathTags> => {
	const imageTags = (await invoke('list_path_tags_by_base_path', {
		basePath,
		sourceId
	})) as PathTags;

	return imageTags;
};

export const get = async (id: string): Promise<PathTag> => {
	const pathTag = (await invoke('get_path_tag', { id })) as PathTag;

	return pathTag;
};

export const remove = async (id: string): Promise<PathTag> => {
	const pathTag = (await invoke('delete_path_tag', { id })) as PathTag;

	return pathTag;
};

export const assign = async (
	basePaths: string[],
	sourceId: string,
	assignments: TagAssignments
) => {
	await invoke('assign_path_tags', {
		basePaths,
		sourceId,
		assignments
	});
};
