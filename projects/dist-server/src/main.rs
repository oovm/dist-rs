use std::mem::size_of;

use chrono::NaiveDateTime;
use sled::Result;
use uuid::{Timestamp, Uuid};

pub struct User {
    id: Uuid,
}

pub enum Archive {
    Borrowed(Uuid),
    Owned(ArchiveMeta),
}

pub struct ArchiveMeta {
    hash: u128,
    kind: ArchiveKind,
    edit: u64,
    last: Option<Uuid>,
    next: Option<Uuid>,
}

impl Archive {
    pub fn id(&self) -> &Uuid {
        match self {
            Archive::Borrowed(v, _) => { v }
            Archive::Owned(v, _) => { v }
        }
    }
    pub async fn meta(&self) {
        match self {
            Archive::Borrowed(_, _) => {}
            Archive::Owned(_, v) => {
                // v.clone()
            }
        }
    }

    pub fn create_time(&self) -> Option<NaiveDateTime> {
        let (s, ms) = self.id().get_timestamp()?.to_unix();
        let naive = NaiveDateTime::from_timestamp(s as i64, ms);
        Some(naive)
    }
    /// ref no edit time
    pub fn edit_time(&self) -> Option<Timestamp> {
        match self {
            Archive::Borrowed(v, _) => {
                // v.get_timestamp()
            }
            Archive::Owned(_, v) => {
                // v
            }
        }
        todo!()
    }
    pub fn last_version(&self) -> Option<ArchiveMeta> {
        todo!()
    }
    pub fn next_version(&self) -> Option<ArchiveMeta> {
        todo!()
    }
}

impl ArchiveMeta {
    pub fn edit_time(&self) {
        // self.create
    }
}

pub enum ArchiveKind {
    Copy(Uuid),
    Text(Box<TextFile>),
}

#[test]
pub fn cont() {
    println!("{}", size_of::<NaiveDateTime>())
}

pub struct TextFile {
    encoding: TextEncoding,
}

#[repr(C)]
pub enum TextEncoding {
    None = 0,
    ASCII = 1,
    UTF8 = 2,
}

impl TextFile {
    pub fn chars(&self) {
        match self.encoding {
            TextEncoding::None => {}
            TextEncoding::UTF8 => {}
            TextEncoding::ASCII => {}
        }
    }
}


impl Default for TextEncoding {
    fn default() -> Self {
        Self::None
    }
}

pub struct PngImage {
    buffer: Vec<u8>,
}

// impl ArchiveOwned {
//     pub fn is_dir(&self) {}
//     pub fn is_file(&self) {}
// }


#[tokio::main]
async fn main() -> Result<()> {
    let db = sled::Config::new()
        .path("/database")
        .use_compression(true)
        .open()?;

// insert and get, similar to std's BTreeMap
    let key = Uuid::default();

    let old_value = db.insert(key, Archive::Borrowed())?;

    assert_eq!(
        db.get(&key)?,
        Some(sled::IVec::from("value")),
    );

// range queries
//     for kv_result in tree.range("key_1".."key_9") {}

// deletion
//     let old_value = tree.remove(&key)?;

// atomic compare and swap
//     tree.compare_and_swap(
//         key,
//         Some("current_value"),
//         Some("new_value"),
//     )?.unwrap();

// block until all operations are stable on disk
// (flush_async also available to get a Future)
    let buffer = db.flush_async().await?;
    println!("写入: {buffer}");

    Ok(())
}
