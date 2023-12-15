import { invoke } from '@tauri-apps/api';

export enum SourceType {
	Local = 'Local'
}

export interface Source {
	id: string;
	name: string;
	path: string;
	source_type: SourceType;
}

export interface Sources {
	sources: Source[];
}

export interface ScanResults {
	entries_deleted: number;
	entries_created: number;
	thumbnails_created: number;
}

export const create = async (name: string, type: string, path: string): Promise<Source> => {
	const source = (await invoke('create_source', { name, sourceType: type, path })) as Source;

	return source;
};

export const list = async (): Promise<Sources> => {
	const sources = (await invoke('list_sources', {})) as Sources;

	return sources;
};

export const get = async (id: string): Promise<Source> => {
	const source = (await invoke('get_source', { id })) as Source;

	return source;
};

export const update = async (id: string, name: string): Promise<Source> => {
	const source = (await invoke('update_source', { id, name })) as Source;

	return source;
};

export const remove = async (id: string): Promise<Source> => {
	const source = (await invoke('delete_source', { id })) as Source;

	return source;
};

export const scan = async (id: string): Promise<ScanResults> => {
	return (await invoke('scan_source_dir', { sourceId: id })) as ScanResults;
};
