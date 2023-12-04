use std::{collections::HashSet, path::Path};

use crate::{
    data::fs_entry::{entity, repo::Repo},
    errors::AppError,
    fs::image,
    fs::scan,
};

pub struct Controller {
    repo: Repo,
}

impl Controller {
    pub fn new(repo: Repo) -> Controller {
        Controller { repo }
    }

    pub fn scan_directory(
        &self,
        source_id: String,
        root_dir: String,
        thumbnail_path: &Path,
    ) -> Result<(), AppError> {
        let entries = scan::scan_directory(&root_dir)?;
        let cur_entries = self.list_by_source_id(&source_id)?;
        self.remove_old_entries(&entries, &cur_entries)?;
        self.persist_entries(entries, &source_id, &root_dir, thumbnail_path)?;

        Ok(())
    }

    fn persist_entries(
        &self,
        entries: Vec<scan::ScanEntry>,
        source_id: &str,
        root_dir: &str,
        thumbnail_dir: &Path,
    ) -> Result<(), AppError> {
        for entry in entries {
            let fs_type = match entry.is_dir {
                true => entity::FileType::Directory,
                false => entity::FileType::File,
            };

            let full_path = Path::new(root_dir)
                .join(entry.subpath.clone())
                .join(entry.name.clone());
            let sha256 = match entry.is_dir {
                true => String::from(""),
                false => image::hash(&full_path)?,
            };

            let mut e = entity::FsEntry {
                name: entry.name,
                subpath: entry.subpath,
                source_id: String::from(source_id),
                fs_type,
                hidden: false,
                sha256,
                image_type: entry.ext,
                thumbnail_path: String::from(""),
                additional_fields: Vec::new(),
            };

            // Generate thumbnail
            if e.fs_type == entity::FileType::File {
                image::gen_thumbnail(&mut e, root_dir, thumbnail_dir)?;
            }

            self.repo.save(e)?;
        }

        Ok(())
    }

    fn remove_old_entries(
        &self,
        entries: &Vec<scan::ScanEntry>,
        cur_entries: &Vec<entity::FsEntry>,
    ) -> Result<(), AppError> {
        let mut found: HashSet<String> = HashSet::new();

        for entry in entries {
            let path = Path::new(&entry.subpath).join(&entry.name);
            found.insert(String::from(path.to_string_lossy()));
        }

        for entry in cur_entries {
            let path = Path::new(&entry.subpath).join(&entry.name);
            let path_str = String::from(path.to_string_lossy());
            if found.contains(&path_str) {
                continue;
            }

            self.repo
                .delete(&entry.name, &entry.subpath, &entry.source_id)?;
        }

        Ok(())
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
}
