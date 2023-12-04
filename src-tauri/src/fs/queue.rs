use std::sync::Arc;

use crate::{
    data::fs_entry::{entity::AdditionalField, repo::Repo},
    errors::AppError,
};

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub subpath: String,
    pub source_id: String,
    pub source: String,
    pub destination: String,
}

pub type TaskQueue = deadqueue::unlimited::Queue<Task>;

pub fn init_queue() -> Arc<TaskQueue> {
    Arc::new(TaskQueue::new())
}

fn update_entry(task: Task, repo: &Arc<Repo>, w: u32, h: u32) -> Result<(), AppError> {
    // Fetch entry
    let mut entry = repo.get_by_ids(&task.name, &task.subpath, &task.source_id)?;

    // Update generation status
    entry.thumbnail_generating = false;

    // Set additional fields
    let mut fields: Vec<AdditionalField> = Vec::new();
    let size_field = AdditionalField {
        label: String::from("Size"),
        value: format!("{}x{}", w, h),
    };
    fields.push(size_field);
    entry.additional_fields = fields;

    // Persist changes
    repo.save(entry)?;

    Ok(())
}

fn check_in_progress(task: Task, repo: &Arc<Repo>) -> Result<bool, AppError> {
    let entry = repo.get_by_ids(&task.name, &task.subpath, &task.source_id)?;

    Ok(entry.thumbnail_generating)
}

pub async fn queue_proc(queue: Arc<TaskQueue>, repo: Arc<Repo>) {
    println!("Waiting for create thumbnails...");
    loop {
        let task = queue.pop().await;

        println!("Creating thumbnail for image '{}'...", task.source);

        // Check if image is marked as generating thumbnail
        let in_progress = check_in_progress(task.clone(), &repo);
        if let Err(err) = in_progress {
            println!("Error fetching entry for task: {}", err);
            continue;
        } else if let Ok(is_generating) = in_progress {
            if !is_generating {
                println!("Thumbnail marked as not generating. Skipping...");
                continue;
            }
        }

        // Resize image
        let mut w: u32 = 0;
        let mut h: u32 = 0;
        let res = crate::fs::image::gen_thumbnail(task.source.clone(), task.destination.clone());
        if let Err(err) = res {
            println!(
                "Error creating thumbnail for image '{}': {}",
                task.source, err
            );
            continue;
        } else if let Ok((new_w, new_h)) = res {
            w = new_w;
            h = new_h;
        }

        // Update generation status
        let update_res = update_entry(task, &repo, w, h);
        if let Err(err) = update_res {
            println!("Error updating fs entry: {}", err);
        }

        println!("Done! {} images to go...", queue.len())
    }
}
