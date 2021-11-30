pub struct MemBlock {
    pub start: usize,
    pub end: usize,
    pub size: usize,
    pub password: Option<String>,
}

pub struct FreeBlock {
    pub start: usize,
    pub end: usize,
}
