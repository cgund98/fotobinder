use std::path::Path;

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

    pub fn scan_directory(&self, source_id: String, root_dir: String) -> Result<(), AppError> {
        let entries = scan::scan_directory(&root_dir)?;
        println!("Entries: {:?}", entries);

        self.persist_entries(entries, &source_id, &root_dir)?;

        Ok(())
    }

    fn persist_entries(
        &self,
        entries: Vec<scan::ScanEntry>,
        source_id: &str,
        root_dir: &str,
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

            let e = entity::FsEntry {
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

            self.repo.save(e)?;
        }

        Ok(())
    }
}
