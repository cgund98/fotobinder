import { invoke } from '@tauri-apps/api';

export enum SourceType {
	Local = 'Local'
}

export enum FileType {
	File = 'File',
	Directory = 'Directory'
}

export enum ImageType {
	None = 'None',
	Jpeg = 'Jpeg',
	Png = 'Png'
}

export interface AdditionalField {
	label: string;
	value: string;
}

export interface FsEntry {
	relative_path: string;
	base_path: string;
	source_id: string;
	fs_type: FileType;
	hidden: boolean;
	image_type: ImageType;
	thumbnail_path: string;
	thumbnail_generating: boolean;
	additional_fields: AdditionalField[];
}

export interface FsEntries {
	entries: FsEntry[];
}

export const listBySourceId = async (sourceId: string, pathPrefix: string) => {
	const entries = (await invoke('list_fs_entries_by_source_id', {
		sourceId,
		pathPrefix
	})) as FsEntries;

	return entries;
};

export const listbyCollectionId = async (collectionId: string) => {
	const entries = (await invoke('list_fs_entries_by_collection_id', {
		collectionId
	})) as FsEntries;

	return entries;
};

export const generateMissingThumbnails = async (sourceId: string) => {
	return (await invoke('generate_missing_thumbnails', {
		sourceId
	})) as number;
};

export const getThumbnailQueueSize = async () => {
	return (await invoke('get_thumbnail_queue_size', {})) as number;
};

export const listByTags = async (
	includes: string[],
	excludes: string[],
	overlapIncludes: boolean
) => {
	const entries = (await invoke('list_fs_entries_by_tags', {
		includes,
		excludes,
		overlapIncludes
	})) as FsEntries;

	return entries;
};

export const get = async (relativePath: string, sourceId: string) => {
	return (await invoke('get_fs_entry_by_ids', {
		relativePath,
		sourceId
	})) as FsEntry;
};

export const getImage = async (relativePath: string, sourceId: string) => {
	return (await invoke('get_fs_entry_image', {
		relativePath,
		sourceId
	})) as string;
};
