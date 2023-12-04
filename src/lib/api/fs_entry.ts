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
	name: string;
	subpath: string;
	source_id: string;
	fs_type: FileType;
	hidden: boolean;
	image_type: ImageType;
	thumbnail_path: string;
	additional_fields: AdditionalField[];
}

export interface FsEntries {
	entries: FsEntry[];
}

export const list_by_source_id = async (sourceId: string, pathPrefix: string) => {
	const entries = (await invoke('list_fs_entries_by_source_id', {
		sourceId,
		pathPrefix
	})) as FsEntries;

	return entries;
};
