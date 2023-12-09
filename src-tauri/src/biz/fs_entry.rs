use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};

use crate::{
    api::fs_entry::ScanResults,
    data::fs_entry::{entity, repo::Repo},
    errors::AppError,
    fs::image,
    fs::{
        queue::{Task, TaskQueue},
        scan,
    },
};

pub struct Controller {
    repo: Arc<Repo>,
    queue: Arc<TaskQueue>,
    thumbnails_path: String,
}

impl Controller {
    pub fn new(repo: Arc<Repo>, queue: Arc<TaskQueue>, thumbnails_path: String) -> Controller {
        Controller {
            repo,
            queue,
            thumbnails_path,
        }
    }

    pub fn generate_missing_thumbnails(
        &self,
        source_id: &str,
        root_dir: &str,
    ) -> Result<u32, AppError> {
        let thumbs_path = Path::new(&self.thumbnails_path);

        let entries = self
            .repo
            .list_by_source_id_and_missing_thumbnails(source_id)?;

        let mut count: u32 = 0;
        for entry in entries {
            // Generate source and destinations for thumbnails
            let th_path = image::gen_origin_path(&entry, root_dir);
            let th_path_str = String::from(th_path.to_string_lossy());
            let dest_path = image::gen_thumbnail_path(&th_path, thumbs_path);
            let dest_path_str = String::from(dest_path.to_string_lossy());

            // Add task to queue
            self.queue.push(Task {
                relative_path: entry.relative_path,
                source_id: entry.source_id,
                source: th_path_str,
                destination: dest_path_str,
            });

            count += 1;
        }

        Ok(count)
    }

    pub fn scan_directory(&self, source_id: &str, root_dir: &str) -> Result<ScanResults, AppError> {
        let thumbs_path = Path::new(&self.thumbnails_path);

        // Scan filesystem
        let entries = scan::scan_directory(root_dir)?;
        let cur_entries = self.list_by_source_id(source_id)?;
        let (deleted_count, entries_map) = self.remove_old_entries(&entries, &cur_entries)?;

        // Calc
        let scanned_count = entries.len();
        let reused_entries = entries_map.len();

        // Persist entries
        let th_created =
            self.persist_entries(entries, entries_map, source_id, root_dir, thumbs_path)?;

        let results = ScanResults {
            entries_created: scanned_count - reused_entries,
            entries_deleted: deleted_count,
            thumbnails_created: th_created,
        };

        Ok(results)
    }

    fn persist_entries(
        &self,
        entries: Vec<scan::ScanEntry>,
        entries_map: HashMap<entity::FsEntryIds, entity::FsEntry>,
        source_id: &str,
        root_dir: &str,
        thumbnails_path: &Path,
    ) -> Result<usize, AppError> {
        let mut thumbnail_count = 0;

        let thumbs_path_str = String::from(thumbnails_path.to_string_lossy());

        for entry in entries {
            let fs_type = match entry.is_dir {
                true => entity::FileType::Directory,
                false => entity::FileType::File,
            };

            let full_path = Path::new(root_dir).join(entry.relative_path.clone());
            let sha256 = match entry.is_dir {
                true => String::from(""),
                false => image::hash(&full_path)?,
            };

            let mut e = entity::FsEntry {
                base_path: entry.base_path,
                relative_path: entry.relative_path,
                source_id: String::from(source_id),
                fs_type,
                hidden: false,
                sha256: String::from(""),
                image_type: entry.ext,
                thumbnail_path: String::from(""),
                thumbnail_generating: true,
                additional_fields: Vec::new(),
            };

            // If entry already exists, use that as a base
            let ids = entity::FsEntryIds(e.relative_path.clone(), e.source_id.clone());
            if entries_map.contains_key(&ids) {
                e = entries_map.get(&ids).unwrap().clone();
            }

            let mut task: Option<Task> = None;

            // Generate thumbnail
            let th_path = image::gen_origin_path(&e, root_dir);
            let th_path_str = String::from(th_path.to_string_lossy());
            let dest_path = image::gen_thumbnail_path(&th_path, thumbnails_path);
            let dest_path_str = String::from(dest_path.to_string_lossy());
            if e.fs_type == entity::FileType::File && (!e.sha256.eq(&sha256) || !dest_path.exists())
            {
                // Add task to queue
                task = Some(Task {
                    relative_path: e.relative_path.clone(),
                    source_id: e.source_id.clone(),
                    source: th_path_str,
                    destination: dest_path_str.clone(),
                });

                // Update entity
                e.sha256 = sha256;
                e.thumbnail_path = dest_path_str.replace(&thumbs_path_str, "");
                e.thumbnail_generating = true;

                thumbnail_count += 1;
            }

            self.repo.save(e)?;

            if let Some(t) = task {
                self.queue.push(t);
            }
        }

        Ok(thumbnail_count)
    }

    fn remove_old_entries(
        &self,
        entries: &Vec<scan::ScanEntry>,
        cur_entries: &Vec<entity::FsEntry>,
    ) -> Result<(usize, HashMap<entity::FsEntryIds, entity::FsEntry>), AppError> {
        let mut found: HashSet<String> = HashSet::new();
        let mut entries_map: HashMap<entity::FsEntryIds, entity::FsEntry> = HashMap::new();

        // Populate hashset with scanned entities
        for entry in entries {
            found.insert(entry.relative_path.clone());
        }

        // Delete entries in the datastore that haven't been scanned
        let mut deleted_count: usize = 0;
        for entry in cur_entries {
            if found.contains(&entry.relative_path) {
                entries_map.insert(
                    entity::FsEntryIds(entry.relative_path.clone(), entry.source_id.clone()),
                    entry.clone(),
                );
                continue;
            }

            // Delete thumbnail
            let mut th_path_str = self.thumbnails_path.clone();
            th_path_str.push_str(&entry.thumbnail_path);
            let th_path = Path::new(&th_path_str);
            if th_path.exists() {
                std::fs::remove_file(th_path)?
            }

            // Delete entity
            self.repo.delete(&entry.relative_path, &entry.source_id)?;

            deleted_count += 1;
        }

        Ok((deleted_count, entries_map))
    }

    pub fn list_by_source_id_and_path_prefix(
        &self,
        source_id: &str,
        path_prefix: &str,
    ) -> Result<Vec<entity::FsEntry>, AppError> {
        self.repo
            .list_by_source_id_and_path_prefix(source_id, path_prefix)
    }

    pub fn list_by_source_id(&self, source_id: &str) -> Result<Vec<entity::FsEntry>, AppError> {
        self.repo.list_by_source_id(source_id)
    }

    pub fn list_by_collection_id(
        &self,
        collection_id: &str,
    ) -> Result<Vec<entity::FsEntry>, AppError> {
        self.repo.list_by_collection_id(collection_id)
    }

    pub fn list_by_tags(
        &self,
        includes: Vec<String>,
        excludes: Vec<String>,
    ) -> Result<Vec<entity::FsEntry>, AppError> {
        self.repo.list_by_tags(includes, excludes)
    }
}
