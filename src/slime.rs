use crate::{
    random::JavaRandom,
    world::{Chunk, PlayerPos, Seed},
};

pub fn nearby_slimes<T: Into<Seed>>(seed: T, player_pos: PlayerPos, chunk_radius: u32) {
    let seed = seed.into();
    let radius = chunk_radius as i32;

    let chunk_x = player_pos.x / 16;
    let chunk_z = player_pos.z / 16;

    for x_offset in -radius..=radius {
        for z_offset in -radius..=radius {
            let x = chunk_x + x_offset;
            let z = chunk_z + z_offset;
            let chunk = Chunk::new(x, z);
            if is_slime_chunk(seed, chunk) {
                println!("Slime chunk starting from {}", chunk.display_world_coords());
            }
        }
    }
}

pub fn is_slime_chunk<T: Into<Seed>>(seed: T, chunk: Chunk) -> bool {
    let mut rnd = create_slime_random(seed.into(), chunk);
    rnd.next_i32_bound(10) == 0
}

fn create_slime_random(seed: Seed, chunk: Chunk) -> JavaRandom {
    let x = chunk.x;
    let z = chunk.z;

    JavaRandom::new(
        (seed.0
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
}
