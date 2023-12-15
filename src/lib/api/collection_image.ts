import { invoke } from '@tauri-apps/api';

export interface CollectionImage {
	relative_path: string;
	source_id: string;
	collection_id: string;
}

export interface CollectionImages {
	collections: CollectionImage[];
}

export const create = async (
	collectionId: string,
	relativePath: string,
	sourceId: string
): Promise<CollectionImage> => {
	const collection = (await invoke('create_collection_image', {
		collectionId,
		relativePath,
		sourceId
	})) as CollectionImage;

	return collection;
};

export const list = async (): Promise<CollectionImages> => {
	const collections = (await invoke('list_collection_images', {})) as CollectionImages;

	return collections;
};

export const get = async (
	collectionId: string,
	relativePath: string,
	sourceId: string
): Promise<CollectionImage> => {
	const collection = (await invoke('get_collection_image', {
		collectionId,
		relativePath,
		sourceId
	})) as CollectionImage;

	return collection;
};

export const remove = async (
	collectionId: string,
	relativePath: string,
	sourceId: string
): Promise<CollectionImage> => {
	const collection = (await invoke('delete_collection_image', {
		collectionId,
		relativePath,
		sourceId
	})) as CollectionImage;

	return collection;
};

export const assignMany = async (
	collectionId: string,
	relativePaths: string[],
	sourceIds: string[]
): Promise<CollectionImage> => {
	const collection = (await invoke('assign_many_collection_images', {
		collectionId,
		relativePaths,
		sourceIds
	})) as CollectionImage;

	return collection;
};
