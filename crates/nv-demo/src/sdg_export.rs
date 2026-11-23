//! Multi-channel OpenEXR Synthetic Data Generation (SDG) burst exporter, backend selection types,
//! and atomic dataset manifest writer backed by `exr-core`.
//!
//! What: defines [`CrowdBackend`], [`ActiveCrowdBackend`], [`SdgOccluderBackend`],
//! [`ActiveSdgOccluder`], [`SdgExrFrame`], [`SdgBurstManifest`], and atomic multi-channel EXR +
//! manifest writers (`R/G/B/A` linear Beauty, `depth.Z`, `motion.X`/`motion.Y`, `semantic.id`,
//! `instance.id`).
//! Why: enables deterministic 16/30/60-frame multi-AOV OpenEXR + JSON keypoint/BBox SDG dataset
//! capture without silent backend downgrades or partial files on crash.
//! Used by: `crate::renderer`, `crate::csg_scene_host`, `crate::app_events`, `crate::ui`,
//! `crate::main`.

use std::io::Write as _;
use std::path::Path;

use exr_core::attr::Compression;
use exr_core::io::MemOStream;
use exr_core::{ChannelData, Image};
use imath_rs::{Box2i, V2i};

/// User-selected articulated crowd simulation backend.
///
/// Explicit `Cpu` or `Cuda` requests never silently downgrade; only `Auto` may fall back and
/// always reports which concrete backend ran via [`ActiveCrowdBackend`].
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum CrowdBackend {
    /// Prefer GPU-direct CUDA crowd kernel when available, falling back cleanly to CPU.
    #[default]
    Auto,
    /// Run CPU `SceneBuilt::step_crowd` exclusively.
    Cpu,
    /// Run GPU-direct CUDA `step_and_emit_actors_cuda` exclusively (returns error if unavailable).
    Cuda,
}

impl CrowdBackend {
    /// All selectable backends in UI order.
    pub const ALL: [Self; 3] = [Self::Auto, Self::Cpu, Self::Cuda];

    /// Compact UI label.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Cpu => "CPU",
            Self::Cuda => "CUDA",
        }
    }
}

/// Concrete crowd simulation backend that actually executed on the latest frame.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ActiveCrowdBackend {
    /// No crowd actors stepped this frame.
    #[default]
    None,
    /// CPU `CrowdSim::step_analytical` executed.
    Cpu,
    /// GPU-direct CUDA `step_and_emit_actors_cuda` executed.
    Cuda,
}

impl ActiveCrowdBackend {
    /// Short HUD/telemetry label.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "Idle",
            Self::Cpu => "CPU",
            Self::Cuda => "CUDA",
        }
    }
}

/// User-selected ray-occlusion backend for SDG 2D/3D keypoint visibility classification.
///
/// Explicit `Cpu` or `Optix` requests never silently downgrade; only `Auto` may fall back and
/// reports the concrete backend that executed via [`ActiveSdgOccluder`].
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum SdgOccluderBackend {
    /// Prefer OptiX 9 hardware `__raygen__sdg_keypoints` when available, falling back to CPU.
    #[default]
    Auto,
    /// Run CPU `csg_actors::CsgRayOccluder` exclusively.
    Cpu,
    /// Run OptiX 9 hardware `__raygen__sdg_keypoints` exclusively (returns error if unavailable).
    Optix,
}

impl SdgOccluderBackend {
    /// All selectable SDG occluder backends in UI order.
    pub const ALL: [Self; 3] = [Self::Auto, Self::Cpu, Self::Optix];

    /// Compact UI label.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Cpu => "CPU",
            Self::Optix => "OptiX",
        }
    }
}

/// Concrete ray-occlusion backend that actually classified keypoints for an SDG export.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ActiveSdgOccluder {
    /// CPU `csg_actors::CsgRayOccluder` ran.
    #[default]
    Cpu,
    /// OptiX 9 hardware `__raygen__sdg_keypoints` ran.
    Optix,
}

impl ActiveSdgOccluder {
    /// Short human-readable label for status messages and dataset manifests.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Optix => "OptiX",
        }
    }
}

/// Borrowed host-side AOV planes packed into a single multi-channel OpenEXR SDG frame.
pub struct SdgExrFrame<'a> {
    /// Image width in pixels (`>= 1`).
    pub width: u32,
    /// Image height in pixels (`>= 1`).
    pub height: u32,
    /// Linear Beauty RGBA (`width * height`).
    pub color_rgba: &'a [[f32; 4]],
    /// Linear view-space depth in meters (`width * height`).
    pub depth: &'a [f32],
    /// Screen-space pixel optical flow `[dx, dy]` (`width * height`).
    pub motion: &'a [[f32; 2]],
    /// Per-pixel `SemanticId` (`0..=19`, `width * height`).
    pub semantic_id: &'a [u32],
    /// Per-pixel `InstanceId` / entity `user_id` (`width * height`).
    pub instance_id: &'a [u32],
}

/// Per-frame entry recorded in an SDG burst `manifest.json`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SdgBurstFrameEntry {
    /// Zero-based index inside the burst (`0..frame_count`).
    pub burst_index: u32,
    /// Render frame index at capture time.
    pub render_frame_index: u32,
    /// Simulation clock in seconds.
    pub time_s: f32,
    /// Relative filename of the multi-channel OpenEXR image (e.g. `"frame_0000.exr"`).
    pub exr_file: String,
    /// Relative filename of the keypoint/BBox annotation JSON (e.g. `"frame_0000.sdg.json"`).
    pub json_file: String,
    /// Number of articulated actors exported in `json_file`.
    pub actor_count: usize,
    /// Concrete occluder backend that classified keypoints (`"OptiX"` or `"CPU"`).
    pub occluder_used: String,
}

/// Top-level `manifest.json` written at the end of a multi-frame SDG EXR + JSON burst.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SdgBurstManifest {
    /// Schema version string.
    pub schema_version: String,
    /// Active CSG scene identifier (e.g. `"inspector"`, `"gundo_osm"`, `"marina"`).
    pub scene_id: String,
    /// Active 1-based scene index (`csg_mode`).
    pub scene_idx: u32,
    /// Active scene variant index (e.g. `109` for the 4-archetype crowd showcase).
    pub variant_idx: u32,
    /// Rendered image width in pixels.
    pub width: u32,
    /// Rendered image height in pixels.
    pub height: u32,
    /// Accumulated samples per pixel for each burst frame.
    pub spp_per_frame: u32,
    /// Optical sensor projection name.
    pub projection: String,
    /// Requested occluder mode (`"Auto"`, `"CPU"`, `"OptiX"`).
    pub requested_occluder: String,
    /// Active crowd simulation backend (`"CUDA"`, `"CPU"`, `"Idle"`).
    pub crowd_backend: String,
    /// Channels stored in every `.exr` file.
    pub exr_channels: Vec<String>,
    /// Ordered per-frame records.
    pub frames: Vec<SdgBurstFrameEntry>,
}

/// Canonical list of channels written into every SDG multi-channel OpenEXR frame.
pub const SDG_EXR_CHANNELS: [&str; 9] = [
    "R",
    "G",
    "B",
    "A",
    "depth.Z",
    "motion.X",
    "motion.Y",
    "semantic.id",
    "instance.id",
];

/// Encodes `frame` into a multi-channel ZIP-compressed scanline OpenEXR image and writes it
/// atomically to `path` (temp file in same directory + `flush` + `sync_all` + `rename`).
pub fn write_sdg_exr_atomic(path: &Path, frame: &SdgExrFrame<'_>) -> Result<(), String> {
    if frame.width == 0 || frame.height == 0 {
        return Err("SDG EXR dimensions must be non-zero".to_string());
    }
    let expected = (frame.width as usize)
        .checked_mul(frame.height as usize)
        .ok_or_else(|| "SDG EXR pixel count overflow".to_string())?;
    if frame.color_rgba.len() != expected
        || frame.depth.len() != expected
        || frame.motion.len() != expected
        || frame.semantic_id.len() != expected
        || frame.instance_id.len() != expected
    {
        return Err(format!(
            "SDG EXR buffer length mismatch: expected {expected}, got color={} depth={} motion={} sem={} inst={}",
            frame.color_rgba.len(),
            frame.depth.len(),
            frame.motion.len(),
            frame.semantic_id.len(),
            frame.instance_id.len(),
        ));
    }

    let mut r = Vec::with_capacity(expected);
    let mut g = Vec::with_capacity(expected);
    let mut b = Vec::with_capacity(expected);
    let mut a = Vec::with_capacity(expected);
    for px in frame.color_rgba {
        r.push(px[0]);
        g.push(px[1]);
        b.push(px[2]);
        a.push(px[3]);
    }
    let mut mx = Vec::with_capacity(expected);
    let mut my = Vec::with_capacity(expected);
    for mv in frame.motion {
        mx.push(mv[0]);
        my.push(mv[1]);
    }

    let dw = Box2i {
        min: V2i { x: 0, y: 0 },
        max: V2i {
            x: (frame.width as i32) - 1,
            y: (frame.height as i32) - 1,
        },
    };
    let img = Image::new(dw)
        .with_channel("R", ChannelData::Float(r))
        .with_channel("G", ChannelData::Float(g))
        .with_channel("B", ChannelData::Float(b))
        .with_channel("A", ChannelData::Float(a))
        .with_channel("depth.Z", ChannelData::Float(frame.depth.to_vec()))
        .with_channel("motion.X", ChannelData::Float(mx))
        .with_channel("motion.Y", ChannelData::Float(my))
        .with_channel("semantic.id", ChannelData::Uint(frame.semantic_id.to_vec()))
        .with_channel("instance.id", ChannelData::Uint(frame.instance_id.to_vec()));

    let mut mem = MemOStream::new();
    img.write_to(&mut mem, Compression::Zip)
        .map_err(|e| format!("exr-core encode failed: {e}"))?;
    let bytes = mem.into_data();

    write_bytes_atomic(path, &bytes, "exr.tmp")
}

/// Serializes `manifest` as pretty-printed JSON and writes it atomically to `path`.
pub fn write_sdg_manifest_atomic(path: &Path, manifest: &SdgBurstManifest) -> Result<(), String> {
    let json = serde_json::to_string_pretty(manifest)
        .map_err(|e| format!("SDG manifest serialize failed: {e}"))?;
    write_bytes_atomic(path, json.as_bytes(), "json.tmp")
}

/// Writes `bytes` atomically via a same-directory temp file + `flush` + `sync_all` + `rename`.
pub fn write_bytes_atomic(path: &Path, bytes: &[u8], tmp_ext: &str) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create dir {} failed: {e}", parent.display()))?;
    }
    let tmp_path = path.with_extension(tmp_ext);
    let mut file = std::fs::File::create(&tmp_path)
        .map_err(|e| format!("create temp {} failed: {e}", tmp_path.display()))?;
    file.write_all(bytes)
        .map_err(|e| format!("write {} failed: {e}", tmp_path.display()))?;
    file.flush()
        .map_err(|e| format!("flush {} failed: {e}", tmp_path.display()))?;
    file.sync_all()
        .map_err(|e| format!("sync_all {} failed: {e}", tmp_path.display()))?;
    drop(file);
    std::fs::rename(&tmp_path, path)
        .map_err(|e| format!("atomic rename to {} failed: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exr_multichannel_roundtrip_verifies_all_nine_channels() {
        let w = 8u32;
        let h = 6u32;
        let n = (w * h) as usize;
        let mut color_rgba = Vec::with_capacity(n);
        let mut depth = Vec::with_capacity(n);
        let mut motion = Vec::with_capacity(n);
        let mut semantic_id = Vec::with_capacity(n);
        let mut instance_id = Vec::with_capacity(n);

        for i in 0..n {
            let f = i as f32;
            color_rgba.push([f * 0.05 + 0.1, f * 0.02 + 0.2, f * 0.01 + 0.3, 1.0]);
            depth.push(2.5 + f * 0.75);
            motion.push([(f - 20.0) * 0.25, (10.0 - f) * 0.125]);
            semantic_id.push(match i % 4 {
                0 => 1,  // ROAD
                1 => 10, // VEHICLE
                2 => 18, // PEDESTRIAN
                _ => 19, // ANIMAL
            });
            instance_id.push(800_000 + i as u32);
        }

        let path = std::env::temp_dir().join("nv_demo_sdg_multichannel_test.exr");
        let frame = SdgExrFrame {
            width: w,
            height: h,
            color_rgba: &color_rgba,
            depth: &depth,
            motion: &motion,
            semantic_id: &semantic_id,
            instance_id: &instance_id,
        };
        write_sdg_exr_atomic(&path, &frame).expect("write multi-channel EXR");

        let read_back = Image::read(&path).expect("read multi-channel EXR back");
        let _ = std::fs::remove_file(&path);

        assert_eq!(read_back.width(), i64::from(w));
        assert_eq!(read_back.height(), i64::from(h));
        for &ch_name in &SDG_EXR_CHANNELS {
            assert!(
                read_back.channel(ch_name).is_some(),
                "missing channel {ch_name}"
            );
        }

        let r_back = match read_back.channel("R") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float R, got {other:?}"),
        };
        let g_back = match read_back.channel("G") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float G, got {other:?}"),
        };
        let b_back = match read_back.channel("B") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float B, got {other:?}"),
        };
        let a_back = match read_back.channel("A") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float A, got {other:?}"),
        };
        let z_back = match read_back.channel("depth.Z") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float depth.Z, got {other:?}"),
        };
        let mx_back = match read_back.channel("motion.X") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float motion.X, got {other:?}"),
        };
        let my_back = match read_back.channel("motion.Y") {
            Some(ChannelData::Float(v)) => v,
            other => panic!("expected Float motion.Y, got {other:?}"),
        };
        let sem_back = match read_back.channel("semantic.id") {
            Some(ChannelData::Uint(v)) => v,
            other => panic!("expected Uint semantic.id, got {other:?}"),
        };
        let inst_back = match read_back.channel("instance.id") {
            Some(ChannelData::Uint(v)) => v,
            other => panic!("expected Uint instance.id, got {other:?}"),
        };

        for i in 0..n {
            assert_eq!(r_back[i].to_bits(), color_rgba[i][0].to_bits());
            assert_eq!(g_back[i].to_bits(), color_rgba[i][1].to_bits());
            assert_eq!(b_back[i].to_bits(), color_rgba[i][2].to_bits());
            assert_eq!(a_back[i].to_bits(), color_rgba[i][3].to_bits());
            assert_eq!(z_back[i].to_bits(), depth[i].to_bits());
            assert_eq!(mx_back[i].to_bits(), motion[i][0].to_bits());
            assert_eq!(my_back[i].to_bits(), motion[i][1].to_bits());
            assert_eq!(sem_back[i], semantic_id[i]);
            assert_eq!(inst_back[i], instance_id[i]);
        }
    }

    #[test]
    fn sdg_burst_manifest_atomic_roundtrip() {
        let manifest = SdgBurstManifest {
            schema_version: "1.0".to_string(),
            scene_id: "inspector".to_string(),
            scene_idx: 5,
            variant_idx: 109,
            width: 896,
            height: 512,
            spp_per_frame: 8,
            projection: "Pinhole".to_string(),
            requested_occluder: "Auto".to_string(),
            crowd_backend: "CUDA".to_string(),
            exr_channels: SDG_EXR_CHANNELS.iter().map(|s| (*s).to_string()).collect(),
            frames: vec![SdgBurstFrameEntry {
                burst_index: 0,
                render_frame_index: 42,
                time_s: 0.5,
                exr_file: "frame_0000.exr".to_string(),
                json_file: "frame_0000.sdg.json".to_string(),
                actor_count: 4,
                occluder_used: "OptiX".to_string(),
            }],
        };
        let path = std::env::temp_dir().join("nv_demo_sdg_manifest_test.json");
        write_sdg_manifest_atomic(&path, &manifest).expect("write manifest");
        let raw = std::fs::read_to_string(&path).expect("read manifest");
        let _ = std::fs::remove_file(&path);
        let back: SdgBurstManifest = serde_json::from_str(&raw).expect("parse manifest");
        assert_eq!(back, manifest);
    }
}
