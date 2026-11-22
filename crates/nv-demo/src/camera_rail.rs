//! Multi-cassette cinematic camera rail (periodic Centripetal Catmull-Rom spline with
//! arc-length parameterization and curvature-driven FPV banking), gimbal damper, and
//! 180-degree shutter subframe timing helpers.
//!
//! What: provides [`RailCassette`] (5 curated collision-free Gundo cinema tracks plus an
//! independent `Custom` user-keyframe deck), [`CameraRail`] with C1-continuous closed loops and
//! fixed-size zero-allocation arc-length LUT, [`CameraDamper`] for exponential camera smoothing,
//! and [`shutter_subframe_dt`] for exact 180-degree shutter motion-blur subframe pacing.
//! Why: keeps all cinematic camera, arc-length reparameterization, and fixed-step timing math
//! deterministic, allocation-free on the frame loop, and unit-testable against real `csg-urban`
//! Gundo geometry without requiring a GPU.
//! Used by: `crate::main`, `crate::ui`, `crate::persist`.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::camera_core::CameraPose;

/// Fixed sample count for the inline zero-allocation arc-length lookup table.
const ARC_LUT_SAMPLES: usize = 256;

/// Selectable camera rail cassette (5 built-in Gundo tracks + user `Custom` keyframes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RailCassette {
    /// Silky Vista Del Mar coastal highway & oceanfront seawall cruise (~18s).
    #[default]
    CoastalCruise,
    /// Thrilling banked FPV drone slalom weaving past Hortonspheres, towers & under pipe racks (~24s).
    RefinerySlalom,
    /// High-adrenaline run down the Marina/El Segundo mid-rise canyon alley & over rooftops (~22s).
    UrbanCanyon,
    /// Low bumper-height banked S-curves along the wet promenade, seawall hairpin & frontage (~16s).
    MidnightDrift,
    /// Epic full-length showcase linking oceanfront, highway, refinery, urban canyon & aerial vista (~38s).
    GrandTour,
    /// User-authored custom keyframe spline (`+ KF` / `Pop KF` / `Clear`), preserved independently.
    Custom,
}

impl RailCassette {
    /// Ordered list of all selectable cassettes for UI decks and automated audits.
    pub const ALL: &'static [Self] = &[
        Self::CoastalCruise,
        Self::RefinerySlalom,
        Self::UrbanCanyon,
        Self::MidnightDrift,
        Self::GrandTour,
        Self::Custom,
    ];

    /// Built-in Gundo showcase cassettes (excluding `Custom`).
    #[cfg(test)]
    pub const BUILTINS: &'static [Self] = &[
        Self::CoastalCruise,
        Self::RefinerySlalom,
        Self::UrbanCanyon,
        Self::MidnightDrift,
        Self::GrandTour,
    ];

    /// Returns `true` for the curated built-in Gundo cassettes.
    #[inline]
    pub const fn is_builtin(self) -> bool {
        !matches!(self, Self::Custom)
    }

    /// Human-readable cassette title for UI dropdowns and HUD labels.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::CoastalCruise => "Coastal Cruise (18s)",
            Self::RefinerySlalom => "Refinery Slalom (24s)",
            Self::UrbanCanyon => "Urban Canyon (22s)",
            Self::MidnightDrift => "Midnight Drift (16s)",
            Self::GrandTour => "Grand Tour (38s)",
            Self::Custom => "Custom Keyframes",
        }
    }

    /// Concise tooltip description for the cassette selector.
    #[inline]
    pub const fn description(self) -> &'static str {
        match self {
            Self::CoastalCruise => {
                "Silky highway & oceanfront promenade cruise past moving traffic, lanterns, and the refinery skyline."
            }
            Self::RefinerySlalom => {
                "Thrilling banked FPV weave through Hortonspheres, distillation towers, tank farm canyon, and under overhead pipe racks."
            }
            Self::UrbanCanyon => {
                "High-speed run threading between Marina/El Segundo mid-rises & lit storefronts before climbing over rooftops for an ocean reveal."
            }
            Self::MidnightDrift => {
                "Low bumper-height banked S-curves slicing along the wet coastal promenade, seawall hairpin, and refinery frontage."
            }
            Self::GrandTour => {
                "Epic 38-second full-world showcase seamlessly linking oceanfront, highway, refinery weave, urban canyon, and high coastal vista."
            }
            Self::Custom => {
                "Plays your own custom camera keyframes (+ KF / Pop KF / Clear) independently of built-in cassettes."
            }
        }
    }

    /// Default loop duration in seconds tuned to each cassette's length and pacing.
    #[inline]
    pub const fn default_duration_s(self) -> f32 {
        match self {
            Self::CoastalCruise => 18.0,
            Self::RefinerySlalom => 24.0,
            Self::UrbanCanyon => 22.0,
            Self::MidnightDrift => 16.0,
            Self::GrandTour => 38.0,
            Self::Custom => 16.0,
        }
    }

    /// Maximum curvature-driven banking roll angle in degrees for this cassette.
    #[inline]
    const fn max_bank_deg(self) -> f32 {
        match self {
            Self::CoastalCruise => 10.0,
            Self::RefinerySlalom => 26.0,
            Self::UrbanCanyon => 22.0,
            Self::MidnightDrift => 18.0,
            Self::GrandTour => 18.0,
            Self::Custom => 0.0,
        }
    }

    /// Curvature-to-bank gain factor controlling how aggressively turns lean into corners.
    #[inline]
    const fn bank_gain(self) -> f32 {
        match self {
            Self::CoastalCruise => 22.0,
            Self::RefinerySlalom => 34.0,
            Self::UrbanCanyon => 30.0,
            Self::MidnightDrift => 28.0,
            Self::GrandTour => 26.0,
            Self::Custom => 0.0,
        }
    }
}

/// Curated look-at spline waypoint for built-in Gundo cassettes.
#[derive(Clone, Copy, Debug)]
struct RailWaypoint {
    eye: [f32; 3],
    target: [f32; 3],
    vfov_deg: f32,
}

impl RailWaypoint {
    const fn new(eye: [f32; 3], target: [f32; 3], vfov_deg: f32) -> Self {
        Self {
            eye,
            target,
            vfov_deg,
        }
    }
}

const COASTAL_CRUISE_WAYPOINTS: &[RailWaypoint] = &[
    RailWaypoint::new([-15.4, 5.0, -130.0], [14.0, 5.5, -35.0], 50.0),
    RailWaypoint::new([-15.4, 4.0, 0.0], [24.0, 8.5, 95.0], 50.0),
    RailWaypoint::new([-15.4, 5.6, 125.0], [32.0, 12.0, 190.0], 50.0),
    // Wide 2-waypoint north turnaround arc over the surf (smooth radius, uniform arc-length speed).
    RailWaypoint::new([-23.0, 10.5, 195.0], [26.0, 12.0, 130.0], 48.0),
    RailWaypoint::new([-42.0, 15.5, 192.0], [24.0, 11.5, 80.0], 48.0),
    RailWaypoint::new([-48.0, 17.5, 70.0], [24.0, 11.0, -20.0], 48.0),
    RailWaypoint::new([-48.0, 17.0, -65.0], [22.0, 9.5, -145.0], 48.0),
    // Wide 2-waypoint south turnaround arc clearing promenade lanterns (`y >= 6.8m` at `x = -18.65`).
    RailWaypoint::new([-42.0, 13.5, -185.0], [14.0, 6.5, -210.0], 48.0),
    RailWaypoint::new([-23.0, 8.8, -192.0], [6.0, 4.5, -120.0], 50.0),
];

const REFINERY_SLALOM_WAYPOINTS: &[RailWaypoint] = &[
    // Frontage Avenue sprint beside Hortonspheres (`x = 21.5` clears highway fence `14.55` & spheres `28.95`).
    RailWaypoint::new([21.5, 6.2, -140.0], [38.0, 9.0, -55.0], 54.0),
    RailWaypoint::new([21.5, 9.8, -50.0], [56.0, 12.5, 5.0], 56.0),
    // Sweep through the wide `z = 0` aisle (30.8m from every Hortonsphere & tower) above longitudinal rack.
    RailWaypoint::new([58.8, 13.4, -2.0], [93.35, 11.5, 35.0], 58.0),
    // Dive into Process Alley (`x = 93.35` = dead center of the 6m cross-yard pipe rack portal bay `90.35..96.35`).
    RailWaypoint::new([93.35, 11.6, 22.0], [93.35, 4.2, 85.0], 60.0),
    // Thrilling fly-UNDER straight through the middle of the `rz = 70.4` overhead pipe rack portal bay!
    RailWaypoint::new([93.35, 3.4, 70.4], [93.35, 6.5, 140.0], 62.0),
    // Soar back up over the `rz = 140.8` pipe rack toward the tank farm & flare stacks.
    RailWaypoint::new([93.35, 13.8, 145.0], [126.0, 18.0, 200.0], 58.0),
    // Wide 2-waypoint north turn into the Tank Farm Canyon (`x = 130.3`, above tank rims & lamps).
    RailWaypoint::new([106.0, 18.2, 202.0], [142.0, 22.0, 155.0], 56.0),
    RailWaypoint::new([130.3, 21.5, 182.0], [155.0, 26.0, 85.0], 56.0),
    RailWaypoint::new([130.3, 24.0, 40.0], [75.0, 18.0, -65.0], 54.0),
    // Wide high south return over tanks & through the `z = -184.8` open aisle (above all Hortonspheres `<=16.3m`).
    RailWaypoint::new([122.0, 23.5, -135.0], [60.0, 16.0, -185.0], 54.0),
    RailWaypoint::new([62.0, 21.5, -188.0], [26.0, 12.0, -205.0], 54.0),
    RailWaypoint::new([25.0, 17.5, -198.0], [21.5, 7.5, -135.0], 54.0),
];

const URBAN_CANYON_WAYPOINTS: &[RailWaypoint] = &[
    // Approach North Urban Wing along the open plaza south of `z = 259.84` and align onto `x = 37.0`.
    RailWaypoint::new([24.0, 8.0, 235.0], [37.0, 5.5, 300.0], 56.0),
    // Thread straight down the 5m-wide Inner Urban Canyon (`x = 37.0`, between Row 0 `33.4` & Row 1 lanterns `40.85`).
    RailWaypoint::new([37.0, 5.2, 265.0], [37.0, 5.8, 345.0], 58.0),
    RailWaypoint::new([37.0, 4.8, 338.0], [37.0, 7.5, 415.0], 60.0),
    RailWaypoint::new([37.0, 7.8, 412.0], [37.0, 18.0, 452.0], 58.0),
    // Wide 2-waypoint banked rooftop turnaround north of `z = 430` before soaring over rooftops (`y = 35m > 29m`).
    RailWaypoint::new([37.0, 17.5, 448.0], [18.0, 32.0, 440.0], 54.0),
    RailWaypoint::new([18.0, 34.5, 452.0], [-18.0, 26.0, 375.0], 52.0),
    RailWaypoint::new([-16.0, 32.0, 410.0], [-22.0, 14.0, 315.0], 52.0),
    RailWaypoint::new([-28.0, 24.0, 335.0], [22.0, 12.0, 265.0], 50.0),
    // Swoop south over the beachfront & cross highway at `y = 12.4` (clears all lamps & signals).
    RailWaypoint::new([-28.0, 15.5, 255.0], [14.0, 10.0, 215.0], 52.0),
    RailWaypoint::new([-4.0, 12.4, 214.0], [26.0, 8.0, 250.0], 54.0),
];

const MIDNIGHT_DRIFT_WAYPOINTS: &[RailWaypoint] = &[
    // Low bumper-height S-curves slicing along the open beachfront beside the seawall (`x <= -22.8`).
    RailWaypoint::new([-23.5, 2.1, -130.0], [-8.0, 2.6, -30.0], 56.0),
    RailWaypoint::new([-29.5, 2.5, -35.0], [-6.0, 3.2, 65.0], 58.0),
    RailWaypoint::new([-22.8, 2.0, 65.0], [6.0, 3.6, 155.0], 58.0),
    // Climb cleanly over the north seawall & highway (`y = 12.8 > 9.6m` median lamps).
    RailWaypoint::new([-21.0, 12.0, 160.0], [12.0, 10.5, 190.0], 54.0),
    RailWaypoint::new([0.0, 12.8, 192.0], [24.0, 8.5, 130.0], 54.0),
    RailWaypoint::new([22.0, 11.8, 158.0], [22.0, 3.5, 70.0], 56.0),
    // Razor-low drift S-curves southbound along the 9.5m-wide Frontage Avenue (`x in [18.8, 25.2]`).
    RailWaypoint::new([18.8, 2.2, 65.0], [24.0, 3.2, -30.0], 58.0),
    RailWaypoint::new([25.2, 2.3, -35.0], [18.0, 3.5, -130.0], 58.0),
    // Wide rounded south hairpin back over the highway (`y = 12.8`).
    RailWaypoint::new([21.5, 11.8, -155.0], [-10.0, 10.5, -190.0], 54.0),
    RailWaypoint::new([0.0, 12.8, -190.0], [-24.0, 6.5, -130.0], 54.0),
];

const GRAND_TOUR_WAYPOINTS: &[RailWaypoint] = &[
    // 1. Vista Del Mar promenade northbound past moving traffic & lanterns.
    RailWaypoint::new([-15.4, 5.2, -155.0], [16.0, 7.0, -45.0], 52.0),
    RailWaypoint::new([-15.4, 5.8, -35.0], [32.0, 11.0, 40.0], 52.0),
    // 2. Clean crossing over highway (`y = 12.4`) right into the wide `z = 0` refinery aisle.
    RailWaypoint::new([21.5, 12.4, 2.0], [60.0, 13.5, 25.0], 54.0),
    RailWaypoint::new([62.0, 13.6, 5.0], [93.35, 14.0, 80.0], 56.0),
    RailWaypoint::new([93.35, 14.0, 110.0], [125.0, 18.0, 195.0], 56.0),
    // Cross west along the wide open plaza (`z = 238..246`, clear of `z = 215.6` towers & `z >= 264` buildings).
    RailWaypoint::new([93.35, 16.5, 238.0], [50.0, 13.0, 250.0], 54.0),
    RailWaypoint::new([44.0, 13.5, 246.0], [37.2, 7.5, 315.0], 56.0),
    // 3. Plunge dead-center down the North Marina/El Segundo Urban Canyon alley (`x = 37.2`).
    RailWaypoint::new([37.2, 7.4, 276.0], [37.2, 7.8, 355.0], 58.0),
    RailWaypoint::new([37.2, 7.6, 342.0], [37.2, 8.2, 415.0], 58.0),
    RailWaypoint::new([37.2, 8.2, 405.0], [37.2, 18.0, 452.0], 56.0),
    // 4. Wide 2-waypoint rooftop turnaround (`y = 35.5m`) into a sweeping high Pacific panorama.
    RailWaypoint::new([37.0, 24.0, 452.0], [-8.0, 32.0, 430.0], 52.0),
    RailWaypoint::new([-10.0, 35.5, 444.0], [-36.0, 24.0, 320.0], 50.0),
    RailWaypoint::new([-40.0, 31.0, 340.0], [40.0, 14.0, 130.0], 48.0),
    RailWaypoint::new([-46.0, 26.0, 140.0], [38.0, 12.0, -30.0], 48.0),
    RailWaypoint::new([-48.0, 21.0, -45.0], [28.0, 9.0, -165.0], 48.0),
    // 5. Wide 2-waypoint Pacific south turnaround arc seamlessly rejoining the promenade.
    RailWaypoint::new([-42.0, 13.5, -225.0], [14.0, 7.0, -215.0], 48.0),
    RailWaypoint::new([-20.0, 8.6, -230.0], [6.0, 5.0, -150.0], 50.0),
];

#[inline]
const fn builtin_waypoints(cassette: RailCassette) -> &'static [RailWaypoint] {
    match cassette {
        RailCassette::CoastalCruise => COASTAL_CRUISE_WAYPOINTS,
        RailCassette::RefinerySlalom => REFINERY_SLALOM_WAYPOINTS,
        RailCassette::UrbanCanyon => URBAN_CANYON_WAYPOINTS,
        RailCassette::MidnightDrift => MIDNIGHT_DRIFT_WAYPOINTS,
        RailCassette::GrandTour => GRAND_TOUR_WAYPOINTS,
        RailCassette::Custom => &[],
    }
}

/// Single camera pose keyframe along a `Custom` [`CameraRail`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraKeyframe {
    /// World-space camera position `[x, y, z]`.
    pub pos: [f32; 3],
    /// Unit orientation quaternion `[x, y, z, w]` (camera-to-world).
    pub orientation: [f32; 4],
    /// Vertical field of view in degrees.
    pub vfov_deg: f32,
}

impl CameraKeyframe {
    /// Captures a keyframe from the current [`CameraPose`].
    pub fn from_pose(pose: CameraPose) -> Self {
        let q = if pose.orientation.is_finite() && pose.orientation.length_squared() > 1e-6 {
            pose.orientation.normalize()
        } else {
            Quat::IDENTITY
        };
        Self {
            pos: pose.eye.to_array(),
            orientation: q.to_array(),
            vfov_deg: pose.fov_y.to_degrees().clamp(10.0, 120.0),
        }
    }

    /// Constructs a keyframe at `eye` aiming at `target` with world `+Y` up.
    #[cfg(test)]
    pub fn from_look_at(eye: [f32; 3], target: [f32; 3], vfov_deg: f32) -> Self {
        let eye_v = Vec3::from_array(eye);
        let tgt_v = Vec3::from_array(target);
        let delta = tgt_v - eye_v;
        let fwd = if delta.length_squared() > 1e-8 {
            delta.normalize()
        } else {
            Vec3::NEG_Z
        };
        let pose = CameraPose::from_eye_forward_up(
            eye_v,
            fwd,
            Vec3::Y,
            vfov_deg.clamp(10.0, 120.0).to_radians(),
            0.05,
            2000.0,
        );
        Self::from_pose(pose)
    }

    /// Converts this keyframe back into a validated [`CameraPose`].
    pub fn to_pose(self, znear: f32, zfar: f32) -> CameraPose {
        let eye = Vec3::from_array(self.pos);
        let q_raw = Quat::from_array(self.orientation);
        let orientation = if q_raw.is_finite() && q_raw.length_squared() > 1e-6 {
            q_raw.normalize()
        } else {
            Quat::IDENTITY
        };
        CameraPose {
            eye: if eye.is_finite() { eye } else { Vec3::ZERO },
            orientation,
            fov_y: self.vfov_deg.clamp(10.0, 120.0).to_radians(),
            znear: znear.max(1e-3),
            zfar: zfar.max(znear + 1.0),
        }
    }

    #[inline]
    fn quat(self) -> Quat {
        let q = Quat::from_array(self.orientation);
        if q.is_finite() && q.length_squared() > 1e-6 {
            q.normalize()
        } else {
            Quat::IDENTITY
        }
    }
}

/// Smoothstep ease-in/out curve `3u^2 - 2u^3` on `[0, 1]`.
#[inline]
pub fn smoothstep01(u: f32) -> f32 {
    let t = if u.is_finite() {
        u.clamp(0.0, 1.0)
    } else {
        0.0
    };
    t * t * (3.0 - 2.0 * t)
}

/// Evaluates a Centripetal Catmull-Rom spline (`alpha = 0.5`) between `p1` and `p2` for `s in [0, 1]`.
fn centripetal_catmull_rom(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, s: f32) -> Vec3 {
    let s = s.clamp(0.0, 1.0);
    let knot = |a: Vec3, b: Vec3| -> f32 { a.distance(b).sqrt().max(1.0e-3) };
    let t0 = 0.0f32;
    let t1 = t0 + knot(p0, p1);
    let t2 = t1 + knot(p1, p2);
    let t3 = t2 + knot(p2, p3);
    let t = t1 + (t2 - t1) * s;

    let lerp_t = |va: Vec3, vb: Vec3, ta: f32, tb: f32| -> Vec3 {
        let denom = tb - ta;
        if denom.abs() < 1.0e-7 {
            va
        } else {
            va * ((tb - t) / denom) + vb * ((t - ta) / denom)
        }
    };

    let a1 = lerp_t(p0, p1, t0, t1);
    let a2 = lerp_t(p1, p2, t1, t2);
    let a3 = lerp_t(p2, p3, t2, t3);
    let b1 = lerp_t(a1, a2, t0, t2);
    let b2 = lerp_t(a2, a3, t1, t3);
    lerp_t(b1, b2, t1, t2)
}

/// Shortest-path quaternion spherical linear interpolation (never takes the >180° long arc).
#[inline]
fn shortest_slerp(a: Quat, b: Quat, t: f32) -> Quat {
    let b_aligned = if a.dot(b) < 0.0 { -b } else { b };
    a.slerp(b_aligned, t.clamp(0.0, 1.0)).normalize()
}

/// Logarithm of a unit quaternion (`0.5 * angle * axis`) for Shoemake SQUAD tangent calculation.
#[inline]
fn quat_log_vec(q: Quat) -> Vec3 {
    let q = if q.w < 0.0 { -q } else { q };
    let v = Vec3::new(q.x, q.y, q.z);
    let sin_half = v.length();
    if sin_half < 1.0e-6 {
        Vec3::ZERO
    } else {
        let half_angle = sin_half.atan2(q.w.clamp(-1.0, 1.0));
        v * (half_angle / sin_half)
    }
}

/// Exponential of a pure-imaginary quaternion vector `v` (`exp(0, v)`).
#[inline]
fn quat_exp_vec(v: Vec3) -> Quat {
    let theta = v.length();
    if theta < 1.0e-6 {
        Quat::IDENTITY
    } else {
        let (s, c) = theta.sin_cos();
        let axis = v / theta;
        Quat::from_xyzw(axis.x * s, axis.y * s, axis.z * s, c).normalize()
    }
}

/// Shoemake inner quadrangle control point `s_i` for C1-continuous spherical cubic interpolation.
#[inline]
fn squad_tangent(q_prev: Quat, q_cur: Quat, q_next: Quat) -> Quat {
    let q_prev = if q_cur.dot(q_prev) < 0.0 { -q_prev } else { q_prev };
    let q_next = if q_cur.dot(q_next) < 0.0 { -q_next } else { q_next };
    let inv = q_cur.conjugate();
    let log_next = quat_log_vec(inv * q_next);
    let log_prev = quat_log_vec(inv * q_prev);
    (q_cur * quat_exp_vec(-0.25 * (log_next + log_prev))).normalize()
}

/// C1-continuous spherical cubic interpolation (SQUAD) across `(q0, q1, q2, q3)` for `s in [0, 1]`.
fn squad_interpolate(q0: Quat, q1: Quat, q2: Quat, q3: Quat, s: f32) -> Quat {
    let s = s.clamp(0.0, 1.0);
    let q1 = q1.normalize();
    let q0 = if q1.dot(q0) < 0.0 { -q0 } else { q0 };
    let q2 = if q1.dot(q2) < 0.0 { -q2 } else { q2 };
    let q3 = if q2.dot(q3) < 0.0 { -q3 } else { q3 };
    let s1 = squad_tangent(q0, q1, q2);
    let s2 = squad_tangent(q1, q2, q3);
    let slerp_12 = shortest_slerp(q1, q2, s);
    let slerp_s = shortest_slerp(s1, s2, s);
    shortest_slerp(slerp_12, slerp_s, 2.0 * s * (1.0 - s))
}

/// Resolved spline segment indices and local parameter `local_s in [0, 1]`.
struct SplineSeg {
    seg_idx: usize,
    seg_count: usize,
    i0: usize,
    i1: usize,
    i2: usize,
    i3: usize,
    local_s: f32,
}

/// Computes the 4 segment indices `(i0, i1, i2, i3)` and local parameter `local_s` for spline `u in [0, 1]`.
#[inline]
fn spline_segment(n: usize, u: f32, looping: bool) -> SplineSeg {
    let seg_count = if looping { n } else { n - 1 };
    let u_wrapped = if looping {
        u.rem_euclid(1.0)
    } else {
        u.clamp(0.0, 1.0)
    };
    let scaled = u_wrapped * (seg_count as f32);
    let seg_idx = (scaled.floor() as usize).min(seg_count - 1);
    let local_s = (scaled - seg_idx as f32).clamp(0.0, 1.0);
    let (i0, i1, i2, i3) = if looping {
        (
            (seg_idx + n - 1) % n,
            seg_idx % n,
            (seg_idx + 1) % n,
            (seg_idx + 2) % n,
        )
    } else {
        (
            seg_idx.saturating_sub(1),
            seg_idx,
            (seg_idx + 1).min(n - 1),
            (seg_idx + 2).min(n - 1),
        )
    };
    SplineSeg {
        seg_idx,
        seg_count,
        i0,
        i1,
        i2,
        i3,
        local_s,
    }
}

/// Evaluates raw (pre-arc-length) eye position on a slice of `RailWaypoint`s.
fn eval_builtin_eye_raw(wps: &[RailWaypoint], u: f32, looping: bool) -> Vec3 {
    let n = wps.len();
    if n == 0 {
        return Vec3::ZERO;
    }
    if n == 1 {
        return Vec3::from_array(wps[0].eye);
    }
    let seg = spline_segment(n, u, looping);
    let p1 = Vec3::from_array(wps[seg.i1].eye);
    let p2 = Vec3::from_array(wps[seg.i2].eye);
    let p0 = if !looping && seg.seg_idx == 0 {
        p1 * 2.0 - p2
    } else {
        Vec3::from_array(wps[seg.i0].eye)
    };
    let p3 = if !looping && seg.seg_idx + 1 == seg.seg_count {
        p2 * 2.0 - p1
    } else {
        Vec3::from_array(wps[seg.i3].eye)
    };
    centripetal_catmull_rom(p0, p1, p2, p3, seg.local_s)
}

/// Evaluates raw (pre-arc-length) look target & vertical FoV on a slice of `RailWaypoint`s.
fn eval_builtin_target_fov_raw(wps: &[RailWaypoint], u: f32, looping: bool) -> (Vec3, f32) {
    let n = wps.len();
    if n == 0 {
        return (Vec3::NEG_Z, 50.0);
    }
    if n == 1 {
        return (Vec3::from_array(wps[0].target), wps[0].vfov_deg);
    }
    let seg = spline_segment(n, u, looping);
    let t1 = Vec3::from_array(wps[seg.i1].target);
    let t2 = Vec3::from_array(wps[seg.i2].target);
    let t0 = if !looping && seg.seg_idx == 0 {
        t1 * 2.0 - t2
    } else {
        Vec3::from_array(wps[seg.i0].target)
    };
    let t3 = if !looping && seg.seg_idx + 1 == seg.seg_count {
        t2 * 2.0 - t1
    } else {
        Vec3::from_array(wps[seg.i3].target)
    };
    let target = centripetal_catmull_rom(t0, t1, t2, t3, seg.local_s);
    // Smooth Catmull-Rom scalar blend on FoV via the same spline weights (clamped safely).
    let f1 = wps[seg.i1].vfov_deg;
    let f2 = wps[seg.i2].vfov_deg;
    let f0 = if !looping && seg.seg_idx == 0 {
        f1 * 2.0 - f2
    } else {
        wps[seg.i0].vfov_deg
    };
    let f3 = if !looping && seg.seg_idx + 1 == seg.seg_count {
        f2 * 2.0 - f1
    } else {
        wps[seg.i3].vfov_deg
    };
    let s = seg.local_s;
    let s2 = s * s;
    let s3 = s2 * s;
    let vfov = 0.5
        * ((2.0 * f1)
            + (-f0 + f2) * s
            + (2.0 * f0 - 5.0 * f1 + 4.0 * f2 - f3) * s2
            + (-f0 + 3.0 * f1 - 3.0 * f2 + f3) * s3);
    (target, vfov.clamp(15.0, 110.0))
}

/// Evaluates raw (pre-arc-length) eye position on `Custom` keyframes.
fn eval_custom_eye_raw(kfs: &[CameraKeyframe], u: f32, looping: bool) -> Vec3 {
    let n = kfs.len();
    if n == 0 {
        return Vec3::ZERO;
    }
    if n == 1 {
        return Vec3::from_array(kfs[0].pos);
    }
    let seg = spline_segment(n, u, looping);
    let p1 = Vec3::from_array(kfs[seg.i1].pos);
    let p2 = Vec3::from_array(kfs[seg.i2].pos);
    let p0 = if !looping && seg.seg_idx == 0 {
        p1 * 2.0 - p2
    } else {
        Vec3::from_array(kfs[seg.i0].pos)
    };
    let p3 = if !looping && seg.seg_idx + 1 == seg.seg_count {
        p2 * 2.0 - p1
    } else {
        Vec3::from_array(kfs[seg.i3].pos)
    };
    centripetal_catmull_rom(p0, p1, p2, p3, seg.local_s)
}

/// Compact inline fixed-size arc-length lookup table (zero heap allocations).
#[derive(Clone, Copy, Debug)]
struct ArcLengthLut {
    signature: u64,
    cum_len: [f32; ARC_LUT_SAMPLES + 1],
    total_len: f32,
}

impl Default for ArcLengthLut {
    fn default() -> Self {
        Self {
            signature: 0,
            cum_len: [0.0; ARC_LUT_SAMPLES + 1],
            total_len: 0.0,
        }
    }
}

/// Cached LUT always compares equal so `PersistedState` dirty tracking ignores runtime caches.
impl PartialEq for ArcLengthLut {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl ArcLengthLut {
    fn build<F: Fn(f32) -> Vec3>(signature: u64, eval_pos: F) -> Self {
        let mut cum_len = [0.0f32; ARC_LUT_SAMPLES + 1];
        let mut prev = eval_pos(0.0);
        let mut acc = 0.0f32;
        let inv_n = 1.0 / (ARC_LUT_SAMPLES as f32);
        for (i, slot) in cum_len.iter_mut().enumerate().skip(1) {
            let u = (i as f32) * inv_n;
            let cur = eval_pos(u);
            acc += prev.distance(cur);
            *slot = acc;
            prev = cur;
        }
        Self {
            signature,
            cum_len,
            total_len: acc,
        }
    }

    /// Maps normalized arc-length fraction `frac in [0, 1]` to raw spline parameter `u in [0, 1]`.
    fn u_for_fraction(&self, frac: f32) -> f32 {
        let f = frac.clamp(0.0, 1.0);
        if self.total_len <= 1.0e-5 {
            return f;
        }
        if f <= 0.0 {
            return 0.0;
        }
        if f >= 1.0 {
            return 1.0;
        }
        let target_len = f * self.total_len;
        let idx = self
            .cum_len
            .partition_point(|&len| len < target_len)
            .clamp(1, ARC_LUT_SAMPLES);
        let l0 = self.cum_len[idx - 1];
        let l1 = self.cum_len[idx];
        let seg_t = if (l1 - l0).abs() > 1.0e-7 {
            ((target_len - l0) / (l1 - l0)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        (((idx - 1) as f32) + seg_t) / (ARC_LUT_SAMPLES as f32)
    }
}

/// Fast FNV-1a signature over `(cassette, looping, custom_keyframes)` to detect stale arc-length LUTs.
fn rail_lut_signature(cassette: RailCassette, looping: bool, keyframes: &[CameraKeyframe]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mix = |h: &mut u64, v: u64| {
        *h ^= v;
        *h = h.wrapping_mul(0x0100_0000_01b3);
    };
    mix(&mut h, (cassette as u64).wrapping_add(1));
    mix(&mut h, u64::from(looping));
    if cassette == RailCassette::Custom {
        mix(&mut h, keyframes.len() as u64);
        for kf in keyframes {
            for &c in &kf.pos {
                mix(&mut h, u64::from(c.to_bits()));
            }
        }
    }
    h.max(1)
}

/// Multi-cassette serializable camera rail with toggleable Auto-Drive, C1 periodic Catmull-Rom
/// loops, zero-allocation arc-length speed equalization, and smooth curvature banking.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraRail {
    /// Selected track cassette (5 built-in Gundo tracks or `Custom`).
    pub cassette: RailCassette,
    /// User-authored `Custom` camera keyframes (never wiped when switching built-in cassettes).
    pub keyframes: Vec<CameraKeyframe>,
    /// Total duration of one full rail pass in seconds.
    pub duration_s: f32,
    /// Loop continuously with C1 periodic wrap when reaching the end of the rail.
    pub looping: bool,
    /// Apply smoothstep ease-in / ease-out over the rail parameter (typically off on closed loops).
    pub ease_io: bool,
    /// Runtime Auto-Drive ON/OFF state (not persisted as auto-playing across restarts).
    #[serde(skip)]
    pub playing: bool,
    /// Current playback cursor in `[0, duration_s]`.
    #[serde(skip)]
    pub time_s: f32,
    /// Inline fixed-size arc-length lookup table rebuilt only on track change.
    #[serde(skip)]
    arc_lut: ArcLengthLut,
}

impl Default for CameraRail {
    fn default() -> Self {
        let mut rail = Self {
            cassette: RailCassette::CoastalCruise,
            keyframes: Vec::new(),
            duration_s: RailCassette::CoastalCruise.default_duration_s(),
            looping: true,
            ease_io: false,
            playing: false,
            time_s: 0.0,
            arc_lut: ArcLengthLut::default(),
        };
        rail.sync_lut();
        rail
    }
}

impl CameraRail {
    /// Returns `true` if the currently selected cassette has enough waypoints (`>= 2`) to play.
    #[inline]
    pub fn can_play(&self) -> bool {
        self.active_waypoint_count() >= 2
    }

    /// Number of waypoints on the active cassette (`builtin` length or `Custom` `keyframes.len()`).
    #[inline]
    pub fn active_waypoint_count(&self) -> usize {
        if self.cassette.is_builtin() {
            builtin_waypoints(self.cassette).len()
        } else {
            self.keyframes.len()
        }
    }

    /// Returns `true` when Auto-Drive playback is actively driving the camera.
    #[inline]
    pub fn is_active(&self) -> bool {
        self.playing && self.can_play()
    }

    /// Switches to `cassette` without erasing `self.keyframes` (the user's `Custom` track).
    ///
    /// Built-in cassettes adopt their tuned duration and continuous loop defaults (`looping = true`,
    /// `ease_io = false`). Switching to `Custom` preserves the user's keyframes and pauses if fewer
    /// than 2 custom keyframes exist.
    pub fn select_cassette(&mut self, cassette: RailCassette) {
        if self.cassette == cassette {
            self.sync_lut();
            return;
        }
        self.cassette = cassette;
        self.time_s = 0.0;
        if cassette.is_builtin() {
            self.duration_s = cassette.default_duration_s();
            self.looping = true;
            self.ease_io = false;
        } else if self.keyframes.len() < 2 {
            self.playing = false;
        }
        self.sync_lut();
    }

    /// Toggles Auto-Drive ON/OFF without altering `self.cassette` or `self.keyframes`.
    ///
    /// Returns the resulting `playing` state.
    pub fn set_playing(&mut self, on: bool) -> bool {
        if on && self.can_play() {
            if !self.looping && self.time_s >= self.duration_s.max(0.5) {
                self.time_s = 0.0;
            }
            self.playing = true;
            self.sync_lut();
        } else {
            self.playing = false;
        }
        self.playing
    }

    /// Appends the current camera pose to the `Custom` keyframe list and selects `RailCassette::Custom`.
    pub fn push_pose(&mut self, pose: CameraPose) {
        self.keyframes.push(CameraKeyframe::from_pose(pose));
        self.cassette = RailCassette::Custom;
        self.sync_lut();
    }

    /// Removes the last `Custom` keyframe, pausing playback only if `Custom` is active with `< 2` KFs.
    pub fn pop_keyframe(&mut self) {
        self.keyframes.pop();
        if self.cassette == RailCassette::Custom && self.keyframes.len() < 2 {
            self.playing = false;
            self.time_s = 0.0;
        }
        self.sync_lut();
    }

    /// Clears all `Custom` keyframes without disturbing built-in cassettes.
    pub fn clear(&mut self) {
        self.keyframes.clear();
        if self.cassette == RailCassette::Custom {
            self.playing = false;
            self.time_s = 0.0;
        }
        self.sync_lut();
    }

    /// Rebuilds the inline arc-length LUT only when the active track signature has changed.
    pub fn sync_lut(&mut self) {
        let sig = rail_lut_signature(self.cassette, self.looping, &self.keyframes);
        if self.arc_lut.signature == sig {
            return;
        }
        self.arc_lut = self.compute_lut(sig);
    }

    fn compute_lut(&self, sig: u64) -> ArcLengthLut {
        if self.cassette.is_builtin() {
            let wps = builtin_waypoints(self.cassette);
            ArcLengthLut::build(sig, |u| eval_builtin_eye_raw(wps, u, self.looping))
        } else {
            ArcLengthLut::build(sig, |u| eval_custom_eye_raw(&self.keyframes, u, self.looping))
        }
    }

    /// Advances the rail clock by `dt` seconds with zero heap allocations.
    pub fn tick(&mut self, dt: f32) {
        self.sync_lut();
        if !self.is_active() || !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let dur = self.duration_s.max(0.5);
        self.time_s += dt;
        if self.looping {
            self.time_s = self.time_s.rem_euclid(dur);
        } else if self.time_s >= dur {
            self.time_s = dur;
            self.playing = false;
        }
    }

    /// Evaluates the active cassette spline at the current cursor `self.time_s`.
    pub fn evaluate_current(&self, znear: f32, zfar: f32) -> Option<CameraPose> {
        self.evaluate_at(self.time_s, znear, zfar)
    }

    /// Evaluates the active cassette spline at `time_s` using arc-length parameterization,
    /// periodic C1 Centripetal Catmull-Rom, and smooth curvature banking on built-in tracks.
    pub fn evaluate_at(&self, time_s: f32, znear: f32, zfar: f32) -> Option<CameraPose> {
        let dur = self.duration_s.max(0.5);
        let raw_frac = if self.looping {
            (time_s / dur).rem_euclid(1.0)
        } else {
            (time_s / dur).clamp(0.0, 1.0)
        };
        let arc_frac = if self.ease_io {
            smoothstep01(raw_frac)
        } else {
            raw_frac
        };

        let sig = rail_lut_signature(self.cassette, self.looping, &self.keyframes);
        let local_lut;
        let lut = if self.arc_lut.signature == sig {
            &self.arc_lut
        } else {
            local_lut = self.compute_lut(sig);
            &local_lut
        };
        let u = lut.u_for_fraction(arc_frac);

        if self.cassette.is_builtin() {
            let wps = builtin_waypoints(self.cassette);
            if wps.is_empty() {
                return None;
            }
            let eye = eval_builtin_eye_raw(wps, u, self.looping);
            let (target, vfov_deg) = eval_builtin_target_fov_raw(wps, u, self.looping);
            let delta = target - eye;
            let fwd = if delta.length_squared() > 1.0e-8 {
                delta.normalize()
            } else {
                Vec3::NEG_Z
            };

            // Compute smooth curvature-driven banking roll around `fwd` via centered arc-length differences.
            let max_bank_rad = self.cassette.max_bank_deg().to_radians();
            let bank_rad = if max_bank_rad > 1.0e-4 && lut.total_len > 1.0 {
                let ds = 0.012f32;
                let (s_prev, s_next, span_frac) = if self.looping {
                    (
                        (arc_frac - ds).rem_euclid(1.0),
                        (arc_frac + ds).rem_euclid(1.0),
                        ds,
                    )
                } else {
                    let sp = (arc_frac - ds).max(0.0);
                    let sn = (arc_frac + ds).min(1.0);
                    (sp, sn, 0.5 * (sn - sp).max(1.0e-4))
                };
                let p_prev = eval_builtin_eye_raw(wps, lut.u_for_fraction(s_prev), self.looping);
                let p_next = eval_builtin_eye_raw(wps, lut.u_for_fraction(s_next), self.looping);
                let h_dist = (span_frac * lut.total_len).max(0.25);
                let vel = (p_next - p_prev) / (2.0 * h_dist);
                let acc = (p_next - 2.0 * eye + p_prev) / (h_dist * h_dist);
                let vel_xz_sq = vel.x * vel.x + vel.z * vel.z;
                let right_raw = fwd.cross(Vec3::Y);
                if vel_xz_sq > 0.05 && right_raw.length_squared() > 1.0e-6 {
                    let right_flat = right_raw.normalize();
                    // Signed horizontal curvature: positive when curving right, negative when curving left.
                    let acc_lat = acc.x * right_flat.x + acc.z * right_flat.z;
                    let curvature = acc_lat / vel_xz_sq.max(0.1);
                    let normalized =
                        (curvature * self.cassette.bank_gain()).atan() * std::f32::consts::FRAC_2_PI;
                    // Lean INTO the turn smoothly (right turn rolls clockwise around +fwd).
                    (normalized * max_bank_rad).clamp(-max_bank_rad, max_bank_rad)
                } else {
                    0.0
                }
            } else {
                0.0
            };

            let up_world = if bank_rad.abs() > 1.0e-5 {
                let right_flat = fwd.cross(Vec3::Y).normalize_or_zero();
                if right_flat.length_squared() > 1.0e-6 {
                    let up_flat = right_flat.cross(fwd).normalize();
                    // Positive `bank_rad` (right turn) tilts camera +up toward +right_flat (leaning into the turn).
                    (up_flat * bank_rad.cos() + right_flat * bank_rad.sin()).normalize()
                } else {
                    Vec3::Y
                }
            } else {
                Vec3::Y
            };

            return Some(CameraPose::from_eye_forward_up(
                eye,
                fwd,
                up_world,
                vfov_deg.clamp(10.0, 120.0).to_radians(),
                znear.max(1e-3),
                zfar.max(znear + 1.0),
            ));
        }

        // Custom user keyframe spline.
        let n = self.keyframes.len();
        if n == 0 {
            return None;
        }
        if n == 1 {
            return Some(self.keyframes[0].to_pose(znear, zfar));
        }

        let seg = spline_segment(n, u, self.looping);
        let k0 = self.keyframes[seg.i0];
        let k1 = self.keyframes[seg.i1];
        let k2 = self.keyframes[seg.i2];
        let k3 = self.keyframes[seg.i3];

        let p1 = Vec3::from_array(k1.pos);
        let p2 = Vec3::from_array(k2.pos);
        let p0 = if !self.looping && seg.seg_idx == 0 {
            p1 * 2.0 - p2
        } else {
            Vec3::from_array(k0.pos)
        };
        let p3 = if !self.looping && seg.seg_idx + 1 == seg.seg_count {
            p2 * 2.0 - p1
        } else {
            Vec3::from_array(k3.pos)
        };

        let eye = centripetal_catmull_rom(p0, p1, p2, p3, seg.local_s);
        let orientation = if n >= 3 {
            squad_interpolate(k0.quat(), k1.quat(), k2.quat(), k3.quat(), seg.local_s)
        } else {
            shortest_slerp(k1.quat(), k2.quat(), seg.local_s)
        };
        let vfov_deg = k1.vfov_deg + (k2.vfov_deg - k1.vfov_deg) * seg.local_s;

        Some(CameraPose {
            eye,
            orientation,
            fov_y: vfov_deg.clamp(10.0, 120.0).to_radians(),
            znear: znear.max(1e-3),
            zfar: zfar.max(znear + 1.0),
        })
    }

    /// Constructs a ready-to-play Gundo Auto-Drive rail on the default `CoastalCruise` cassette.
    #[cfg(test)]
    pub fn gundo_auto_drive() -> Self {
        let mut rail = Self::default();
        rail.select_cassette(RailCassette::CoastalCruise);
        rail.playing = true;
        rail
    }
}

/// Exponential gimbal camera damper smoothing position, orientation (`slerp`), and vertical FoV.
///
/// Snaps immediately on teleport / preset changes or when `tau <= 1e-4`, and settles cleanly onto
/// the exact target pose once within convergence epsilon so progressive path tracing can converge.
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraDamper {
    smoothed: Option<CameraPose>,
}

impl CameraDamper {
    /// Immediately snaps the smoothed pose to `pose` without damping lag.
    #[inline]
    pub fn snap(&mut self, pose: CameraPose) {
        self.smoothed = Some(pose);
    }

    /// Advances the damped pose toward `target` over `dt` seconds with time constant `tau` seconds.
    pub fn step(&mut self, target: CameraPose, dt: f32, tau: f32) -> CameraPose {
        if !tau.is_finite() || tau <= 1.0e-4 || !dt.is_finite() || dt <= 0.0 {
            self.smoothed = Some(target);
            return target;
        }
        let Some(prev) = self.smoothed else {
            self.smoothed = Some(target);
            return target;
        };

        let alpha = (1.0 - (-dt / tau).exp()).clamp(0.0, 1.0);
        let eye = prev.eye.lerp(target.eye, alpha);
        let orientation = shortest_slerp(prev.orientation, target.orientation, alpha);
        let fov_y = prev.fov_y + (target.fov_y - prev.fov_y) * alpha;

        // Snap once converged so path-tracing accumulation stops resetting on micro-residuals.
        let pos_settled = (eye - target.eye).length_squared() < 1.0e-10;
        let rot_settled = (1.0 - orientation.dot(target.orientation).abs()) < 1.0e-10;
        let fov_settled = (fov_y - target.fov_y).abs() < 1.0e-6;
        let next = if pos_settled && rot_settled && fov_settled {
            target
        } else {
            CameraPose {
                eye,
                orientation,
                fov_y,
                znear: target.znear,
                zfar: target.zfar,
            }
        };
        self.smoothed = Some(next);
        next
    }
}

/// Computes the exact simulation time delta for subframe `sub_idx` (`0..sub_count`) of a
/// 180-degree shutter window spanning `0.5 * frame_dt` per output video frame.
///
/// The sum of deltas across `sub_idx = 0..sub_count` equals `frame_dt` identically so long
/// recordings experience zero time drift regardless of `sub_count` (`1`, `2`, or `4`).
#[inline]
pub fn shutter_subframe_dt(frame_dt: f32, sub_idx: u32, sub_count: u32) -> f32 {
    if !frame_dt.is_finite() || frame_dt <= 0.0 {
        return 0.0;
    }
    let n = sub_count.max(1);
    if n == 1 {
        return frame_dt;
    }
    let sub_dt = (0.5 * frame_dt) / (n as f32);
    if sub_idx == 0 {
        // Jump across the closed-shutter dark interval from the previous frame's last subframe.
        frame_dt - sub_dt * ((n - 1) as f32)
    } else {
        sub_dt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoothstep_is_monotonic_and_clamped() {
        assert_eq!(smoothstep01(-0.5), 0.0);
        assert_eq!(smoothstep01(0.0), 0.0);
        assert_eq!(smoothstep01(1.0), 1.0);
        assert_eq!(smoothstep01(1.5), 1.0);
        assert_eq!(smoothstep01(f32::NAN), 0.0);

        let mut prev = 0.0f32;
        for i in 1..=100 {
            let v = smoothstep01(i as f32 / 100.0);
            assert!(v >= prev, "smoothstep must be monotonic: {v} < {prev}");
            prev = v;
        }
    }

    #[test]
    fn open_rail_interpolates_exact_endpoints_and_shortest_slerp() {
        let mut rail = CameraRail {
            cassette: RailCassette::Custom,
            duration_s: 10.0,
            looping: false,
            ease_io: true,
            ..Default::default()
        };
        let k0 = CameraKeyframe::from_look_at([0.0, 2.0, -10.0], [0.0, 1.0, 0.0], 45.0);
        // Negate quaternion sign on k1 to verify shortest-path interpolation never flips 360 degrees.
        let mut k1 = CameraKeyframe::from_look_at([10.0, 4.0, 0.0], [0.0, 1.0, 0.0], 60.0);
        k1.orientation = k1.orientation.map(|v| -v);
        let k2 = CameraKeyframe::from_look_at([0.0, 3.0, 10.0], [0.0, 1.0, 0.0], 50.0);
        rail.keyframes = vec![k0, k1, k2];
        rail.sync_lut();

        let p_start = rail.evaluate_at(0.0, 0.05, 500.0).expect("start");
        assert!((p_start.eye - Vec3::from_array(k0.pos)).length() < 1e-4);
        assert!((p_start.fov_y.to_degrees() - 45.0).abs() < 1e-4);

        let p_end = rail.evaluate_at(10.0, 0.05, 500.0).expect("end");
        assert!((p_end.eye - Vec3::from_array(k2.pos)).length() < 1e-4);
        assert!((p_end.fov_y.to_degrees() - 50.0).abs() < 1e-4);

        let mut prev_fwd = p_start.forward();
        for i in 1..=40 {
            let p = rail
                .evaluate_at(10.0 * (i as f32 / 40.0), 0.05, 500.0)
                .expect("sample");
            assert!(p.eye.is_finite());
            assert!(p.orientation.is_finite());
            assert!(
                p.forward().dot(prev_fwd) > 0.85,
                "orientation discontinuity at step {i}"
            );
            prev_fwd = p.forward();
        }
    }

    #[test]
    fn closed_loop_c1_seam_continuity_for_all_cassettes() {
        for &cassette in RailCassette::BUILTINS {
            let mut rail = CameraRail::default();
            rail.select_cassette(cassette);
            let dur = rail.duration_s;
            let eps = 0.001 * dur;

            let p_before = rail.evaluate_at(dur - eps, 0.05, 2000.0).expect("before");
            let p_zero = rail.evaluate_at(0.0, 0.05, 2000.0).expect("zero");
            let p_after = rail.evaluate_at(eps, 0.05, 2000.0).expect("after");

            // Position continuity across the seam.
            let seam_jump = p_before.eye.distance(p_zero.eye);
            let next_step = p_zero.eye.distance(p_after.eye);
            assert!(
                seam_jump > 1e-4 && next_step > 1e-4,
                "{cassette:?}: degenerate seam step"
            );
            let speed_ratio = (seam_jump - next_step).abs() / next_step.max(1e-4);
            assert!(
                speed_ratio < 0.06,
                "{cassette:?}: velocity magnitude pop at loop seam (before={seam_jump}, after={next_step})"
            );

            // C1 tangent velocity direction continuity across `1.0 -> 0.0`.
            let v_in = (p_zero.eye - p_before.eye).normalize();
            let v_out = (p_after.eye - p_zero.eye).normalize();
            assert!(
                v_in.dot(v_out) > 0.99,
                "{cassette:?}: C1 velocity kink across loop seam (dot={})",
                v_in.dot(v_out)
            );

            // Orientation (forward + up/bank) continuity across `1.0 -> 0.0`.
            assert!(
                p_before.forward().dot(p_zero.forward()) > 0.995,
                "{cassette:?}: forward jump across loop seam"
            );
            assert!(
                p_before.up().dot(p_zero.up()) > 0.99,
                "{cassette:?}: bank/up jump across loop seam"
            );
        }

        // Also verify periodic closed-loop Custom rail with SQUAD orientation continuity.
        let mut custom = CameraRail {
            cassette: RailCassette::Custom,
            duration_s: 12.0,
            looping: true,
            ease_io: false,
            keyframes: vec![
                CameraKeyframe::from_look_at([-20.0, 5.0, -20.0], [0.0, 2.0, 0.0], 50.0),
                CameraKeyframe::from_look_at([20.0, 8.0, -20.0], [0.0, 2.0, 0.0], 52.0),
                CameraKeyframe::from_look_at([20.0, 6.0, 20.0], [0.0, 2.0, 0.0], 50.0),
                CameraKeyframe::from_look_at([-20.0, 9.0, 20.0], [0.0, 2.0, 0.0], 48.0),
            ],
            ..Default::default()
        };
        custom.sync_lut();
        let eps = 0.012;
        let p_b = custom.evaluate_at(12.0 - eps, 0.05, 500.0).expect("b");
        let p_0 = custom.evaluate_at(0.0, 0.05, 500.0).expect("0");
        let p_a = custom.evaluate_at(eps, 0.05, 500.0).expect("a");
        let v_in = (p_0.eye - p_b.eye).normalize();
        let v_out = (p_a.eye - p_0.eye).normalize();
        assert!(v_in.dot(v_out) > 0.99, "custom loop velocity kink");
        assert!(p_b.forward().dot(p_0.forward()) > 0.995, "custom loop orientation pop");

        // Verify banking leans INTO a right-curving turn (`up.dot(right_flat) > 0`).
        // `RefinerySlalom` curves right from `+Z` (Frontage Ave) into `+X` (refinery) around `t = 0.14 * dur`.
        let mut slalom = CameraRail::default();
        slalom.select_cassette(RailCassette::RefinerySlalom);
        let mut saw_right_bank = false;
        let mut saw_left_bank = false;
        let bank_dt = 0.012 * slalom.duration_s;
        for i in 0..64 {
            let t = slalom.duration_s * (i as f32 / 64.0);
            let p_prev = slalom
                .evaluate_at((t - bank_dt).rem_euclid(slalom.duration_s), 0.05, 2000.0)
                .expect("bp");
            let p = slalom.evaluate_at(t, 0.05, 2000.0).expect("bc");
            let p_next = slalom
                .evaluate_at((t + bank_dt).rem_euclid(slalom.duration_s), 0.05, 2000.0)
                .expect("bn");
            let right_flat = p.forward().cross(Vec3::Y).normalize_or_zero();
            if right_flat.length_squared() > 0.5 {
                let acc = p_next.eye - 2.0 * p.eye + p_prev.eye;
                let lat = acc.x * right_flat.x + acc.z * right_flat.z;
                let up_lean = p.up().dot(right_flat);
                if lat > 0.4 {
                    assert!(
                        up_lean > 0.0,
                        "right turn must bank INTO turn (up.dot(right_flat)={up_lean}, lat={lat})"
                    );
                    saw_right_bank = true;
                } else if lat < -0.4 {
                    assert!(
                        up_lean < 0.0,
                        "left turn must bank INTO turn (up.dot(right_flat)={up_lean}, lat={lat})"
                    );
                    saw_left_bank = true;
                }
            }
        }
        assert!(saw_right_bank && saw_left_bank, "expected both right and left banked turns");
    }

    #[test]
    fn arc_length_parameterization_keeps_speed_uniform() {
        // Deliberately non-uniform control spacing: 3m micro-segment next to a 60m straight!
        let mut rail = CameraRail {
            cassette: RailCassette::Custom,
            duration_s: 10.0,
            looping: false,
            ease_io: false,
            keyframes: vec![
                CameraKeyframe::from_look_at([0.0, 4.0, 0.0], [0.0, 4.0, 100.0], 50.0),
                CameraKeyframe::from_look_at([0.0, 4.0, 3.0], [0.0, 4.0, 100.0], 50.0),
                CameraKeyframe::from_look_at([0.0, 4.0, 63.0], [0.0, 4.0, 100.0], 50.0),
                CameraKeyframe::from_look_at([0.0, 4.0, 68.0], [0.0, 4.0, 100.0], 50.0),
            ],
            ..Default::default()
        };
        rail.sync_lut();

        let steps = 64;
        let mut prev = rail.evaluate_at(0.0, 0.05, 500.0).expect("s0").eye;
        let mut min_step = f32::INFINITY;
        let mut max_step = 0.0f32;
        for i in 1..=steps {
            let t = rail.duration_s * (i as f32 / steps as f32);
            let cur = rail.evaluate_at(t, 0.05, 500.0).expect("si").eye;
            let d = cur.distance(prev);
            min_step = min_step.min(d);
            max_step = max_step.max(d);
            prev = cur;
        }
        // Without arc-length parameterization this ratio would be ~20x; with LUT it is ~1.0x!
        assert!(
            max_step / min_step.max(1e-5) < 1.12,
            "expected uniform speed across uneven control points, min={min_step}, max={max_step}"
        );

        // Verify uniform speed across all built-in cassettes too.
        for &cassette in RailCassette::BUILTINS {
            let mut r = CameraRail::default();
            r.select_cassette(cassette);
            let mut prev = r.evaluate_at(0.0, 0.05, 2000.0).expect("c0").eye;
            let mut d_min = f32::INFINITY;
            let mut d_max = 0.0f32;
            for i in 1..=96 {
                let t = r.duration_s * (i as f32 / 96.0);
                let cur = r.evaluate_at(t, 0.05, 2000.0).expect("ci").eye;
                let d = cur.distance(prev);
                d_min = d_min.min(d);
                d_max = d_max.max(d);
                prev = cur;
            }
            assert!(
                d_max / d_min.max(1e-5) < 1.15,
                "{cassette:?}: uneven speed (min={d_min}, max={d_max})"
            );
        }
    }

    #[test]
    fn all_builtin_gundo_cassettes_clear_scene_geometry() {
        use csg_core::eval::trace_compound;
        use csg_core::math::{Aabb, Ray, Vec3 as CsgVec3};
        use csg_urban::{GundoParams, build_gundo_scene};

        let world = build_gundo_scene(&GundoParams::default());
        let clearance = 0.55f32;
        let pad = CsgVec3::new(clearance, clearance, clearance);
        let probe_dirs = [
            CsgVec3::new(1.0, 0.0, 0.0),
            CsgVec3::new(-1.0, 0.0, 0.0),
            CsgVec3::new(0.0, 1.0, 0.0),
            CsgVec3::new(0.0, -1.0, 0.0),
            CsgVec3::new(0.0, 0.0, 1.0),
            CsgVec3::new(0.0, 0.0, -1.0),
        ];

        let check_instance =
            |cassette: RailCassette, sample_idx: usize, eye_w: CsgVec3, inst: &csg_scene::Instance| {
                let Some(asm) = world.assembly(inst.assembly_id) else {
                    return;
                };
                let world_box = inst.xform.xform_aabb(asm.local_aabb);
                let expanded = Aabb::new(world_box.min - pad, world_box.max + pad);
                if eye_w.x < expanded.min.x
                    || eye_w.x > expanded.max.x
                    || eye_w.y < expanded.min.y
                    || eye_w.y > expanded.max.y
                    || eye_w.z < expanded.min.z
                    || eye_w.z > expanded.max.z
                {
                    return;
                }
                for part in &asm.parts {
                    let full_xform = inst.xform * part.local_xform;
                    let Some(inv) = full_xform.invert() else {
                        continue;
                    };
                    let ro = inv.xform_pt(eye_w);
                    for &cid in part.lods.compounds_for_lod(0) {
                        for &dir_w in &probe_dirs {
                            let rd = inv.xform_vec(dir_w).norm();
                            let ray = Ray::new(ro, rd, 1.0e-4, 200.0);
                            if let Some(hit) = trace_compound(&world.library, cid, ray) {
                                assert!(
                                    hit.entering,
                                    "{cassette:?} sample {sample_idx} at ({:.2}, {:.2}, {:.2}) is INSIDE solid assembly '{}' (exit t={:.3})",
                                    eye_w.x,
                                    eye_w.y,
                                    eye_w.z,
                                    asm.name,
                                    hit.t
                                );
                                assert!(
                                    hit.t >= clearance,
                                    "{cassette:?} sample {sample_idx} at ({:.2}, {:.2}, {:.2}) clips within {:.2}m of '{}' (dist={:.3}m)",
                                    eye_w.x,
                                    eye_w.y,
                                    eye_w.z,
                                    clearance,
                                    asm.name,
                                    hit.t
                                );
                            }
                        }
                    }
                }
            };

        let samples_per_cassette = 192usize;
        for &cassette in RailCassette::BUILTINS {
            let mut rail = CameraRail::default();
            rail.select_cassette(cassette);
            for i in 0..samples_per_cassette {
                let t = rail.duration_s * (i as f32 / samples_per_cassette as f32);
                let pose = rail
                    .evaluate_at(t, 0.05, 2000.0)
                    .expect("evaluate cassette pose");
                assert!(
                    pose.eye.is_finite() && pose.orientation.is_finite(),
                    "{cassette:?} sample {i}: non-finite pose"
                );
                assert!(
                    pose.eye.y > 0.6,
                    "{cassette:?} sample {i}: below ground clearance (y={:.2})",
                    pose.eye.y
                );
                let eye_w = CsgVec3::new(pose.eye.x, pose.eye.y, pose.eye.z);

                for chunk in world.chunks() {
                    let cb = chunk.bounds;
                    if eye_w.x < cb.min.x - clearance
                        || eye_w.x > cb.max.x + clearance
                        || eye_w.y < cb.min.y - clearance
                        || eye_w.y > cb.max.y + clearance
                        || eye_w.z < cb.min.z - clearance
                        || eye_w.z > cb.max.z + clearance
                    {
                        continue;
                    }
                    for inst in chunk.instances() {
                        check_instance(cassette, i, eye_w, inst);
                    }
                }
                for inst in world.dynamic_instances() {
                    check_instance(cassette, i, eye_w, inst);
                }
            }
        }
    }

    #[test]
    fn custom_keyframes_preserved_across_cassette_switches_and_serde() {
        let mut rail = CameraRail::default();
        let p0 = CameraKeyframe::from_look_at([-10.0, 3.0, -10.0], [0.0, 2.0, 0.0], 48.0)
            .to_pose(0.05, 500.0);
        let p1 = CameraKeyframe::from_look_at([12.0, 5.0, 14.0], [0.0, 2.0, 0.0], 55.0)
            .to_pose(0.05, 500.0);
        rail.push_pose(p0);
        rail.push_pose(p1);
        assert_eq!(rail.cassette, RailCassette::Custom);
        assert_eq!(rail.keyframes.len(), 2);

        // Switch through built-in cassettes and toggle Auto-Drive OFF/ON.
        rail.select_cassette(RailCassette::RefinerySlalom);
        assert!(rail.set_playing(true));
        assert!(!rail.set_playing(false));
        assert_eq!(rail.keyframes.len(), 2);

        rail.select_cassette(RailCassette::GrandTour);
        assert_eq!(rail.keyframes.len(), 2);

        let json = serde_json::to_string(&rail).expect("serialize rail");
        let decoded: CameraRail = serde_json::from_str(&json).expect("deserialize rail");
        assert_eq!(decoded.cassette, RailCassette::GrandTour);
        assert_eq!(decoded.keyframes.len(), 2);
        assert!(!decoded.playing);
        assert_eq!(decoded.time_s, 0.0);
        assert_eq!(decoded, rail);
    }

    #[test]
    fn gimbal_damper_snaps_on_zero_and_converges_smoothly() {
        let p0 = CameraKeyframe::from_look_at([0.0, 2.0, -5.0], [0.0, 0.0, 0.0], 45.0)
            .to_pose(0.05, 500.0);
        let p1 = CameraKeyframe::from_look_at([6.0, 5.0, 4.0], [0.0, 1.0, 0.0], 60.0)
            .to_pose(0.05, 500.0);

        let mut damper = CameraDamper::default();
        damper.snap(p0);
        let snapped = damper.step(p1, 1.0 / 60.0, 0.0);
        assert_eq!(snapped.eye, p1.eye);
        assert_eq!(snapped.fov_y, p1.fov_y);

        damper.snap(p0);
        let mut prev_dist = p0.eye.distance(p1.eye);
        for _ in 0..600 {
            let cur = damper.step(p1, 1.0 / 60.0, 0.25);
            let d = cur.eye.distance(p1.eye);
            assert!(d <= prev_dist + 1e-6);
            prev_dist = d;
        }
        assert!(prev_dist < 1e-4, "expected convergence, dist={prev_dist}");
    }

    #[test]
    fn shutter_subframe_deltas_sum_exactly_to_frame_dt() {
        let frame_dt = 1.0 / 60.0;
        for n in [1u32, 2, 4] {
            let sum: f32 = (0..n).map(|k| shutter_subframe_dt(frame_dt, k, n)).sum();
            assert!(
                (sum - frame_dt).abs() < 1e-6,
                "n={n}: sum={sum} != frame_dt={frame_dt}"
            );
            for k in 0..n {
                assert!(shutter_subframe_dt(frame_dt, k, n) > 0.0);
            }
        }
    }
}
