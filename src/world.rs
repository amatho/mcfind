use std::fmt::Display;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Chunk {
        Chunk { x, z }
    }

    pub fn display_world_coords(&self) -> ChunkWorldDisplay {
        ChunkWorldDisplay { chunk: self }
    }

    pub fn display_chunk_coords(&self) -> ChunkDisplay {
        ChunkDisplay { chunk: self }
    }
}

pub struct ChunkWorldDisplay<'a> {
    chunk: &'a Chunk,
}

impl Display for ChunkWorldDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let world_x = self.chunk.x * 16;
        let world_z = self.chunk.z * 16;
        write!(
            f,
            "({}, {}) to ({}, {})",
            world_x,
            world_z,
            world_x + 15,
            world_z + 15
        )
    }
}

pub struct ChunkDisplay<'a> {
    chunk: &'a Chunk,
}

impl Display for ChunkDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.chunk.x, self.chunk.z)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PlayerPos {
    pub fn new(x: i32, y: i32, z: i32) -> PlayerPos {
        PlayerPos { x, y, z }
    }
}

impl Display for PlayerPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
