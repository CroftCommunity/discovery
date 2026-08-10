//! Does Rapier's `enhanced-determinism` actually produce identical results on
//! `aarch64-apple-darwin` and `wasm32-unknown-unknown`?
//!
//! This is the decisive measurement behind the emoji-wars tier question: if the
//! two targets agree bit-for-bit, a physics game can carry a verifiable outcome
//! (`fun`'s Tier-1 bar) without a fixed-point rewrite. If they diverge, they
//! diverge, and we stop arguing about it.
//!
//! The scenario is deliberately emoji-wars-shaped rather than a toy: a launched
//! hero body strikes a stack of fragile pieces resting on an angled ramp, so the
//! run exercises restitution, friction, rotation, and a long contact-rich tail
//! where divergence compounds instead of cancelling.

use rapier2d::prelude::*;

/// Fixed timestep. A physics claim measured under a variable `requestAnimationFrame`
/// delta is not a physics claim; it is a claim about the frame scheduler.
pub const DT: f32 = 1.0 / 60.0;

/// Ten seconds of simulation. Long enough that any per-step divergence has room
/// to compound into the hash rather than staying under the rounding.
pub const STEPS: usize = 600;

/// FNV-1a (64-bit). Chosen because it is trivial to re-implement exactly, has no
/// dependency, and — unlike a `DefaultHasher` — is specified rather than
/// whatever the standard library happens to do this release.
struct Fnv(u64);

impl Fnv {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    /// Hash a float by its **bit pattern**, never by its decimal rendering.
    /// Formatting would silently paper over exactly the low-bit divergence this
    /// spike exists to detect.
    fn push_f32(&mut self, v: f32) {
        for byte in v.to_bits().to_le_bytes() {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }
}

/// Build the scene, step it, and fold every dynamic body's final pose into one
/// hash.
///
/// Body handles are kept in a `Vec` in insertion order and the hash walks *that*,
/// never the `RigidBodySet`'s own iteration order — so the digest cannot change
/// because a container reordered internally.
pub fn scenario_hash() -> u64 {
    scenario_hash_with(21.0).0
}

/// The scenario, parameterised by the hero's launch speed, returning both the
/// digest and the raw final poses.
///
/// The parameter exists for the sensitivity check: a digest that agrees across
/// two targets proves nothing unless a minimal change to the input provably
/// moves it. The poses come back so a run can be inspected rather than trusted.
pub fn scenario_hash_with(launch_dx: f32) -> (u64, Vec<f32>) {
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut dynamic: Vec<RigidBodyHandle> = Vec::new();

    // Ground: a wide static slab.
    let ground = bodies.insert(RigidBodyBuilder::fixed().translation(Vector::new(0.0, 0.0)));
    colliders.insert_with_parent(ColliderBuilder::cuboid(60.0, 0.5), ground, &mut bodies);

    // An angled ramp. Rotation is the reason this body is here: constructing the
    // isometry runs sin/cos through nalgebra/simba, which is precisely the path
    // `libm_force` is supposed to make platform-independent.
    let ramp = bodies.insert(
        RigidBodyBuilder::fixed()
            .translation(Vector::new(14.0, 3.0))
            .rotation(-0.42),
    );
    colliders.insert_with_parent(ColliderBuilder::cuboid(9.0, 0.4), ramp, &mut bodies);

    // The fragile structure: a 3-wide, 4-high stack of small boxes. Contact-rich
    // and unstable on purpose — a stack that merely settles would hide drift.
    for col in 0..3 {
        for row in 0..4 {
            let x = 10.0 + (col as f32) * 1.4;
            let y = 1.2 + (row as f32) * 1.1;
            let handle = bodies.insert(
                RigidBodyBuilder::dynamic()
                    .translation(Vector::new(x, y))
                    .rotation(0.03 * (row as f32) - 0.02 * (col as f32)),
            );
            colliders.insert_with_parent(
                ColliderBuilder::cuboid(0.6, 0.5)
                    .density(1.2)
                    .friction(0.55)
                    .restitution(0.12),
                handle,
                &mut bodies,
            );
            dynamic.push(handle);
        }
    }

    // The hero, launched. Angular velocity is set too so the spin couples into
    // every subsequent contact.
    let hero = bodies.insert(
        RigidBodyBuilder::dynamic()
            .translation(Vector::new(-6.0, 6.5))
            .linvel(Vector::new(launch_dx, 3.5))
            .angvel(-4.0),
    );
    colliders.insert_with_parent(
        ColliderBuilder::ball(0.55)
            .density(2.4)
            .friction(0.35)
            .restitution(0.15),
        hero,
        &mut bodies,
    );
    dynamic.push(hero);

    let params = IntegrationParameters {
        dt: DT,
        ..Default::default()
    };

    let mut pipeline = PhysicsPipeline::new();
    let mut islands = IslandManager::new();
    let mut broad = DefaultBroadPhase::new();
    let mut narrow = NarrowPhase::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();
    let mut ccd = CCDSolver::new();
    let gravity = Vector::new(0.0, -9.81);

    for _ in 0..STEPS {
        pipeline.step(
            gravity,
            &params,
            &mut islands,
            &mut broad,
            &mut narrow,
            &mut bodies,
            &mut colliders,
            &mut impulse_joints,
            &mut multibody_joints,
            &mut ccd,
            &(),
            &(),
        );
    }

    let mut hash = Fnv::new();
    let mut poses = Vec::with_capacity(dynamic.len() * 3);
    for handle in &dynamic {
        let body = &bodies[*handle];
        let t = body.translation();
        for v in [t.x, t.y, body.rotation().angle()] {
            hash.push_f32(v);
            poses.push(v);
        }
    }
    (hash.0, poses)
}

/// The wasm entry point. Returns i64 (JS sees a BigInt) so the digest crosses the
/// boundary without a float round-trip that could itself lose bits.
#[no_mangle]
pub extern "C" fn spike_hash() -> i64 {
    scenario_hash() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The golden vector, recorded from an actual run on `aarch64-apple-darwin`
    /// (rustc 1.97.1, rapier2d 0.35.1, `enhanced-determinism`). It was RED at 0
    /// first, so this value is measured rather than assumed.
    ///
    /// The wasm side is checked against this same constant by `verify.mjs`.
    const GOLDEN: u64 = 2_135_883_295_078_246_327;

    #[test]
    fn scenario_hash_matches_the_recorded_golden_vector() {
        assert_eq!(scenario_hash(), GOLDEN);
    }

    /// The check on the check. A cross-target digest match is only evidence if
    /// the digest would actually have moved had the physics differed — so
    /// perturb the launch by **one ULP** (the smallest representable change to
    /// an f32) and require a different digest.
    ///
    /// Without this, "the hashes matched" is indistinguishable from "the hash is
    /// insensitive to the simulation".
    #[test]
    fn a_one_ulp_change_to_the_launch_moves_the_digest() {
        let baseline = scenario_hash_with(21.0).0;
        let nudged = scenario_hash_with(f32::from_bits(21.0f32.to_bits() + 1)).0;
        assert_ne!(
            baseline, nudged,
            "digest did not respond to a 1-ULP input change; it is not measuring the sim"
        );
    }

    /// Guards the other way a match could be vacuous: if every body had come to
    /// rest at its spawn point, the digest would be stable and meaningless. The
    /// launched hero must have travelled, and the stack must have been disturbed.
    #[test]
    fn the_simulation_actually_does_work() {
        let (_, poses) = scenario_hash_with(21.0);
        assert_eq!(
            poses.len(),
            13 * 3,
            "12 stack boxes + 1 hero, 3 floats each"
        );

        // Hero is the last body; it starts at x = -6.0 and is launched +x.
        let hero_x = poses[poses.len() - 3];
        assert!(
            hero_x > 0.0,
            "hero never travelled (final x = {hero_x}); the launch did nothing"
        );

        // At least one stack box must have left its spawn column by a visible
        // margin, or nothing was actually knocked over.
        let spawn_xs: Vec<f32> = (0..3)
            .flat_map(|col| (0..4).map(move |_| 10.0 + (col as f32) * 1.4))
            .collect();
        let disturbed = spawn_xs
            .iter()
            .enumerate()
            .filter(|(i, sx)| (poses[i * 3] - **sx).abs() > 0.25)
            .count();
        assert!(
            disturbed > 0,
            "no stack piece moved more than 0.25; the scenario is a static pile"
        );
    }

    #[test]
    fn scenario_hash_is_stable_across_runs_in_one_process() {
        // Guards the boring failure that would masquerade as a platform
        // difference: leftover global state or an unstable iteration order
        // making the digest vary even on one machine.
        assert_eq!(scenario_hash(), scenario_hash());
    }
}
