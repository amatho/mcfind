use crate::{
    random::JavaRandom,
    world::{Chunk, PlayerPos},
};

pub fn nearby_slimes(seed: i64, player_pos: PlayerPos, chunk_radius: u32) -> Vec<Chunk> {
    let radius = chunk_radius as i32;

    let chunk_x = player_pos.x / 16;
    let chunk_z = player_pos.z / 16;

    let mut slime_chunks = Vec::new();
    for z_offset in -radius..=radius {
        for x_offset in -radius..=radius {
            let x = chunk_x + x_offset;
            let z = chunk_z + z_offset;
            let chunk = Chunk::new(x, z);
            if is_slime_chunk(seed, chunk) {
                slime_chunks.push(chunk);
            }
        }
    }

    slime_chunks
}

pub fn is_slime_chunk(seed: i64, chunk: Chunk) -> bool {
    let mut rnd = create_slime_random(seed, chunk);
    rnd.next_i32_bound(10) == 0
}

fn create_slime_random(seed: i64, chunk: Chunk) -> JavaRandom {
    let x = chunk.x;
    let z = chunk.z;

    JavaRandom::new(
        (seed
            + x.wrapping_mul(x).wrapping_mul(0x4c1906) as i64
            + x.wrapping_mul(0x5ac0db) as i64
            + z.wrapping_mul(z).wrapping_mul(0x4307a7) as i64
            + z.wrapping_mul(0x5f24f) as i64)
            ^ 0x3ad8025f,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_slime_chunk_parity() {
        let seed = 12345i64;

        assert!(is_slime_chunk(seed, Chunk::new(3, 0)));
        assert!(is_slime_chunk(seed, Chunk::new(0, -2)));
        assert!(is_slime_chunk(seed, Chunk::new(-2, 1)));
        assert!(is_slime_chunk(seed, Chunk::new(-2, -4)));
        assert!(!is_slime_chunk(seed, Chunk::new(2, -4)));
    }

    #[test]
    fn test_nearby_slimes() {
        let seed = 12345;
        let player_pos = PlayerPos::new(40, 0, 8);
        let expected: Vec<Chunk> = [(0, -2), (3, 0), (4, 1)]
            .iter()
            .map(|&(x, z)| Chunk::new(x, z))
            .collect();

        assert_eq!(nearby_slimes(seed, player_pos, 2), expected);
    }
}
