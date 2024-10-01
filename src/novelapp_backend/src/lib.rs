use candid::{CandidType, Deserialize};  // Importing from candid crate
use ic_cdk::{query, update};            // Importing query and update macros from ic_cdk
use std::collections::HashMap;

#[derive(Clone, CandidType, Deserialize)]
struct Chapter {
    title: String,
    content: String,
}

#[derive(Clone, CandidType, Deserialize)]
struct Novel {
    title: String,
    chapters: Vec<Chapter>,
}

type Novels = HashMap<String, Novel>; // Map from novel ID to the Novel

// This will store all novels
static mut NOVELS: Option<Novels> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        NOVELS = Some(HashMap::new());
    }
}

#[update]
fn add_novel(id: String, title: String) {
    let novel = Novel {
        title,
        chapters: vec![],
    };
    unsafe {
        if let Some(ref mut novels) = NOVELS {
            novels.insert(id, novel);
        }
    }
}

#[update]
fn add_chapter(novel_id: String, title: String, content: String) {
    unsafe {
        if let Some(ref mut novels) = NOVELS {
            if let Some(novel) = novels.get_mut(&novel_id) {
                novel.chapters.push(Chapter { title, content });
            }
        }
    }
}

#[query]
fn get_novel(novel_id: String) -> Option<Novel> {
    unsafe {
        if let Some(ref novels) = NOVELS {
            novels.get(&novel_id).cloned()
        } else {
            None
        }
    }
}

#[query]
fn list_novels() -> Vec<String> {
    unsafe {
        if let Some(ref novels) = NOVELS {
            novels.keys().cloned().collect()
        } else {
            vec![]
        }
    }
}

