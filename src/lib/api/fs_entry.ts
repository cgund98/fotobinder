import { invoke } from '@tauri-apps/api';

// export enum SourceType {
// 	Local = 'Local'
// }

// export interface Source {
// 	id: string;
// 	name: string;
// 	path: string;
// 	source_type: SourceType;
// }

// export interface Sources {
// 	sources: Source[];
// }

// export const create = async (name: string, type: string, path: string): Promise<Source> => {
// 	const source = (await invoke('create_source', { name, sourceType: type, path })) as Source;

// 	return source;
// };
