use std::{
    fs::OpenOptions,
    io::{Read, Seek},
    marker::PhantomData,
    os::windows::fs::OpenOptionsExt,
};

pub(super) struct File<'a> {
    file: std::fs::File,
    path: &'a str,
    _marker: PhantomData<&'a ()>,
}

impl<'a> File<'a> {
    #[inline(always)]
    pub(super) unsafe fn new(path: &'a str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .share_mode(0x00000001) // FILE_SHARE_READ
            .open(path)
            .unwrap_unchecked();
        Self {
            file,
            path,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub(super) unsafe fn read(&mut self) -> u8 {
        let mut buffer = [0u8; 1];
        self.file.read_exact(&mut buffer).unwrap_unchecked();
        *buffer.get_unchecked(0)
    }

    pub(super) unsafe fn file_size(&self) -> u64 {
        self.file.metadata().unwrap_unchecked().len()
    }

    #[inline(always)]
    pub(super) unsafe fn seek(&mut self, pos: u64) {
        self.file
            .seek(std::io::SeekFrom::Start(pos))
            .unwrap_unchecked();
    }

    #[inline(always)]
    pub(super) unsafe fn position(&mut self) -> u64 {
        self.file.stream_position().unwrap_unchecked()
    }
}

impl Clone for File<'_> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe { Self::new(self.path) }
    }
}
