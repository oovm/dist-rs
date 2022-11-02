use std::mem::size_of;

use anyhow::Result;
use chrono::NaiveDateTime;
use opendal::Operator;
use opendal::Scheme;
use uuid::{Timestamp, Uuid};

use futures::StreamExt;
use futures::TryStreamExt;

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
        todo!()
    }
    pub async fn meta(&self) {
        todo!()
    }

    pub fn create_time(&self) -> Option<NaiveDateTime> {
        let (s, ms) = self.id().get_timestamp()?.to_unix();
        let naive = NaiveDateTime::from_timestamp(s as i64, ms);
        Some(naive)
    }
    /// ref no edit time
    pub fn edit_time(&self) -> Option<Timestamp> {
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


