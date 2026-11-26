//! Conversion, re-timing, and `camera-guide-rs` dynamics analysis for `nv-demo` flight tracks.
//!
//! **What.** Bridges legacy curated [`csg_scene::RailSpec`] cassettes and Flight Lab manual
//! keyframes into unified [`Track`] trajectories, and runs [`plan`] on any track to produce
//! jerk-limited speed/bank curves ([`DynamicsCurvePlot`]) and a re-timed [`Track`] preserving the
//! track's aiming and optical framing.
//! **Why.** Having one canonical [`Track`] format eliminates split playback engines
//! between curated rails, manual keyframes, and disk-stored `*.fnav.json` flights while keeping
//! `camera-guide-rs` as the single authoritative trajectory dynamics solver.
//! **How.** [`curated_rail_to_track`] compiles a [`csg_scene::RailSpec`] once, samples
//! [`csg_scene::CompiledRail::eval_at_fraction`] densely along uniform arc length, aligns
//! quaternion hemispheres, and constructs a C2 [`Track`] via
//! [`Track::from_keyframe_poses`] (matching curated poses within 1 cm and 0.1 deg).
//! [`analyze_and_retime_track`] extracts continuous spans between cuts, solves [`plan`]
//! on each span's [`GuidePath`], and rebuilds variable-speed C2 [`Transition`]
//! segments honoring the solved `s(t)` timing and coordinated bank `bank(t)`.
//! **Uses.** `camera`, `csg_scene`, `flight_navigator::{camera_guide, fnav_core}`, `glam`.
//! **Used by.** `crate::flight::lab`, `crate::flight::player`, and unit tests in `crate::flight`.

use camera::{CameraPose, FOV_MAX, FOV_MIN};
use csg_scene::{CompiledRail, RailSpec};
use flight_navigator::camera_guide::{
    GRID_DS_M, GuideError, GuideLimits, GuidePath, LimitTerm, PlanInput, SpeedTarget, VerifyReport,
    plan,
};
use flight_navigator::fnav_core::{
    Aim, CameraState, FilmBack, Kinematic, Lens, Segment, Track, Transition,
};
use glam::{Quat, Vec3};

use crate::flight::lab::{DynamicsCurvePlot, ManualKeyframe};

/// Minimum curated-rail sample count ensuring spline chord error stays well below 1 cm / 0.1 deg.
const MIN_CURATED_SAMPLES: usize = 64;
/// Upper bound on curated-rail sample count to keep track serialization compact.
const MAX_CURATED_SAMPLES: usize = 512;
/// Minimum path length (m) required by `camera-guide-rs` (`4 * GRID_DS_M`).
const MIN_GUIDE_PATH_LEN_M: f32 = 4.0 * GRID_DS_M;

/// Normalizes `q` safely, returning [`Quat::IDENTITY`] if non-finite or near-zero.
#[inline]
pub(crate) fn safe_unit_quat(q: Quat) -> Quat {
    if q.is_finite() && q.length_squared() > 1.0e-6 {
        q.normalize()
    } else {
        Quat::IDENTITY
    }
}

/// Aligns `q` into the same quaternion hemisphere as `prev` so shortest-arc slerp never flips.
#[inline]
fn align_quat(prev: Quat, q: Quat) -> Quat {
    let u = safe_unit_quat(q);
    if prev.dot(u) < 0.0 { -u } else { u }
}

/// Converts a curated [`csg_scene::RailSpec`] into an immutable [`Track`].
///
/// Compiles [`CompiledRail::compile`] once, samples [`CompiledRail::eval_at_fraction`] densely
/// along normalized arc length (`max(64, ceil(duration_s * 4))` samples, capped at `512`), and
/// builds a C2 track via [`Track::from_keyframe_poses`] so evaluated samples match the curated
/// rail within 1 cm in position and 0.1 deg in orientation.
///
/// # Errors
/// Returns an error description if `spec` has fewer than 2 waypoints or a non-positive duration.
pub fn curated_rail_to_track(spec: &RailSpec) -> Result<Track, String> {
    if !spec.is_valid() {
        return Err(format!(
            "curated rail `{}` is invalid (need >= 2 waypoints and positive duration)",
            spec.id
        ));
    }

    let compiled = CompiledRail::compile(spec.clone());
    let dur = f64::from(spec.duration_s.max(2.0));
    let n = ((dur * 4.0).ceil() as usize).clamp(MIN_CURATED_SAMPLES, MAX_CURATED_SAMPLES);
    let sample_count = if spec.looping { n } else { n + 1 };

    let mut poses = Vec::with_capacity(sample_count);
    let mut prev_q: Option<Quat> = None;

    for i in 0..sample_count {
        let frac = (i as f32) / (n as f32);
        let smp = compiled.eval_at_fraction(frac);
        let eye = Vec3::new(smp.eye.x, smp.eye.y, smp.eye.z);
        let fwd = Vec3::new(smp.forward.x, smp.forward.y, smp.forward.z);
        let up = Vec3::new(smp.up.x, smp.up.y, smp.up.z);
        let fov_y = smp.vfov_deg.to_radians().clamp(FOV_MIN, FOV_MAX);

        let pose = CameraPose::from_eye_forward_up(eye, fwd, up, fov_y, 0.05, 5000.0);
        let q = match prev_q {
            Some(pq) => align_quat(pq, pose.orientation),
            None => safe_unit_quat(pose.orientation),
        };
        prev_q = Some(q);
        poses.push((pose.eye, q, pose.fov_y));
    }

    Ok(Track::from_keyframe_poses(
        &poses,
        dur,
        spec.looping,
        FilmBack::default(),
    ))
}

/// Builds a smooth [`Track`] from Flight Lab [`ManualKeyframe`]s, applying
/// `camera-guide-rs` jerk-limited timing and coordinated bank when the path is long enough.
///
/// # Errors
/// Returns an error description if fewer than 2 keyframes are provided, `duration_s` is invalid,
/// or `limits` are rejected by `camera-guide-rs`.
pub fn keyframes_to_track(
    kfs: &[ManualKeyframe],
    duration_s: f64,
    looping: bool,
    limits: &GuideLimits,
) -> Result<Track, String> {
    if kfs.len() < 2 {
        return Err("need at least 2 keyframes to build a flight track".to_string());
    }
    if !duration_s.is_finite() || duration_s <= 0.1 {
        return Err(format!("invalid track duration {duration_s:.2} s"));
    }

    let mut poses = Vec::with_capacity(kfs.len());
    let mut prev_q: Option<Quat> = None;
    for kf in kfs {
        let eye = Vec3::from_array(kf.pos);
        if !eye.is_finite() {
            return Err("keyframe contains non-finite position coordinates".to_string());
        }
        let q_raw = Quat::from_array(kf.orientation);
        let q = match prev_q {
            Some(pq) => align_quat(pq, q_raw),
            None => safe_unit_quat(q_raw),
        };
        prev_q = Some(q);
        let fov_y = Lens::pinhole(kf.focal_mm).fov_y().clamp(FOV_MIN, FOV_MAX);
        poses.push((eye, q, fov_y));
    }

    let raw = Track::from_keyframe_poses(&poses, duration_s, looping, FilmBack::default());
    let path_len = estimate_track_length_m(&raw);
    if path_len < MIN_GUIDE_PATH_LEN_M {
        return Ok(raw);
    }

    match analyze_and_retime_track(&raw, limits) {
        Ok((retimed, _plot)) => Ok(retimed),
        Err(err) if err.contains("path too short") => Ok(raw),
        Err(err) => Err(err),
    }
}

/// Analyzes `track`'s eye trajectory with `camera-guide-rs` under `limits` and returns both a
/// re-timed [`Track`] (preserving optics and aiming with coordinated bank) and a
/// [`DynamicsCurvePlot`] for Flight Lab inspection.
///
/// Tracks containing [`Segment::Cut`] boundaries are re-timed per continuous span so cuts remain
/// intact and `camera-guide` never bridges across an instantaneous teleport.
///
/// # Errors
/// Returns an error description if `track` is empty, has zero duration, is shorter than
/// `4 * GRID_DS_M` (2.0 m), or if [`plan`] rejects `limits`.
pub fn analyze_and_retime_track(
    track: &Track,
    limits: &GuideLimits,
) -> Result<(Track, DynamicsCurvePlot), String> {
    let total_dur = track.duration_s();
    if track.segments.is_empty() || !total_dur.is_finite() || total_dur <= 1.0e-4 {
        return Err("track is empty or has zero duration".to_string());
    }

    // Partition the track into continuous spans separated by `Segment::Cut`.
    let mut spans: Vec<(Vec<Segment>, Option<flight_navigator::fnav_core::Cut>)> = Vec::new();
    let mut cur_segs: Vec<Segment> = Vec::new();
    let mut leading_cut: Option<flight_navigator::fnav_core::Cut> = None;

    for seg in &track.segments {
        match seg {
            Segment::Cut(c) => {
                if !cur_segs.is_empty() {
                    spans.push((std::mem::take(&mut cur_segs), leading_cut.take()));
                }
                leading_cut = Some(c.clone());
            }
            Segment::Shot(_) | Segment::Transition(_) => {
                if seg.duration_s() > 0.0 {
                    cur_segs.push(seg.clone());
                }
            }
        }
    }
    if !cur_segs.is_empty() {
        spans.push((cur_segs, leading_cut.take()));
    }

    if spans.is_empty() {
        return Err("track has no timed segments".to_string());
    }

    let is_single_span = spans.len() == 1 && leading_cut.is_none();
    let mut out_segments = Vec::new();
    let mut plot_s = Vec::new();
    let mut plot_cap = Vec::new();
    let mut plot_v = Vec::new();
    let mut plot_bank_deg = Vec::new();
    let mut plot_binding = Vec::new();
    let mut combined_report = VerifyReport::default();
    let mut s_offset = 0.0_f32;

    for (span_segs, cut_before) in spans {
        if let Some(cut) = cut_before {
            out_segments.push(Segment::Cut(cut));
        }
        let span_looping = is_single_span && track.looping;
        let sub_track = Track {
            segments: span_segs,
            looping: span_looping,
        };
        let (retimed_segs, sub_plot) = retime_continuous_span(&sub_track, limits)?;
        out_segments.extend(retimed_segs);

        let span_len = sub_plot.s.last().copied().unwrap_or(0.0);
        for i in 0..sub_plot.s.len() {
            plot_s.push(s_offset + sub_plot.s[i]);
            plot_cap.push(sub_plot.cap[i]);
            plot_v.push(sub_plot.v[i]);
            plot_bank_deg.push(sub_plot.bank_deg[i]);
            plot_binding.push(sub_plot.binding[i]);
        }
        s_offset += span_len;
        merge_verify_report(&mut combined_report, &sub_plot.report);
    }

    if let Some(trailing) = leading_cut {
        out_segments.push(Segment::Cut(trailing));
    }

    let retimed = Track {
        segments: out_segments,
        looping: track.looping,
    };
    let plot = DynamicsCurvePlot {
        s: plot_s,
        cap: plot_cap,
        v: plot_v,
        bank_deg: plot_bank_deg,
        binding: plot_binding,
        report: combined_report,
    };
    Ok((retimed, plot))
}

/// Re-times one continuous (cut-free) [`Track`] span through `camera-guide-rs`.
fn retime_continuous_span(
    span: &Track,
    limits: &GuideLimits,
) -> Result<(Vec<Segment>, DynamicsCurvePlot), String> {
    let orig_dur = span.duration_s();
    if orig_dur <= 1.0e-4 {
        return Err("continuous span has zero duration".to_string());
    }

    let n_sample = ((orig_dur * 10.0).ceil() as usize).clamp(48, 1024);
    let m_pts = if span.looping { n_sample } else { n_sample + 1 };

    let mut orig_t = Vec::with_capacity(m_pts + 1);
    let mut orig_eyes = Vec::with_capacity(m_pts);
    for i in 0..m_pts {
        let t = orig_dur * (i as f64) / (n_sample as f64);
        let smp = span.eval(t);
        orig_t.push(t);
        orig_eyes.push(smp.state.eye.to_array());
    }

    // Build cumulative arc-length table for inverting `s -> orig_t`.
    let mut orig_cum_s = Vec::with_capacity(m_pts + 1);
    orig_cum_s.push(0.0_f64);
    let mut acc_s = 0.0_f64;
    for w in orig_eyes.windows(2) {
        let d = Vec3::from_array(w[1]) - Vec3::from_array(w[0]);
        acc_s += f64::from(d.length());
        orig_cum_s.push(acc_s);
    }
    if span.looping && m_pts >= 2 {
        let d = Vec3::from_array(orig_eyes[0]) - Vec3::from_array(orig_eyes[m_pts - 1]);
        acc_s += f64::from(d.length());
        orig_cum_s.push(acc_s);
        orig_t.push(orig_dur);
    }

    let total_len = acc_s as f32;
    if total_len < MIN_GUIDE_PATH_LEN_M {
        return Err(GuideError::PathTooShort.to_string());
    }

    let guide_path = GuidePath {
        points: orig_eyes,
        looping: span.looping,
    };
    let profile = guide_path.analyze().map_err(|e| e.to_string())?;
    let base_speed = (profile.length_m / (orig_dur as f32).max(0.5))
        .clamp(limits.v_min.max(0.5), 45.0);
    let target = SpeedTarget::base_mps(base_speed);

    let input = PlanInput {
        path: &profile,
        limits,
        target: &target,
        extra_cap: None,
        v_start: None,
        v_end: None,
    };
    let traj = plan(&input).map_err(|e| e.to_string())?;

    let plot = DynamicsCurvePlot {
        s: traj.s.clone(),
        cap: traj.cap.clone(),
        v: traj.v.clone(),
        bank_deg: traj.bank_rad.iter().map(|r| r.to_degrees()).collect(),
        binding: traj.binding.clone(),
        report: traj.report,
    };

    // Resample the solved trajectory at uniform output times so the resulting C2 Transition
    // segments reproduce `s(t)` (variable speed) and `bank(t)` accurately while keeping optics/aim.
    let new_dur = traj.duration_s.max(0.1);
    let seg_count = ((new_dur * 4.0).ceil() as usize).clamp(32, 384);
    let knot_count = if span.looping { seg_count } else { seg_count + 1 };
    let dt_seg = new_dur / (seg_count as f64);

    let mut states = Vec::with_capacity(knot_count);
    let mut prev_q: Option<Quat> = None;

    for i in 0..knot_count {
        let t_new = (i as f64) * dt_seg;
        let g = traj.at_time(t_new);
        let t_orig = invert_arc_length(&orig_cum_s, &orig_t, f64::from(g.s), span.looping);
        let base = span.eval(t_orig);
        let mut st = CameraState::from_forward_bank(
            base.state.eye,
            base.state.forward(),
            g.bank_rad,
            base.state.lens,
        );
        st.target = base.state.target;
        st.orientation = match prev_q {
            Some(pq) => align_quat(pq, st.orientation),
            None => safe_unit_quat(st.orientation),
        };
        prev_q = Some(st.orientation);
        states.push(st);
    }

    // Derive C2-consistent velocities, accelerations, angular velocities, and zoom rates at knots.
    let mut knots = Vec::with_capacity(knot_count);
    for i in 0..knot_count {
        let prev_i = if span.looping {
            (i + knot_count - 1) % knot_count
        } else {
            i.saturating_sub(1)
        };
        let next_i = if span.looping {
            (i + 1) % knot_count
        } else {
            (i + 1).min(knot_count - 1)
        };

        let dt_span = if !span.looping && (i == 0 || i + 1 == knot_count) {
            dt_seg as f32
        } else {
            (2.0 * dt_seg) as f32
        }
        .max(1.0e-4);

        let p_prev = states[prev_i].eye;
        let p_cur = states[i].eye;
        let p_next = states[next_i].eye;
        let vel = (p_next - p_prev) / dt_span;
        let acc = if !span.looping && (i == 0 || i + 1 == knot_count) {
            Vec3::ZERO
        } else {
            let h = (dt_seg as f32).max(1.0e-4);
            (p_next - 2.0 * p_cur + p_prev) / (h * h)
        };

        let ang_vel = finite_diff_ang_vel(states[prev_i].orientation, states[next_i].orientation, dt_span);
        let f_prev = states[prev_i].lens.focal_mm.max(1.0);
        let f_next = states[next_i].lens.focal_mm.max(1.0);
        let zoom_rate = (f_next.ln() - f_prev.ln()) / dt_span;

        knots.push(Kinematic {
            state: states[i],
            vel,
            acc,
            ang_vel,
            zoom_rate,
        });
    }

    let mut segments = Vec::with_capacity(seg_count);
    for i in 0..seg_count {
        let next_i = (i + 1) % knot_count;
        segments.push(Segment::Transition(Transition::direct(
            knots[i],
            knots[next_i],
            dt_seg,
            Aim::Squad,
        )));
    }

    Ok((segments, plot))
}

/// Maps arc length `s` back to original track time via piecewise-linear lookup on `cum_s`.
fn invert_arc_length(cum_s: &[f64], times: &[f64], s: f64, looping: bool) -> f64 {
    let Some(&total_s) = cum_s.last() else {
        return 0.0;
    };
    if cum_s.len() < 2 || times.len() < cum_s.len() || total_s <= 1.0e-6 {
        return 0.0;
    }
    let ss = if looping {
        s.rem_euclid(total_s)
    } else {
        s.clamp(0.0, total_s)
    };
    let idx = cum_s
        .partition_point(|&v| v < ss)
        .clamp(1, cum_s.len() - 1);
    let s0 = cum_s[idx - 1];
    let s1 = cum_s[idx];
    let t0 = times[idx - 1];
    let t1 = times[idx];
    let alpha = if (s1 - s0).abs() > 1.0e-9 {
        ((ss - s0) / (s1 - s0)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    t0 + (t1 - t0) * alpha
}

/// Estimates world-space angular velocity (rad/s) between `q0` and `q1` over `dt` seconds.
fn finite_diff_ang_vel(q0: Quat, q1: Quat, dt: f32) -> Vec3 {
    if dt <= 1.0e-5 {
        return Vec3::ZERO;
    }
    let a = safe_unit_quat(q0);
    let b = align_quat(a, q1);
    let delta = (b * a.conjugate()).normalize();
    let v = Vec3::new(delta.x, delta.y, delta.z);
    let sin_half = v.length();
    if sin_half < 1.0e-6 {
        Vec3::ZERO
    } else {
        let angle = 2.0 * sin_half.atan2(delta.w.clamp(-1.0, 1.0));
        (v / sin_half) * (angle / dt)
    }
}

/// Estimates the total eye path length (metres) of `track` via coarse sampling.
fn estimate_track_length_m(track: &Track) -> f32 {
    let dur = track.duration_s();
    if track.segments.is_empty() || dur <= 1.0e-4 {
        return 0.0;
    }
    let steps = 64usize;
    let mut prev = track.eval(0.0).state.eye;
    let mut acc = 0.0_f32;
    for i in 1..=steps {
        let t = dur * (i as f64) / (steps as f64);
        let cur = track.eval(t).state.eye;
        acc += prev.distance(cur);
        prev = cur;
    }
    acc
}

/// Merges two [`VerifyReport`]s by taking the worst ratio per limit term.
fn merge_verify_report(dst: &mut VerifyReport, src: &VerifyReport) {
    dst.lateral_accel = dst.lateral_accel.max(src.lateral_accel);
    dst.yaw_rate = dst.yaw_rate.max(src.yaw_rate);
    dst.bank = dst.bank.max(src.bank);
    dst.bank_rate = dst.bank_rate.max(src.bank_rate);
    dst.bank_accel = dst.bank_accel.max(src.bank_accel);
    dst.long_accel = dst.long_accel.max(src.long_accel);
    dst.long_decel = dst.long_decel.max(src.long_decel);
    dst.jerk = dst.jerk.max(src.jerk);
    dst.extra = dst.extra.max(src.extra);
    dst.iterations = dst.iterations.max(src.iterations);
}

/// Short human-readable label for a [`LimitTerm`] shown in Flight Lab badges/tooltips.
#[must_use]
pub const fn limit_term_label(term: LimitTerm) -> &'static str {
    match term {
        LimitTerm::Target => "Target",
        LimitTerm::Lateral => "Lateral Accel",
        LimitTerm::Yaw => "Yaw Rate",
        LimitTerm::Bank => "Max Bank",
        LimitTerm::BankRate => "Bank Rate",
        LimitTerm::BankAccel => "Bank Accel",
        LimitTerm::LongAccel => "Long Accel",
        LimitTerm::LongDecel => "Long Decel",
        LimitTerm::Jerk => "Jerk",
        LimitTerm::Extra => "Extra Cap",
        LimitTerm::VMin => "Min Speed",
    }
}
