import { invoke } from '@tauri-apps/api';

export interface ImageTag {
	relative_path: string;
	source_id: string;
	tag_id: string;
}

export interface ImageTags {
	image_tags: ImageTag[];
}

export interface TagAssignments {
	add: string[];
	remove: string[];
}

export const create = async (name: string, parentId: string): Promise<ImageTag> => {
	const imageTag = (await invoke('create_image_tag', { name, parentId })) as ImageTag;

	return imageTag;
};

export const list = async (): Promise<ImageTags> => {
	const imageTags = (await invoke('list_image_tags', {})) as ImageTags;

	return imageTags;
};

export const listByRelativePath = async (
	relativePath: string,
	sourceId: string
): Promise<ImageTags> => {
	const imageTags = (await invoke('list_image_tags_by_relative_path', {
		relativePath,
		sourceId
	})) as ImageTags;

	return imageTags;
};

export const get = async (id: string): Promise<ImageTag> => {
	const imageTag = (await invoke('get_image_tag', { id })) as ImageTag;

	return imageTag;
};

export const remove = async (
	tagId: string,
	sourceId: string,
	relativePath: string
): Promise<ImageTag> => {
	const imageTag = (await invoke('delete_image_tag', {
		tagId,
		sourceId,
		relativePath
	})) as ImageTag;

	return imageTag;
};

export const assign = async (
	relativePaths: string[],
	sourceId: string,
	assignments: TagAssignments
) => {
	await invoke('assign_image_tags', {
		relativePaths,
		sourceId,
		assignments
	});
};
