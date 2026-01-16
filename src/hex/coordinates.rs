use bevy::prelude::*;

/// Axial hex coordinates (q, r)
/// Using flat-top hexagons with pointy sides on left/right
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Default)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub const ZERO: Self = Self { q: 0, r: 0 };

    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Get the cube coordinate S (q + r + s = 0)
    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    /// Convert axial coordinates to world position
    /// Uses flat-top hex orientation
    pub fn to_world(&self, hex_size: f32) -> Vec3 {
        let x = hex_size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let z = hex_size * (3.0 / 2.0 * self.r as f32);
        Vec3::new(x, 0.0, z)
    }

    /// Convert world position to axial coordinates
    pub fn from_world(pos: Vec3, hex_size: f32) -> Self {
        let q = (3.0_f32.sqrt() / 3.0 * pos.x - 1.0 / 3.0 * pos.z) / hex_size;
        let r = (2.0 / 3.0 * pos.z) / hex_size;
        Self::axial_round(q, r)
    }

    /// Round floating point axial coordinates to nearest hex
    fn axial_round(q: f32, r: f32) -> Self {
        let s = -q - r;

        let mut rq = q.round();
        let mut rr = r.round();
        let rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        }

        Self::new(rq as i32, rr as i32)
    }

    /// Get all 6 neighboring hex coordinates
    pub fn neighbors(&self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q + 1, self.r),
            HexCoord::new(self.q + 1, self.r - 1),
            HexCoord::new(self.q, self.r - 1),
            HexCoord::new(self.q - 1, self.r),
            HexCoord::new(self.q - 1, self.r + 1),
            HexCoord::new(self.q, self.r + 1),
        ]
    }

    /// Calculate distance between two hex coordinates
    pub fn distance(&self, other: &HexCoord) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = HexCoord::new(0, 0);
        let b = HexCoord::new(2, -1);
        assert_eq!(a.distance(&b), 2);
    }

    #[test]
    fn test_neighbors() {
        let center = HexCoord::ZERO;
        let neighbors = center.neighbors();
        assert_eq!(neighbors.len(), 6);
        for neighbor in &neighbors {
            assert_eq!(center.distance(neighbor), 1);
        }
    }
}
