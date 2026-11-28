//! Multi-AOV compressed `.exv` (EXR Video) stream recorder for Synthetic Data Generation (SDG).
//!
//! - **What**: defines the canonical 16-channel multi-AOV schema ([`EXV_CHANNELS`]), SDG encoding
//!   presets ([`ExvProfile`]: `Compact`, `Hybrid`, `Lossless`, `Fast`), OpenEXR/Cryptomatte
//!   metadata builder ([`ExvClipMetadata`], [`build_exv_meta`]), a bounded zero-churn host
//!   readback buffer pool ([`ExvFrameBuffers`], [`ExvFrameSlot`]), and a background-threaded
//!   streaming writer ([`ExvRecorder`]) supporting both standard `exv_video::ClipWriter` clips and
//!   zero-FFmpeg `CodecId::HYBRID` containers via `exv_video::HybridClipWriter` (10-bit PQ10
//!   Rec.2020 NVENC HEVC ISO-BMFF segment + 24-byte `HYBV` Castagnoli CRC-32C locator + 13
//!   bit-exact lossless `Wavelet53` AOV channels) with embedded `FMET`/`MetaFooter` per-frame &
//!   clip-level `RustBox` metadata and atomic temp-file finalization (one self-contained `.exv`,
//!   zero sibling `.sdg.json` sidecar files).
//! - **Why**: replaces raw per-frame EXR bursts and loose JSON sidecars for continuous synthetic
//!   training capture with a single temporally compressed, seek-indexed `.exv` stream where
//!   `semantic.id` and `instance.id` (`U32`) remain 100% bit-exact lossless and per-frame SDG
//!   camera/keypoint metadata is O(1) random-accessible without decoding pixels, while keeping GPU
//!   rendering unblocked and steady-state render-thread heap allocations at zero.
//! - **How**: [`ExvRecorder::start`] pre-allocates [`EXV_POOL_SLOTS`] (`3`) [`ExvFrameBuffers`]
//!   bundles and opens a temporary `.exv.tmp` file in the target directory. The render thread
//!   leases an [`ExvFrameSlot`] (`acquire_slot` for backpressure or `try_acquire_slot` for live
//!   drop-on-full), DMAs GPU AOVs directly into `slot.bufs`, optionally attaches a typed
//!   [`crate::csg_scene_host::SdgExportFrame`], and submits it over a bounded `sync_channel`. The
//!   background worker packs planes via one shared SSOT helper (`pack_non_rgb_aov_planes`), drops
//!   the slot immediately so its RAII `Drop` returns `bufs` to the pool, converts any attached SDG
//!   record into a `RustBox`, and streams into `ClipWriter::push` / `push_with_meta` (standard
//!   profiles) or `HybridClipWriter::push_aov_planes` / `push_aov_planes_with_meta` (13 lossless
//!   `Wavelet53` AOVs). On [`ExvRecorder::finish`] / [`ExvRecorder::finish_hybrid`], queued frames
//!   drain, clip-level `clip_box` metadata is attached, Hybrid streams the temp `.mp4` in 256 KiB
//!   chunks via `HybridClipWriter::finish_with_video_file` (`FMET` + `HYBV` + CRC-32C),
//!   `sync_all()` flushes the OS page cache, handles close, and the temp file is atomically
//!   renamed to the final `.exv`.
//! - **Uses**: `exv_format`, `exv_codec`, `exv_video`, `exr_core`, `imath_rs`, `csg_core`,
//!   `crate::csg_scene_host`, `crate::sdg_export`, `crate::semantic_filter`.
//! - **Used by**: `crate::renderer`, `crate::app_events`, `crate::persist`, `crate::ui`,
//!   `crate::main`.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

use csg_core::ast::semantic;
use exv_codec::image::Image;
use exv_codec::{
    ChromaSubsampling, HybridParams, HybridPrimaries, HybridTransfer, HybridVideoCodec, Profile,
};
use exv_format::half::f32_to_half;
use exv_format::{Channel, CodecId, Dtype, Header, Meta};
use exv_video::{BoxValue, ClipWriter, HybridClipWriter, RustBox, WriteOpts};
use serde::{Deserialize, Serialize};

use crate::csg_scene_host::SdgExportFrame;
use crate::sdg_export::ActiveSdgOccluder;
use crate::semantic_filter::semantic_name;

/// Number of AOV channels written into every `nv-demo` `.exv` clip.
pub const EXV_CHANNEL_COUNT: usize = 16;

/// Number of non-RGB AOV channels carried in the tiled lossless track of [`ExvProfile::Hybrid`].
pub const EXV_HYBRID_AOV_COUNT: usize = EXV_CHANNEL_COUNT - 3;

/// Canonical multi-AOV channel specification `(name, dtype)` written into every `.exv` clip.
///
/// Indices `0..3` are always top-level `"R"`, `"G"`, `"B"` (`F16`), matching [`EXV_RGB_INDICES`]
/// and `exv_format::Header::rgb_triad` so RGB-bound perceptual `YCbCr` (`Compact`), Hybrid HEVC
/// (`Hybrid`), and EXR/EXV viewers resolve primary linear Beauty colour deterministically.
pub const EXV_CHANNELS: [(&str, Dtype); EXV_CHANNEL_COUNT] = [
    ("R", Dtype::F16),
    ("G", Dtype::F16),
    ("B", Dtype::F16),
    ("A", Dtype::F16),
    ("albedo.R", Dtype::F16),
    ("albedo.G", Dtype::F16),
    ("albedo.B", Dtype::F16),
    ("normal.X", Dtype::F16),
    ("normal.Y", Dtype::F16),
    ("normal.Z", Dtype::F16),
    ("roughness", Dtype::F16),
    ("depth.Z", Dtype::F32),
    ("motion.X", Dtype::F32),
    ("motion.Y", Dtype::F32),
    ("semantic.id", Dtype::U32),
    ("instance.id", Dtype::U32),
];

/// Channel indices of `("R", "G", "B")` inside [`EXV_CHANNELS`].
pub const EXV_RGB_INDICES: [u16; 3] = [0, 1, 2];

/// Number of reusable host AOV frame slots circulating between the render thread and EXV encoder.
pub const EXV_POOL_SLOTS: usize = 3;

/// Default GOP length (I-frame cadence in display frames) for streaming `.exv` clips.
pub const EXV_DEFAULT_GOP: u32 = 16;

/// Default global motion-vector search radius in pixels for streaming `.exv` clips.
pub const EXV_DEFAULT_SEARCH_RADIUS: i32 = 4;

/// Default perceptual HTJ2K 9/7 quality (`1..=40`) for [`ExvProfile::Compact`].
pub const EXV_COMPACT_HDR97_Q: u8 = 20;

/// Nominal quality/CRF metadata tag recorded in [`HybridParams::crf`] for [`ExvProfile::Hybrid`].
pub const EXV_HYBRID_CRF: u8 = 22;

/// Channel slot indices inside [`EXV_CHANNELS`] — locked at compile time so packing never drifts.
const CH_R: usize = 0;
const CH_G: usize = 1;
const CH_B: usize = 2;
const CH_A: usize = 3;
const CH_ALBEDO_R: usize = 4;
const CH_ALBEDO_G: usize = 5;
const CH_ALBEDO_B: usize = 6;
const CH_NORMAL_X: usize = 7;
const CH_NORMAL_Y: usize = 8;
const CH_NORMAL_Z: usize = 9;
const CH_ROUGHNESS: usize = 10;
const CH_DEPTH_Z: usize = 11;
const CH_MOTION_X: usize = 12;
const CH_MOTION_Y: usize = 13;
const CH_SEMANTIC_ID: usize = 14;
const CH_INSTANCE_ID: usize = 15;

/// Size of the `BufWriter` buffer (1 MiB) wrapping the streaming `.exv` output file.
const EXV_WRITE_BUF_BYTES: usize = 1 << 20;

const _: () = {
    assert!(EXV_RGB_INDICES[0] as usize == CH_R);
    assert!(EXV_RGB_INDICES[1] as usize == CH_G);
    assert!(EXV_RGB_INDICES[2] as usize == CH_B);
    assert!(CH_INSTANCE_ID + 1 == EXV_CHANNEL_COUNT);
    assert!(EXV_HYBRID_AOV_COUNT == 13);
    assert!(EXV_POOL_SLOTS >= 2);
};

/// Compression preset for multi-AOV `.exv` Synthetic Data Generation stream recording.
///
/// Every preset preserves `semantic.id` and `instance.id` (`Dtype::U32`) 100% bit-exact lossless.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExvProfile {
    /// Highest-compression pure-Rust streaming profile: perceptual HDR HTJ2K 9/7 (`q = 20`,
    /// `4:2:0` perceptual `YCbCr` on Beauty RGB, full-res float AOVs, `512x512` tiles, closed-loop
    /// temporal I/P `gop = 16`, `search = 4`, zero cumulative GOP drift). `semantic.id` and
    /// `instance.id` (`U32`) remain 100% bit-exact lossless.
    #[default]
    Compact,
    /// Ultra-compact Hybrid `.exv` (`CodecId::HYBRID`): hardware 10-bit PQ10 Rec.2020 NVENC HEVC
    /// for linear Beauty RGB embedded in a contiguous ISO-BMFF segment (`HYBV` + CRC-32C)
    /// alongside 100% bit-exact lossless tiled `Wavelet53` (`256x256`) for all 13 non-RGB AOVs
    /// (`A`, `albedo`, `normal`, `roughness`, `depth.Z`, `motion`, `semantic.id`, `instance.id`).
    /// Zero FFmpeg dependency; requires NVENC (never silently downgrades).
    Hybrid,
    /// 100% bit-exact lossless reversible 5/3 wavelet (`Wavelet53`) + temporal I/P (`gop = 16`,
    /// `search = 4`, `256x256` tiles). GPU-decodable in `exv-gpu` / `exv-cuda` / `exv-dlpack`.
    Lossless,
    /// Fastest 100% bit-exact lossless streaming encoder (`LosslessFast`: reorder+delta+rANS) +
    /// temporal I/P (`gop = 16`, `search = 4`, `256x256` tiles). GPU-decodable.
    Fast,
}

impl ExvProfile {
    /// All selectable EXV recording profiles in UI order.
    pub const ALL: [Self; 4] = [Self::Compact, Self::Hybrid, Self::Lossless, Self::Fast];

    /// Stable machine-readable identifier stored in `.exv` OpenEXR metadata.
    #[inline]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Compact => "sdg_compact",
            Self::Hybrid => "sdg_hybrid",
            Self::Lossless => "sdg_lossless",
            Self::Fast => "sdg_fast",
        }
    }

    /// Compact human-readable label for UI combo boxes and status badges.
    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Compact => "Compact (HTJ2K 9/7)",
            Self::Hybrid => "Hybrid (NVENC+AOV)",
            Self::Lossless => "Lossless (Wavelet)",
            Self::Fast => "Fast (Lossless)",
        }
    }

    /// Plain-English tooltip describing compression tradeoffs and ML/SDG ground-truth guarantees.
    #[inline]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Compact => {
                "Smallest pure-Rust files: perceptual HDR HTJ2K 9/7 (4:2:0 Beauty, full-res float \
                 AOVs, closed-loop temporal GOP). U32 semantic/instance IDs stay 100% bit-exact \
                 lossless."
            }
            Self::Hybrid => {
                "Ultra-compact Hybrid EXV: 10-bit PQ10 Rec.2020 NVENC HEVC for Beauty RGB + 100% \
                 bit-exact lossless Wavelet53 for all 13 non-RGB AOVs (A, Albedo, Normal, Depth, \
                 Motion, U32 IDs) in one self-contained .exv. Zero FFmpeg; requires NVENC."
            }
            Self::Lossless => {
                "100% bit-exact ground truth across all 16 AOVs (Beauty, Albedo, Normal, Depth, \
                 Motion, U32 IDs) with reversible 5/3 wavelet + temporal GOP. GPU-decodable in \
                 exv-cuda / exv-dlpack."
            }
            Self::Fast => {
                "Fastest 100% bit-exact lossless capture (reorder+delta+rANS + temporal GOP) for \
                 high-FPS SDG recording. Larger than Wavelet; GPU-decodable in exv-cuda / \
                 exv-dlpack."
            }
        }
    }

    /// Tile edge size in pixels for this profile.
    #[inline]
    pub const fn tile_size(self) -> u32 {
        match self {
            Self::Compact => 512,
            Self::Hybrid | Self::Lossless | Self::Fast => 256,
        }
    }

    /// GOP length (I-frame cadence in display frames).
    #[inline]
    pub const fn gop(self) -> u32 {
        EXV_DEFAULT_GOP
    }

    /// Global motion-vector search radius in pixels.
    #[inline]
    pub const fn search_radius(self) -> i32 {
        EXV_DEFAULT_SEARCH_RADIUS
    }

    /// Returns `true` when every channel (including float Beauty, depth, and motion) is bit-exact.
    #[inline]
    pub const fn is_lossless(self) -> bool {
        match self {
            Self::Compact | Self::Hybrid => false,
            Self::Lossless | Self::Fast => true,
        }
    }

    /// Returns `true` for [`Self::Hybrid`] (10-bit NVENC PQ10 HEVC Beauty RGB + lossless AOVs).
    #[inline]
    pub const fn is_hybrid(self) -> bool {
        matches!(self, Self::Hybrid)
    }

    /// Canonical [`HybridParams`] wire payload for [`Self::Hybrid`] (`H.265`, `PQ`, `Rec.2020`,
    /// `Wavelet53` lossless for all 13 non-RGB AOVs).
    pub fn hybrid_params() -> HybridParams {
        let aov_profile = Profile::lossless_wave();
        HybridParams {
            video_codec: HybridVideoCodec::H265,
            transfer: HybridTransfer::Pq,
            primaries: HybridPrimaries::Rec2020,
            crf: EXV_HYBRID_CRF,
            rgb: EXV_RGB_INDICES,
            aov_codec_id: aov_profile.codec_id(),
            aov_codec_params: aov_profile.params(),
        }
    }

    /// Builds the `exv_codec::Profile` bound to the canonical [`EXV_CHANNELS`] RGB triad.
    ///
    /// For [`Self::Hybrid`], returns the lossless AOV tile codec (`Profile::lossless_wave()`);
    /// the top-level container header carries [`CodecId::HYBRID`] + [`Self::hybrid_params`].
    pub fn to_codec_profile(self) -> Profile {
        match self {
            Self::Compact => {
                let mut profile =
                    Profile::htj2k_hdr97_chroma(EXV_COMPACT_HDR97_Q, ChromaSubsampling::C420);
                // `EXV_RGB_INDICES = [0, 1, 2]` are pairwise distinct (pinned by compile-time assert).
                if let Err(err) = profile.bind_rgb(Some(EXV_RGB_INDICES)) {
                    debug_assert!(false, "canonical EXV RGB indices rejected by bind_rgb: {err}");
                }
                profile
            }
            Self::Hybrid | Self::Lossless => Profile::lossless_wave(),
            Self::Fast => Profile::lossless_fast(),
        }
    }

    /// Builds and validates the `exv_codec::Profile` against a concrete [`Header`], dynamically
    /// binding `Header::rgb_triad(&header.channels)` when required.
    pub fn resolve_codec_profile(self, header: &Header) -> Result<Profile, String> {
        let mut profile = self.to_codec_profile();
        if !self.is_hybrid() {
            let triad = Header::rgb_triad(&header.channels);
            profile
                .bind_rgb(triad)
                .map_err(|e| format!("EXV profile RGB binding failed: {e}"))?;
        }
        Ok(profile)
    }

    /// Builds streaming [`WriteOpts`] (`b_frames = 0` as required by `ClipWriter::push`).
    pub fn write_opts(self, meta: Meta) -> WriteOpts {
        WriteOpts {
            gop: self.gop(),
            search: self.search_radius(),
            b_frames: 0,
            meta,
        }
    }
}

/// High-level clip metadata embedded into `.exv` (`exv_format::Meta` + OpenEXR header attributes).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExvClipMetadata {
    /// Playback frame rate as exact rational `(num, den)` (e.g. `(60, 1)` or `(24000, 1001)`).
    pub fps: (u32, u32),
    /// Active scene identifier (e.g. `"gundo_osm"`, `"marina"`, `"inspector"`, `"classic"`).
    pub scene_id: String,
    /// Optical sensor projection name (e.g. `"Pinhole"`, `"Fisheye180"`).
    pub projection: String,
    /// Recording profile used for this clip.
    pub profile: ExvProfile,
    /// `true` when linear Beauty (`R/G/B/A`) came from the denoiser output; `false` for raw PT.
    pub beauty_denoised: bool,
    /// Optional object/instance manifest entries `(name, instance_id)` stored as OpenEXR
    /// `cryptomatte/<id>/manifest` JSON (`{"name": "0123abcd", ...}`).
    pub instance_manifest: Vec<(String, u32)>,
}

/// Builds the canonical 16-channel EXV [`Header`] for `(width, height)` and `profile`.
///
/// The returned header carries `frame_count = 1` (single-frame shape template) and the exact
/// `codec_id` / `codec_params` of `profile` (including [`CodecId::HYBRID`] + [`HybridParams`] for
/// [`ExvProfile::Hybrid`]).
pub fn build_exv_header(width: u32, height: u32, profile: ExvProfile) -> Result<Header, String> {
    if width == 0 || height == 0 {
        return Err(format!("EXV dimensions ({width}x{height}) must be non-zero"));
    }
    let tile = profile.tile_size();
    let channels: Vec<Channel> = EXV_CHANNELS
        .iter()
        .map(|&(name, dtype)| Channel {
            name: name.to_string(),
            dtype,
        })
        .collect();
    let mut header = Header {
        width,
        height,
        frame_count: 1,
        tile_w: tile,
        tile_h: tile,
        codec_id: 0,
        codec_params: Vec::new(),
        channels,
    };
    if profile.is_hybrid() {
        let params = ExvProfile::hybrid_params();
        header.codec_id = CodecId::HYBRID.0;
        header.codec_params = params
            .try_encode()
            .map_err(|e| format!("EXV HybridParams encode failed: {e}"))?;
    } else {
        let codec = profile.resolve_codec_profile(&header)?;
        header.codec_id = codec.codec_id().0;
        header.codec_params = codec.params();
    }
    for ch in &header.channels {
        header
            .plane_len(ch.dtype)
            .ok_or_else(|| format!("EXV plane size overflows for {width}x{height} {:?}", ch.dtype))?;
    }
    Ok(header)
}

/// Computes standard MurmurHash3-32 (`seed = 0`) over arbitrary bytes for Cryptomatte namespace IDs.
fn murmur3_32_bytes(data: &[u8]) -> u32 {
    const C1: u32 = 0xcc9e_2d51;
    const C2: u32 = 0x1b87_3593;
    let mut h = 0u32;
    let (chunks, rem) = data.as_chunks::<4>();
    for &chunk in chunks {
        let mut k = u32::from_le_bytes(chunk);
        k = k.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);
        h ^= k;
        h = h.rotate_left(13).wrapping_mul(5).wrapping_add(0xe654_6b64);
    }
    let mut k1 = 0u32;
    if rem.len() >= 3 {
        k1 ^= u32::from(rem[2]) << 16;
    }
    if rem.len() >= 2 {
        k1 ^= u32::from(rem[1]) << 8;
    }
    if !rem.is_empty() {
        k1 ^= u32::from(rem[0]);
        k1 = k1.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);
        h ^= k1;
    }
    h ^= data.len() as u32;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^ (h >> 16)
}

/// Deterministic 7-hex-character Cryptomatte namespace identifier for the `"instance.id"` channel
/// (first 7 hex digits of `MurmurHash3_32("instance.id")`, per the OpenEXR Cryptomatte spec).
pub fn instance_cryptomatte_id() -> String {
    let hex = format!("{:08x}", murmur3_32_bytes(b"instance.id"));
    hex[..7].to_string()
}

/// Builds a self-describing [`exv_format::Meta`] embedding exact `fps`, pixel aspect, associated
/// alpha intent, and a full native OpenEXR header (`Meta::exr_headers`) carrying:
/// - All 16 channel descriptors and `framesPerSecond`
/// - `software`, `nv_demo/scene_id`, `nv_demo/projection`, `nv_demo/exv_profile`,
///   `nv_demo/exv_lossless`, `nv_demo/beauty_source`
/// - `nv_demo/semantic_map` JSON mapping every `csg_core::ast::semantic` ID to its canonical name
/// - OpenEXR Cryptomatte attributes (`cryptomatte/<id>/{name,hash,conversion,manifest}`) for
///   `instance.id`
pub fn build_exv_meta(width: u32, height: u32, info: &ExvClipMetadata) -> Result<Meta, String> {
    if width == 0 || height == 0 {
        return Err(format!("EXV meta dimensions ({width}x{height}) must be non-zero"));
    }
    let (fps_num, fps_den) = info.fps;
    if fps_num == 0 || fps_den == 0 {
        return Err(format!("EXV meta fps ({fps_num}/{fps_den}) must be non-zero"));
    }
    let w_i32 = i32::try_from(width).map_err(|_| "EXV width exceeds i32::MAX".to_string())?;
    let h_i32 = i32::try_from(height).map_err(|_| "EXV height exceeds i32::MAX".to_string())?;
    let fps_num_i32 =
        i32::try_from(fps_num).map_err(|_| "EXV fps numerator exceeds i32::MAX".to_string())?;

    let mut exr = exr_core::header::Header::with_dimensions(
        w_i32,
        h_i32,
        1.0,
        imath_rs::V2f::new(0.0, 0.0),
        1.0,
        exr_core::attr::LineOrder::IncreasingY,
        exr_core::attr::Compression::Zip,
    )
    .map_err(|e| format!("EXR metadata header init failed: {e}"))?;

    for &(name, dtype) in &EXV_CHANNELS {
        let pt = match dtype {
            Dtype::F16 => exr_core::attr::PixelType::Half,
            Dtype::F32 | Dtype::Bf16 => exr_core::attr::PixelType::Float,
            Dtype::U8 | Dtype::U16 | Dtype::U32 => exr_core::attr::PixelType::Uint,
        };
        exr.channels_mut()
            .map_err(|e| format!("EXR channel list access failed: {e}"))?
            .insert(name, exr_core::attr::Channel::new(pt, 1, 1, false))
            .map_err(|e| format!("EXR channel `{name}` insert failed: {e}"))?;
    }

    exr.add_frames_per_second(exr_core::attr::Rational::new(fps_num_i32, fps_den))
        .map_err(|e| format!("EXR framesPerSecond insert failed: {e}"))?;

    let mut insert_str = |key: &str, value: String| -> Result<(), String> {
        exr.insert(key, exr_core::attr::StringAttribute::new(value.into()))
            .map_err(|e| format!("EXR attribute `{key}` insert failed: {e}"))
    };

    insert_str("software", "nv-demo (nv-rs)".to_string())?;
    insert_str("nv_demo/scene_id", info.scene_id.clone())?;
    insert_str("nv_demo/projection", info.projection.clone())?;
    insert_str("nv_demo/exv_profile", info.profile.id().to_string())?;
    insert_str(
        "nv_demo/exv_lossless",
        if info.profile.is_lossless() { "true" } else { "false" }.to_string(),
    )?;
    insert_str(
        "nv_demo/beauty_source",
        if info.beauty_denoised { "denoised" } else { "raw" }.to_string(),
    )?;

    // Canonical semantic ID -> display name map sourced from `csg_core::ast::semantic::COUNT`.
    let mut sem_map = serde_json::Map::with_capacity(usize::from(semantic::COUNT));
    for id in 0..semantic::COUNT {
        sem_map.insert(
            id.to_string(),
            serde_json::Value::String(semantic_name(id).to_string()),
        );
    }
    let sem_json = serde_json::to_string(&sem_map)
        .map_err(|e| format!("semantic map JSON serialize failed: {e}"))?;
    insert_str("nv_demo/semantic_map", sem_json)?;

    // Deterministic OpenEXR Cryptomatte manifest for `instance.id`.
    let crypto_id = instance_cryptomatte_id();
    insert_str(
        &format!("cryptomatte/{crypto_id}/name"),
        "instance.id".to_string(),
    )?;
    insert_str(
        &format!("cryptomatte/{crypto_id}/hash"),
        "uint32_instance_id".to_string(),
    )?;
    insert_str(
        &format!("cryptomatte/{crypto_id}/conversion"),
        "uint32_to_uint32".to_string(),
    )?;

    let mut manifest_sorted: BTreeMap<&str, String> = BTreeMap::new();
    for (name, id) in &info.instance_manifest {
        if !name.is_empty() {
            manifest_sorted.insert(name.as_str(), format!("{id:08x}"));
        }
    }
    let manifest_json = serde_json::to_string(&manifest_sorted)
        .map_err(|e| format!("cryptomatte manifest JSON serialize failed: {e}"))?;
    insert_str(&format!("cryptomatte/{crypto_id}/manifest"), manifest_json)?;

    let mut meta = Meta {
        fps: Some((fps_num, fps_den)),
        primaries: 0,
        transfer: 0,
        range: 0,
        alpha: 2, // associated / premultiplied linear RGBA
        pixel_aspect: Some((1, 1)),
        timecode: None,
        exr_headers: Vec::new(),
    };
    meta.set_exr_headers(&[exr])
        .map_err(|e| format!("EXV Meta::set_exr_headers failed: {e}"))?;
    Ok(meta)
}

/// Generates a unique, sanitized `.exv` output path under `renders/` (e.g.
/// `renders/nv_demo_gundo_osm_1764289123456.exv`).
pub fn default_exv_output_path(scene_id: &str) -> PathBuf {
    let safe_scene: String = scene_id
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    let tag = if safe_scene.is_empty() { "scene" } else { safe_scene.as_str() };
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    PathBuf::from("renders").join(format!("nv_demo_{tag}_{stamp}.exv"))
}

/// Returns the sibling temporary `.mp4` path used to stage the NVENC 10-bit PQ10 HEVC stream
/// before [`ExvRecorder::finish_hybrid`] embeds it into `exv_path`.
pub fn hybrid_temp_mp4_path(exv_path: &Path) -> PathBuf {
    exv_path.with_extension("hybrid.tmp.mp4")
}

/// Reusable host-side AOV readback buffer bundle for one frame (all 7 GPU readback streams).
///
/// Pre-allocated once per pool slot so steady-state GPU-to-host readback performs zero heap
/// allocations on the render thread.
#[derive(Debug, Default)]
pub struct ExvFrameBuffers {
    /// Linear Beauty RGBA (`width * height`; denoised when enabled, else raw path-traced).
    pub color_rgba: Vec<[f32; 4]>,
    /// Linear Albedo RGBA (`width * height`; `.rgb` written to `albedo.{R,G,B}`).
    pub albedo_rgba: Vec<[f32; 4]>,
    /// World/view-space shading normal (`.xyz`) + roughness (`.w`) (`width * height`).
    pub normal_rough: Vec<[f32; 4]>,
    /// `depth.Z` in meters (`width * height`).
    pub depth: Vec<f32>,
    /// Backward pixel optical flow `[dx, dy]` (`width * height`).
    pub motion: Vec<[f32; 2]>,
    /// Per-pixel semantic class ID (`width * height`).
    pub semantic_id: Vec<u32>,
    /// Per-pixel logical/instance ID (`width * height`).
    pub instance_id: Vec<u32>,
    /// Optional per-frame SDG ground-truth record (camera intrinsics/extrinsics + ray-occluded
    /// articulated actor keypoints/BBoxes, reusing [`SdgExportFrame`] directly). When populated on
    /// a frame, [`ExvRecorder`] embeds it as a compressed `RustBox` row inside the `.exv`
    /// container's `FMET`/`MetaFooter` `"frames"` table (no sibling `.sdg.json` file is emitted).
    pub sdg_frame: Option<SdgExportFrame>,
    /// Concrete ray-occlusion backend (`OptiX` or `CPU`) that classified `sdg_frame`, if any.
    pub sdg_occluder: Option<ActiveSdgOccluder>,
}

impl ExvFrameBuffers {
    /// Allocates all 7 host AOV buffers sized exactly for `width * height` pixels.
    pub fn with_dims(width: u32, height: u32) -> Result<Self, String> {
        let n = checked_pixel_count(width, height)?;
        Ok(Self {
            color_rgba: vec![[0.0; 4]; n],
            albedo_rgba: vec![[0.0; 4]; n],
            normal_rough: vec![[0.0; 4]; n],
            depth: vec![0.0; n],
            motion: vec![[0.0; 2]; n],
            semantic_id: vec![0; n],
            instance_id: vec![0; n],
            sdg_frame: None,
            sdg_occluder: None,
        })
    }

    /// Validates that all 7 AOV buffers match `expected` pixel count.
    fn validate_len(&self, expected: usize) -> Result<(), String> {
        if self.color_rgba.len() != expected
            || self.albedo_rgba.len() != expected
            || self.normal_rough.len() != expected
            || self.depth.len() != expected
            || self.motion.len() != expected
            || self.semantic_id.len() != expected
            || self.instance_id.len() != expected
        {
            return Err(format!(
                "EXV frame buffer length mismatch: expected {expected}, got color={} albedo={} \
                 normal={} depth={} motion={} sem={} inst={}",
                self.color_rgba.len(),
                self.albedo_rgba.len(),
                self.normal_rough.len(),
                self.depth.len(),
                self.motion.len(),
                self.semantic_id.len(),
                self.instance_id.len(),
            ));
        }
        Ok(())
    }
}

/// RAII lease over a pooled [`ExvFrameBuffers`] slot.
///
/// Fill `slot.bufs` directly (e.g. via `cuMemcpyDtoH_v2`) and pass the slot to
/// [`ExvRecorder::submit_slot`]. Dropping a slot on any path (after encoding or on an early error)
/// returns its buffers to the pool automatically without reallocating; panics discard the slot.
pub struct ExvFrameSlot {
    /// Pooled host AOV readback buffers owned by this lease.
    pub bufs: ExvFrameBuffers,
    recycle: Option<mpsc::SyncSender<ExvFrameBuffers>>,
}

impl Drop for ExvFrameSlot {
    fn drop(&mut self) {
        if !std::thread::panicking()
            && let Some(tx) = self.recycle.take()
        {
            let mut bufs = std::mem::take(&mut self.bufs);
            bufs.sdg_frame = None;
            bufs.sdg_occluder = None;
            if tx.try_send(bufs).is_err() {
                // Pool receiver already dropped during recorder shutdown; drop buffers cleanly.
            }
        }
    }
}

/// Borrowed host-side AOV slices for submitting a single frame in unit tests.
#[cfg(test)]
struct ExvFrameInput<'a> {
    width: u32,
    height: u32,
    color_rgba: &'a [[f32; 4]],
    albedo_rgba: &'a [[f32; 4]],
    normal_rough: &'a [[f32; 4]],
    depth: &'a [f32],
    motion: &'a [[f32; 2]],
    semantic_id: &'a [u32],
    instance_id: &'a [u32],
    sdg_frame: Option<SdgExportFrame>,
    sdg_occluder: Option<ActiveSdgOccluder>,
}

/// Live atomic snapshot of EXV stream recording progress.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExvStats {
    /// Total frames handed to the background encoder queue.
    pub frames_submitted: u64,
    /// Total frames compressed and written into the `.exv` stream.
    pub frames_encoded: u64,
    /// Total live frames skipped because all pool slots were busy (via [`ExvRecorder::note_dropped_frame`]).
    pub frames_dropped: u64,
    /// Bytes written to the `.exv` stream so far (finalized total includes index + trailer).
    pub bytes_written: u64,
}

/// Summary returned by [`ExvRecorder::finish`] / [`ExvRecorder::finish_hybrid`] once the `.exv`
/// file is synced and atomically renamed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExvFinishReport {
    /// Final `.exv` file path on disk (self-contained with embedded `FMET`/`MetaFooter`).
    pub path: PathBuf,
    /// Number of frames carrying embedded per-frame `RustBox` SDG metadata in `MetaFooter` `"frames"`.
    pub meta_frames_embedded: u64,
    /// Total frames encoded into the `.exv` stream.
    pub frames_encoded: u64,
    /// Total frames skipped via live drop-on-full.
    pub frames_dropped: u64,
    /// Final `.exv` file size in bytes.
    pub bytes_written: u64,
}

/// Shared atomic telemetry, error state, and Hybrid finalize handoff between [`ExvRecorder`] and
/// its background worker thread.
struct SharedRecorderState {
    frames_submitted: AtomicU64,
    frames_encoded: AtomicU64,
    frames_dropped: AtomicU64,
    bytes_written: AtomicU64,
    abort_on_close: AtomicBool,
    hybrid_video_mp4: Mutex<Option<PathBuf>>,
    last_error: Mutex<Option<String>>,
}

impl SharedRecorderState {
    fn new() -> Self {
        Self {
            frames_submitted: AtomicU64::new(0),
            frames_encoded: AtomicU64::new(0),
            frames_dropped: AtomicU64::new(0),
            bytes_written: AtomicU64::new(0),
            abort_on_close: AtomicBool::new(true),
            hybrid_video_mp4: Mutex::new(None),
            last_error: Mutex::new(None),
        }
    }

    fn set_error(&self, msg: String) {
        if let Ok(mut guard) = self.last_error.lock()
            && guard.is_none()
        {
            *guard = Some(msg);
        }
    }

    fn get_error(&self) -> Option<String> {
        self.last_error.lock().ok().and_then(|g| g.clone())
    }

    fn set_hybrid_video_mp4(&self, path: PathBuf) -> Result<(), String> {
        let mut guard = self
            .hybrid_video_mp4
            .lock()
            .map_err(|_| "EXV hybrid finalize mutex poisoned".to_string())?;
        *guard = Some(path);
        Ok(())
    }

    fn take_hybrid_video_mp4(&self) -> Option<PathBuf> {
        self.hybrid_video_mp4.lock().ok().and_then(|mut g| g.take())
    }

    fn snapshot(&self) -> ExvStats {
        ExvStats {
            frames_submitted: self.frames_submitted.load(Ordering::Acquire),
            frames_encoded: self.frames_encoded.load(Ordering::Acquire),
            frames_dropped: self.frames_dropped.load(Ordering::Acquire),
            bytes_written: self.bytes_written.load(Ordering::Acquire),
        }
    }
}

/// Builds a per-frame [`RustBox`] row stored in the `.exv` `FMET`/`MetaFooter` `"frames"` table.
///
/// Stores typed top-level camera & frame attributes (`camera.eye` as `BoxValue::Vec3`, `time_s`,
/// `occluder_used`, `scene_id`, etc.) for O(1) typed inspection alongside the complete lossless
/// [`SdgExportFrame`] under `"sdg.frame"`.
fn build_frame_meta_box(
    exv_frame_index: u64,
    sdg: &SdgExportFrame,
    occluder: Option<ActiveSdgOccluder>,
) -> Result<RustBox, String> {
    let mut b = RustBox::new();
    let exv_idx_i64 = i64::try_from(exv_frame_index)
        .map_err(|_| "EXV frame index exceeds i64::MAX".to_string())?;
    let actor_count_i64 = i64::try_from(sdg.actors.len())
        .map_err(|_| "EXV actor count exceeds i64::MAX".to_string())?;

    let err_map = |k: &'static str| move |e| format!("EXV frame metadata `{k}` set failed: {e}");

    b.set_i64("exv_frame_index", exv_idx_i64)
        .map_err(err_map("exv_frame_index"))?;
    b.set_str("scene_id", sdg.scene_id.as_str())
        .map_err(err_map("scene_id"))?;
    b.set_i64("scene_idx", i64::from(sdg.scene_idx))
        .map_err(err_map("scene_idx"))?;
    b.set_i64("variant_idx", i64::from(sdg.variant_idx))
        .map_err(err_map("variant_idx"))?;
    b.set_i64("sim_frame_index", i64::from(sdg.frame_index))
        .map_err(err_map("sim_frame_index"))?;
    b.set_f64("time_s", f64::from(sdg.time_s))
        .map_err(err_map("time_s"))?;
    if let Some(occ) = occluder {
        b.set_str("occluder_used", occ.label())
            .map_err(err_map("occluder_used"))?;
    }
    b.set_str("camera.projection", format!("{:?}", sdg.camera.projection))
        .map_err(err_map("camera.projection"))?;
    b.set_path(
        "camera.eye",
        BoxValue::Vec3([
            f64::from(sdg.camera.eye.x),
            f64::from(sdg.camera.eye.y),
            f64::from(sdg.camera.eye.z),
        ]),
    )
    .map_err(err_map("camera.eye"))?;
    b.set_path(
        "camera.forward",
        BoxValue::Vec3([
            f64::from(sdg.camera.forward.x),
            f64::from(sdg.camera.forward.y),
            f64::from(sdg.camera.forward.z),
        ]),
    )
    .map_err(err_map("camera.forward"))?;
    b.set_path(
        "camera.up",
        BoxValue::Vec3([
            f64::from(sdg.camera.up.x),
            f64::from(sdg.camera.up.y),
            f64::from(sdg.camera.up.z),
        ]),
    )
    .map_err(err_map("camera.up"))?;
    b.set_f64("camera.vfov_rad", f64::from(sdg.camera.vfov_rad))
        .map_err(err_map("camera.vfov_rad"))?;
    b.set_i64("camera.width", i64::from(sdg.camera.width))
        .map_err(err_map("camera.width"))?;
    b.set_i64("camera.height", i64::from(sdg.camera.height))
        .map_err(err_map("camera.height"))?;
    b.set_i64("sdg.actor_count", actor_count_i64)
        .map_err(err_map("sdg.actor_count"))?;

    let sdg_json = serde_json::to_value(sdg)
        .map_err(|e| format!("EXV SDG frame JSON value serialize failed: {e}"))?;
    b.set_path("sdg.frame", BoxValue::from_json_value(sdg_json))
        .map_err(err_map("sdg.frame"))?;
    Ok(b)
}

/// Builds the clip-level trailing `clip_box` embedded in `MetaFooter` at finalize time.
fn build_clip_meta_box(
    width: u32,
    height: u32,
    profile: ExvProfile,
    frames_encoded: u64,
    frames_dropped: u64,
    meta_frames_embedded: u64,
) -> Result<RustBox, String> {
    let mut b = RustBox::new();
    let enc_i64 = i64::try_from(frames_encoded)
        .map_err(|_| "EXV frames_encoded exceeds i64::MAX".to_string())?;
    let drop_i64 = i64::try_from(frames_dropped)
        .map_err(|_| "EXV frames_dropped exceeds i64::MAX".to_string())?;
    let meta_i64 = i64::try_from(meta_frames_embedded)
        .map_err(|_| "EXV meta_frames_embedded exceeds i64::MAX".to_string())?;

    let err_map = |k: &'static str| move |e| format!("EXV clip metadata `{k}` set failed: {e}");

    b.set_str("schema", "nv_demo.exv_meta.v1")
        .map_err(err_map("schema"))?;
    b.set_str("generator", "nv-demo (nv-rs)")
        .map_err(err_map("generator"))?;
    b.set_i64("width", i64::from(width))
        .map_err(err_map("width"))?;
    b.set_i64("height", i64::from(height))
        .map_err(err_map("height"))?;
    b.set_str("profile", profile.id())
        .map_err(err_map("profile"))?;
    b.set_i64("frames_encoded", enc_i64)
        .map_err(err_map("frames_encoded"))?;
    b.set_i64("frames_dropped", drop_i64)
        .map_err(err_map("frames_dropped"))?;
    b.set_i64("meta_frames_embedded", meta_i64)
        .map_err(err_map("meta_frames_embedded"))?;

    let channels_json = serde_json::Value::Array(
        EXV_CHANNELS
            .iter()
            .map(|&(name, _)| serde_json::Value::String(name.to_string()))
            .collect(),
    );
    b.set_path("channels", BoxValue::from_json_value(channels_json))
        .map_err(err_map("channels"))?;
    Ok(b)
}

/// Active container writer backend inside the background encoder thread.
enum WorkerWriter {
    Standard(Box<ClipWriter<BufWriter<File>>>),
    Hybrid(Box<HybridClipWriter<BufWriter<File>>>),
}

impl WorkerWriter {
    #[inline]
    fn frames(&self) -> usize {
        match self {
            Self::Standard(w) => w.frames(),
            Self::Hybrid(w) => w.frames(),
        }
    }

    #[inline]
    fn bytes(&self) -> u64 {
        match self {
            Self::Standard(w) => w.bytes(),
            Self::Hybrid(w) => w.bytes(),
        }
    }
}

/// Background-threaded multi-AOV `.exv` stream recorder with bounded buffer pooling and atomic
/// finalization.
pub struct ExvRecorder {
    width: u32,
    height: u32,
    profile: ExvProfile,
    shared: Arc<SharedRecorderState>,
    recycle_tx: mpsc::SyncSender<ExvFrameBuffers>,
    recycle_rx: Mutex<mpsc::Receiver<ExvFrameBuffers>>,
    frame_tx: Option<mpsc::SyncSender<ExvFrameSlot>>,
    worker: Option<JoinHandle<Result<ExvFinishReport, String>>>,
}

impl ExvRecorder {
    /// Opens a temporary `.exv.tmp` file in `path`'s directory, writes the EXV prelude and
    /// metadata synchronously, pre-allocates [`EXV_POOL_SLOTS`] host readback slots, and spawns
    /// the background compression thread.
    pub fn start(
        path: PathBuf,
        width: u32,
        height: u32,
        profile: ExvProfile,
        meta: Meta,
    ) -> Result<Self, String> {
        checked_pixel_count(width, height)?;
        let header = build_exv_header(width, height, profile)?;

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create EXV dir {} failed: {e}", parent.display()))?;
        }
        let tmp_path = path.with_extension("exv.tmp");
        let file = File::create(&tmp_path)
            .map_err(|e| format!("create EXV temp {} failed: {e}", tmp_path.display()))?;
        let buf_writer = BufWriter::with_capacity(EXV_WRITE_BUF_BYTES, file);

        let writer = if profile.is_hybrid() {
            let params = ExvProfile::hybrid_params();
            match HybridClipWriter::new(buf_writer, &header, &params, profile.gop(), &meta) {
                Ok(w) => WorkerWriter::Hybrid(Box::new(w)),
                Err(err) => {
                    return Err(cleanup_temp_file(
                        &tmp_path,
                        format!("EXV HybridClipWriter init failed: {err}"),
                    ));
                }
            }
        } else {
            let codec_profile = match profile.resolve_codec_profile(&header) {
                Ok(p) => p,
                Err(err) => {
                    drop(buf_writer);
                    return Err(cleanup_temp_file(&tmp_path, err));
                }
            };
            let write_opts = profile.write_opts(meta);
            match ClipWriter::new(buf_writer, &header, &codec_profile, &write_opts) {
                Ok(w) => WorkerWriter::Standard(Box::new(w)),
                Err(err) => {
                    return Err(cleanup_temp_file(
                        &tmp_path,
                        format!("EXV ClipWriter init failed: {err}"),
                    ));
                }
            }
        };

        let (recycle_tx, recycle_rx) = mpsc::sync_channel::<ExvFrameBuffers>(EXV_POOL_SLOTS);
        for _ in 0..EXV_POOL_SLOTS {
            let bufs = match ExvFrameBuffers::with_dims(width, height) {
                Ok(b) => b,
                Err(err) => {
                    drop(writer);
                    return Err(cleanup_temp_file(&tmp_path, err));
                }
            };
            if recycle_tx.try_send(bufs).is_err() {
                drop(writer);
                return Err(cleanup_temp_file(
                    &tmp_path,
                    "EXV buffer pool initialization overflow".to_string(),
                ));
            }
        }

        let (frame_tx, frame_rx) = mpsc::sync_channel::<ExvFrameSlot>(EXV_POOL_SLOTS);
        let shared = Arc::new(SharedRecorderState::new());
        shared.bytes_written.store(writer.bytes(), Ordering::Release);

        let worker_shared = Arc::clone(&shared);
        let worker_tmp_path = tmp_path.clone();
        let worker = std::thread::Builder::new()
            .name("nv-demo-exv-enc".to_string())
            .spawn(move || {
                run_encoder_worker(
                    writer,
                    header,
                    profile,
                    path,
                    worker_tmp_path,
                    frame_rx,
                    worker_shared,
                )
            })
            .map_err(|e| {
                cleanup_temp_file(&tmp_path, format!("spawn EXV encoder thread failed: {e}"))
            })?;

        Ok(Self {
            width,
            height,
            profile,
            shared,
            recycle_tx,
            recycle_rx: Mutex::new(recycle_rx),
            frame_tx: Some(frame_tx),
            worker: Some(worker),
        })
    }

    /// Clip width in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Clip height in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Active compression preset.
    #[inline]
    pub fn profile(&self) -> ExvProfile {
        self.profile
    }

    /// Lock-free snapshot of live submission/encoding/byte counters.
    #[inline]
    pub fn stats(&self) -> ExvStats {
        self.shared.snapshot()
    }

    /// Returns the first fatal encoder error encountered on the background thread, if any.
    pub fn last_error(&self) -> Option<String> {
        self.shared.get_error()
    }

    /// Blocks until a reusable [`ExvFrameSlot`] is available from the bounded pool.
    ///
    /// Provides natural backpressure for deterministic (`FixedStep` / playlist) capture so zero
    /// frames are dropped and host memory stays strictly bounded to [`EXV_POOL_SLOTS`] frames.
    pub fn acquire_slot(&self) -> Result<ExvFrameSlot, String> {
        if let Some(err) = self.shared.get_error() {
            return Err(err);
        }
        let rx = self
            .recycle_rx
            .lock()
            .map_err(|_| "EXV buffer pool mutex poisoned".to_string())?;
        let bufs = rx.recv().map_err(|_| {
            self.shared
                .get_error()
                .unwrap_or_else(|| "EXV buffer pool closed unexpectedly".to_string())
        })?;
        if let Some(err) = self.shared.get_error() {
            return Err(err);
        }
        Ok(ExvFrameSlot {
            bufs,
            recycle: Some(self.recycle_tx.clone()),
        })
    }

    /// Non-blocking slot acquisition for interactive live viewport recording.
    ///
    /// Returns `Ok(None)` when all [`EXV_POOL_SLOTS`] buffers are currently in flight on the
    /// background encoder thread (call [`Self::note_dropped_frame`] when skipping a live frame).
    pub fn try_acquire_slot(&self) -> Result<Option<ExvFrameSlot>, String> {
        if let Some(err) = self.shared.get_error() {
            return Err(err);
        }
        let rx = self
            .recycle_rx
            .lock()
            .map_err(|_| "EXV buffer pool mutex poisoned".to_string())?;
        match rx.try_recv() {
            Ok(bufs) => Ok(Some(ExvFrameSlot {
                bufs,
                recycle: Some(self.recycle_tx.clone()),
            })),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(self
                .shared
                .get_error()
                .unwrap_or_else(|| "EXV buffer pool disconnected".to_string())),
        }
    }

    /// Records a skipped live frame when [`Self::try_acquire_slot`] returned `Ok(None)`.
    #[inline]
    pub fn note_dropped_frame(&self) {
        self.shared.frames_dropped.fetch_add(1, Ordering::Relaxed);
    }

    /// Submits a filled [`ExvFrameSlot`] to the background encoder thread.
    pub fn submit_slot(&self, slot: ExvFrameSlot) -> Result<(), String> {
        if let Some(err) = self.shared.get_error() {
            return Err(err);
        }
        let expected = checked_pixel_count(self.width, self.height)?;
        slot.bufs.validate_len(expected)?;
        let tx = self
            .frame_tx
            .as_ref()
            .ok_or_else(|| "EXV recorder already finished".to_string())?;
        tx.send(slot).map_err(|_| {
            self.shared
                .get_error()
                .unwrap_or_else(|| "EXV encoder thread terminated unexpectedly".to_string())
        })?;
        self.shared.frames_submitted.fetch_add(1, Ordering::Release);
        Ok(())
    }

    /// Convenience blocking submission from borrowed host AOV slices for tests (acquires a pooled
    /// slot, copies into it, and hands it to the background encoder).
    #[cfg(test)]
    fn submit_frame(&self, input: &ExvFrameInput<'_>) -> Result<(), String> {
        if input.width != self.width || input.height != self.height {
            return Err(format!(
                "EXV frame dimensions ({}x{}) do not match clip ({}x{})",
                input.width, input.height, self.width, self.height
            ));
        }
        let expected = checked_pixel_count(self.width, self.height)?;
        if input.color_rgba.len() != expected
            || input.albedo_rgba.len() != expected
            || input.normal_rough.len() != expected
            || input.depth.len() != expected
            || input.motion.len() != expected
            || input.semantic_id.len() != expected
            || input.instance_id.len() != expected
        {
            return Err(format!(
                "EXV frame input length mismatch: expected {expected}, got color={} albedo={} \
                 normal={} depth={} motion={} sem={} inst={}",
                input.color_rgba.len(),
                input.albedo_rgba.len(),
                input.normal_rough.len(),
                input.depth.len(),
                input.motion.len(),
                input.semantic_id.len(),
                input.instance_id.len(),
            ));
        }
        let mut slot = self.acquire_slot()?;
        slot.bufs.color_rgba.copy_from_slice(input.color_rgba);
        slot.bufs.albedo_rgba.copy_from_slice(input.albedo_rgba);
        slot.bufs.normal_rough.copy_from_slice(input.normal_rough);
        slot.bufs.depth.copy_from_slice(input.depth);
        slot.bufs.motion.copy_from_slice(input.motion);
        slot.bufs.semantic_id.copy_from_slice(input.semantic_id);
        slot.bufs.instance_id.copy_from_slice(input.instance_id);
        slot.bufs.sdg_frame = input.sdg_frame.clone();
        slot.bufs.sdg_occluder = input.sdg_occluder;
        self.submit_slot(slot)
    }

    /// Finalizes a standard (non-Hybrid) `.exv` clip: drains queued frames, emits `MetaFooter` /
    /// `FMET` (per-frame SDG `RustBox`es + trailing `clip_box`), writes the seek index and patched
    /// frame counters, calls `sync_all()`, and atomically renames the temp file to the target `.exv`.
    ///
    /// Returns a typed error if called on an [`ExvProfile::Hybrid`] recorder (use
    /// [`Self::finish_hybrid`]).
    pub fn finish(mut self) -> Result<ExvFinishReport, String> {
        if self.profile.is_hybrid() {
            // Abort cleanly via Drop so `.exv.tmp` is removed before returning the contract error.
            drop(self);
            return Err(
                "ExvRecorder::finish() called on Hybrid profile; use finish_hybrid(video_mp4_path)"
                    .to_string(),
            );
        }
        self.finish_inner()
    }

    /// Finalizes an [`ExvProfile::Hybrid`] `.exv` clip via `exv_video::HybridClipWriter`: streams
    /// the finished NVENC ISO-BMFF `video_mp4_path` in 256 KiB chunks with incremental Castagnoli
    /// CRC-32C, appends `MetaFooter` + 24-byte `FMET` + 24-byte `HYBV` + `VideoIndex` + trailer,
    /// patches `frame_count` & `FLAG_META_FOOTER`, and syncs + atomically renames `.exv.tmp -> .exv`.
    ///
    /// Consumes and deletes `video_mp4_path` on both success and failure so no temporary `.mp4`
    /// files leak on disk. Returns a typed error if called on a non-Hybrid recorder.
    pub fn finish_hybrid(mut self, video_mp4_path: PathBuf) -> Result<ExvFinishReport, String> {
        if !self.profile.is_hybrid() {
            let err = format!(
                "ExvRecorder::finish_hybrid() called on non-Hybrid profile {:?}",
                self.profile
            );
            drop(self);
            return Err(cleanup_temp_file(&video_mp4_path, err));
        }
        if let Err(err) = self.shared.set_hybrid_video_mp4(video_mp4_path.clone()) {
            drop(self);
            return Err(cleanup_temp_file(&video_mp4_path, err));
        }
        self.finish_inner()
    }

    fn finish_inner(&mut self) -> Result<ExvFinishReport, String> {
        self.shared.abort_on_close.store(false, Ordering::Release);
        drop(self.frame_tx.take());
        let handle = self
            .worker
            .take()
            .ok_or_else(|| "EXV recorder worker already joined".to_string())?;
        handle
            .join()
            .map_err(|_| "EXV encoder thread panicked during finalize".to_string())?
    }
}

impl Drop for ExvRecorder {
    fn drop(&mut self) {
        // If dropped without an explicit `finish()`/`finish_hybrid()`, `abort_on_close` stays
        // `true`, causing the worker to remove `.exv.tmp` cleanly without leaving partial files.
        drop(self.frame_tx.take());
        if let Some(handle) = self.worker.take()
            && handle.join().is_err()
        {
            // Worker panicked during unwind/abort; avoid double-panic inside Drop.
        }
    }
}

/// Removes `tmp_path` on an error/abort path, preserving any cleanup failure in the message.
fn cleanup_temp_file(tmp_path: &Path, err: String) -> String {
    match std::fs::remove_file(tmp_path) {
        Ok(()) => err,
        Err(rm_err) if rm_err.kind() == std::io::ErrorKind::NotFound => err,
        Err(rm_err) => format!(
            "{err}; removing temp {} also failed: {rm_err}",
            tmp_path.display()
        ),
    }
}

/// Background encoder loop: packs pooled slots into planar buffers, recycles slots immediately,
/// embeds per-frame `RustBox` metadata into `ClipWriter` or `HybridClipWriter`, and performs
/// durable atomic finalization on channel close.
fn run_encoder_worker(
    mut writer: WorkerWriter,
    header: Header,
    profile: ExvProfile,
    final_path: PathBuf,
    tmp_path: PathBuf,
    frame_rx: mpsc::Receiver<ExvFrameSlot>,
    shared: Arc<SharedRecorderState>,
) -> Result<ExvFinishReport, String> {
    let mut meta_frames_embedded = 0u64;
    let mut fatal_err: Option<String> = None;

    for mut slot in frame_rx {
        if fatal_err.is_some() {
            // Drain remaining slots so they recycle cleanly and the render thread never deadlocks.
            drop(slot);
            continue;
        }

        let sdg_frame = slot.bufs.sdg_frame.take();
        let sdg_occluder = slot.bufs.sdg_occluder.take();
        let exv_frame_index = writer.frames() as u64;

        let encoded_res = match &mut writer {
            WorkerWriter::Standard(clip_writer) => {
                let packed = pack_frame_buffers_to_image(&slot.bufs, &header);
                // Recycle readback buffers immediately before metadata serialization & CPU
                // compression so frame N+1 GPU readback overlaps with frame N encoding.
                drop(slot);
                let frame_box = sdg_frame
                    .as_ref()
                    .map(|sdg| build_frame_meta_box(exv_frame_index, sdg, sdg_occluder))
                    .transpose();
                match (packed, frame_box) {
                    (Ok(img), Ok(Some(fbox))) => clip_writer
                        .push_with_meta(img, &fbox)
                        .map(|()| true)
                        .map_err(|e| format!("EXV frame {exv_frame_index} encode failed: {e}")),
                    (Ok(img), Ok(None)) => clip_writer
                        .push(img)
                        .map(|()| false)
                        .map_err(|e| format!("EXV frame {exv_frame_index} encode failed: {e}")),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            WorkerWriter::Hybrid(hyb_writer) => {
                let packed = pack_hybrid_aov_planes(&slot.bufs, header.width, header.height);
                drop(slot);
                let frame_box = sdg_frame
                    .as_ref()
                    .map(|sdg| build_frame_meta_box(exv_frame_index, sdg, sdg_occluder))
                    .transpose();
                match (packed, frame_box) {
                    (Ok(aov_planes), Ok(Some(fbox))) => hyb_writer
                        .push_aov_planes_with_meta(&aov_planes, &fbox)
                        .map(|()| true)
                        .map_err(|e| {
                            format!("EXV hybrid frame {exv_frame_index} encode failed: {e}")
                        }),
                    (Ok(aov_planes), Ok(None)) => hyb_writer
                        .push_aov_planes(&aov_planes)
                        .map(|()| false)
                        .map_err(|e| {
                            format!("EXV hybrid frame {exv_frame_index} encode failed: {e}")
                        }),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
        };

        match encoded_res {
            Ok(has_meta) => {
                if has_meta {
                    meta_frames_embedded += 1;
                }
            }
            Err(err) => {
                shared.set_error(err.clone());
                fatal_err = Some(err);
                continue;
            }
        }

        shared
            .frames_encoded
            .store(writer.frames() as u64, Ordering::Release);
        shared
            .bytes_written
            .store(writer.bytes(), Ordering::Release);
    }

    let hybrid_mp4 = shared.take_hybrid_video_mp4();
    let cleanup_all = |err: String| -> Result<ExvFinishReport, String> {
        let mut msg = cleanup_temp_file(&tmp_path, err);
        if let Some(ref mp4) = hybrid_mp4 {
            msg = cleanup_temp_file(mp4, msg);
        }
        Err(msg)
    };

    if let Some(err) = fatal_err {
        drop(writer);
        return cleanup_all(err);
    }

    if shared.abort_on_close.load(Ordering::Acquire) {
        drop(writer);
        return cleanup_all("EXV recording aborted before finish".to_string());
    }

    let encoded_count = writer.frames() as u64;
    if encoded_count == 0 {
        drop(writer);
        return cleanup_all("EXV recording stopped with 0 frames (nothing saved)".to_string());
    }

    let frames_dropped = shared.frames_dropped.load(Ordering::Acquire);
    let clip_box = match build_clip_meta_box(
        header.width,
        header.height,
        profile,
        encoded_count,
        frames_dropped,
        meta_frames_embedded,
    ) {
        Ok(b) => b,
        Err(err) => {
            drop(writer);
            return cleanup_all(err);
        }
    };

    let (mut buf_writer, total_bytes) = match writer {
        WorkerWriter::Standard(mut clip_writer) => {
            if let Err(err) = clip_writer.set_clip_box(&clip_box) {
                drop(clip_writer);
                return cleanup_all(format!("EXV set_clip_box failed: {err}"));
            }
            match clip_writer.finish() {
                Ok(res) => res,
                Err(err) => {
                    return cleanup_all(format!("EXV ClipWriter::finish failed: {err}"));
                }
            }
        }
        WorkerWriter::Hybrid(mut hyb_writer) => {
            let Some(ref mp4_path) = hybrid_mp4 else {
                drop(hyb_writer);
                return cleanup_all(
                    "EXV Hybrid finalize missing NVENC MP4 segment path".to_string(),
                );
            };
            if let Err(err) = hyb_writer.set_clip_box(&clip_box) {
                drop(hyb_writer);
                return cleanup_all(format!("EXV hybrid set_clip_box failed: {err}"));
            }
            match hyb_writer.finish_with_video_file(mp4_path) {
                Ok(res) => res,
                Err(err) => {
                    return cleanup_all(format!("EXV HybridClipWriter::finish failed: {err}"));
                }
            }
        }
    };

    if let Err(err) = buf_writer.flush() {
        drop(buf_writer);
        return cleanup_all(format!("EXV flush {} failed: {err}", tmp_path.display()));
    }
    let file = match buf_writer.into_inner() {
        Ok(f) => f,
        Err(err) => {
            return cleanup_all(format!(
                "EXV BufWriter::into_inner {} failed: {err}",
                tmp_path.display()
            ));
        }
    };
    if let Err(err) = file.sync_all() {
        drop(file);
        return cleanup_all(format!("EXV sync_all {} failed: {err}", tmp_path.display()));
    }
    // Close handle before atomic rename (mandatory on Windows).
    drop(file);

    if let Err(err) = std::fs::rename(&tmp_path, &final_path) {
        return cleanup_all(format!(
            "EXV atomic rename {} -> {} failed: {err}",
            tmp_path.display(),
            final_path.display()
        ));
    }

    // Remove the staged Hybrid temp MP4 now that it is durably embedded inside `final_path`.
    if let Some(ref mp4_path) = hybrid_mp4
        && let Err(rm_err) = std::fs::remove_file(mp4_path)
        && rm_err.kind() != std::io::ErrorKind::NotFound
    {
        return Err(format!(
            "EXV Hybrid saved {}, but removing temp MP4 {} failed: {rm_err}",
            final_path.display(),
            mp4_path.display()
        ));
    }

    shared.bytes_written.store(total_bytes, Ordering::Release);

    Ok(ExvFinishReport {
        path: final_path,
        meta_frames_embedded,
        frames_encoded: encoded_count,
        frames_dropped,
        bytes_written: total_bytes,
    })
}

/// Packs the 13 non-RGB AOV channels (`A`, `albedo.{R,G,B}`, `normal.{X,Y,Z}`, `roughness`,
/// `depth.Z`, `motion.{X,Y}`, `semantic.id`, `instance.id`) shared by both standard 16-channel
/// clips and [`ExvProfile::Hybrid`] (SSOT — zero copy-paste between packing paths).
fn pack_non_rgb_aov_planes(bufs: &ExvFrameBuffers, n: usize) -> Result<[Vec<u8>; 13], String> {
    let f16_bytes = n
        .checked_mul(2)
        .ok_or_else(|| "EXV F16 plane byte size overflow".to_string())?;
    let f32_bytes = n
        .checked_mul(4)
        .ok_or_else(|| "EXV F32/U32 plane byte size overflow".to_string())?;

    let mut ch_a = Vec::with_capacity(f16_bytes);
    for px in &bufs.color_rgba {
        ch_a.extend_from_slice(&f32_to_half(px[3]).to_le_bytes());
    }

    // Single linear pass over interleaved Albedo RGBA -> 3 F16 planes.
    let mut ch_alb_r = Vec::with_capacity(f16_bytes);
    let mut ch_alb_g = Vec::with_capacity(f16_bytes);
    let mut ch_alb_b = Vec::with_capacity(f16_bytes);
    for px in &bufs.albedo_rgba {
        ch_alb_r.extend_from_slice(&f32_to_half(px[0]).to_le_bytes());
        ch_alb_g.extend_from_slice(&f32_to_half(px[1]).to_le_bytes());
        ch_alb_b.extend_from_slice(&f32_to_half(px[2]).to_le_bytes());
    }

    // Single linear pass over Normal.xyz + Roughness.w -> 4 F16 planes.
    let mut ch_nx = Vec::with_capacity(f16_bytes);
    let mut ch_ny = Vec::with_capacity(f16_bytes);
    let mut ch_nz = Vec::with_capacity(f16_bytes);
    let mut ch_rough = Vec::with_capacity(f16_bytes);
    for px in &bufs.normal_rough {
        ch_nx.extend_from_slice(&f32_to_half(px[0]).to_le_bytes());
        ch_ny.extend_from_slice(&f32_to_half(px[1]).to_le_bytes());
        ch_nz.extend_from_slice(&f32_to_half(px[2]).to_le_bytes());
        ch_rough.extend_from_slice(&f32_to_half(px[3]).to_le_bytes());
    }

    // Single linear pass over interleaved Motion XY -> 2 F32 planes.
    let mut ch_mx = Vec::with_capacity(f32_bytes);
    let mut ch_my = Vec::with_capacity(f32_bytes);
    for mv in &bufs.motion {
        ch_mx.extend_from_slice(&mv[0].to_le_bytes());
        ch_my.extend_from_slice(&mv[1].to_le_bytes());
    }

    // Contiguous F32 / U32 planes (memcpy on little-endian hosts).
    let ch_depth = pack_f32_slice_le(&bufs.depth);
    let ch_sem = pack_u32_slice_le(&bufs.semantic_id);
    let ch_inst = pack_u32_slice_le(&bufs.instance_id);

    Ok([
        ch_a, ch_alb_r, ch_alb_g, ch_alb_b, ch_nx, ch_ny, ch_nz, ch_rough, ch_depth, ch_mx, ch_my,
        ch_sem, ch_inst,
    ])
}

/// Packs the 13 non-RGB AOV channels for one [`ExvProfile::Hybrid`] frame (skipping CPU RGB
/// packing because NVENC encodes Beauty RGB directly on the GPU).
fn pack_hybrid_aov_planes(
    bufs: &ExvFrameBuffers,
    width: u32,
    height: u32,
) -> Result<[Vec<u8>; EXV_HYBRID_AOV_COUNT], String> {
    let n = checked_pixel_count(width, height)?;
    bufs.validate_len(n)?;
    pack_non_rgb_aov_planes(bufs, n)
}

/// Packs one [`ExvFrameBuffers`] frame into an [`exv_codec::image::Image`] matching `header`'s
/// canonical 16-channel [`EXV_CHANNELS`] schema.
pub fn pack_frame_buffers_to_image(
    bufs: &ExvFrameBuffers,
    header: &Header,
) -> Result<Image, String> {
    let n = checked_pixel_count(header.width, header.height)?;
    bufs.validate_len(n)?;
    if header.channels.len() != EXV_CHANNEL_COUNT {
        return Err(format!(
            "EXV header channel count ({}) != canonical {EXV_CHANNEL_COUNT}",
            header.channels.len()
        ));
    }

    let f16_bytes = n
        .checked_mul(2)
        .ok_or_else(|| "EXV F16 plane byte size overflow".to_string())?;

    let mut ch_r = Vec::with_capacity(f16_bytes);
    let mut ch_g = Vec::with_capacity(f16_bytes);
    let mut ch_b = Vec::with_capacity(f16_bytes);
    for px in &bufs.color_rgba {
        ch_r.extend_from_slice(&f32_to_half(px[0]).to_le_bytes());
        ch_g.extend_from_slice(&f32_to_half(px[1]).to_le_bytes());
        ch_b.extend_from_slice(&f32_to_half(px[2]).to_le_bytes());
    }

    let [
        ch_a,
        ch_alb_r,
        ch_alb_g,
        ch_alb_b,
        ch_nx,
        ch_ny,
        ch_nz,
        ch_rough,
        ch_depth,
        ch_mx,
        ch_my,
        ch_sem,
        ch_inst,
    ] = pack_non_rgb_aov_planes(bufs, n)?;

    let planes = vec![
        ch_r,     // CH_R
        ch_g,     // CH_G
        ch_b,     // CH_B
        ch_a,     // CH_A
        ch_alb_r, // CH_ALBEDO_R
        ch_alb_g, // CH_ALBEDO_G
        ch_alb_b, // CH_ALBEDO_B
        ch_nx,    // CH_NORMAL_X
        ch_ny,    // CH_NORMAL_Y
        ch_nz,    // CH_NORMAL_Z
        ch_rough, // CH_ROUGHNESS
        ch_depth, // CH_DEPTH_Z
        ch_mx,    // CH_MOTION_X
        ch_my,    // CH_MOTION_Y
        ch_sem,   // CH_SEMANTIC_ID
        ch_inst,  // CH_INSTANCE_ID
    ];

    debug_assert_eq!(planes.len(), EXV_CHANNEL_COUNT);
    debug_assert_eq!(CH_A, 3);
    debug_assert_eq!(CH_ALBEDO_R, 4);
    debug_assert_eq!(CH_ALBEDO_G, 5);
    debug_assert_eq!(CH_ALBEDO_B, 6);
    debug_assert_eq!(CH_NORMAL_X, 7);
    debug_assert_eq!(CH_NORMAL_Y, 8);
    debug_assert_eq!(CH_NORMAL_Z, 9);
    debug_assert_eq!(CH_ROUGHNESS, 10);
    debug_assert_eq!(CH_DEPTH_Z, 11);
    debug_assert_eq!(CH_MOTION_X, 12);
    debug_assert_eq!(CH_MOTION_Y, 13);
    debug_assert_eq!(CH_SEMANTIC_ID, 14);
    debug_assert_eq!(CH_INSTANCE_ID, 15);

    Image::new(header.clone(), planes).map_err(|e| format!("EXV Image::new failed: {e}"))
}

#[inline]
fn checked_pixel_count(width: u32, height: u32) -> Result<usize, String> {
    if width == 0 || height == 0 {
        return Err(format!("EXV dimensions ({width}x{height}) must be non-zero"));
    }
    (width as usize)
        .checked_mul(height as usize)
        .ok_or_else(|| format!("EXV pixel count overflow for {width}x{height}"))
}

#[inline]
fn pack_f32_slice_le(src: &[f32]) -> Vec<u8> {
    #[cfg(target_endian = "little")]
    {
        bytemuck::cast_slice::<f32, u8>(src).to_vec()
    }
    #[cfg(not(target_endian = "little"))]
    {
        let mut out = Vec::with_capacity(src.len() * 4);
        for &v in src {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }
}

#[inline]
fn pack_u32_slice_le(src: &[u32]) -> Vec<u8> {
    #[cfg(target_endian = "little")]
    {
        bytemuck::cast_slice::<u32, u8>(src).to_vec()
    }
    #[cfg(not(target_endian = "little"))]
    {
        let mut out = Vec::with_capacity(src.len() * 4);
        for &v in src {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csg_actors::{SdgCamera, SdgProjection, Vec3 as ActorVec3};
    use exv_codec::frame::FrameReader;
    use exv_format::Verify;
    use exv_format::half::half_to_f32;
    use exv_video::{
        Chans, HYBRID_LOCATOR_LEN, HybridLocator, VideoIndex, decode_frame, frame_view,
        verify_checksums,
    };

    struct SynthFrame {
        color_rgba: Vec<[f32; 4]>,
        albedo_rgba: Vec<[f32; 4]>,
        normal_rough: Vec<[f32; 4]>,
        depth: Vec<f32>,
        motion: Vec<[f32; 2]>,
        semantic_id: Vec<u32>,
        instance_id: Vec<u32>,
    }

    fn make_synth_frame(w: u32, h: u32, frame_idx: u32) -> SynthFrame {
        let n = (w as usize) * (h as usize);
        let mut color_rgba = Vec::with_capacity(n);
        let mut albedo_rgba = Vec::with_capacity(n);
        let mut normal_rough = Vec::with_capacity(n);
        let mut depth = Vec::with_capacity(n);
        let mut motion = Vec::with_capacity(n);
        let mut semantic_id = Vec::with_capacity(n);
        let mut instance_id = Vec::with_capacity(n);

        let shift = frame_idx as f32;
        for y in 0..h {
            for x in 0..w {
                let u = (x as f32 + shift * 0.5) / (w as f32);
                let v = (y as f32) / (h as f32);
                // Smooth HDR linear radiance with a realistic 4-row sky band (`y < 4`) aligned
                // to 2 full `4:2:0` chroma blocks, plus smooth ground gradients below.
                let is_sky = y < 4;
                let r = if is_sky { 4.5 } else { 0.15 + 1.8 * u };
                let g = if is_sky { 6.2 } else { 0.10 + 1.2 * v };
                let b = if is_sky { 9.0 } else { 0.25 + 0.6 * (1.0 - u) };
                color_rgba.push([r, g, b, 1.0]);
                albedo_rgba.push([0.2 + 0.5 * u, 0.3 + 0.4 * v, 0.25, 1.0]);

                let nx = (u * 2.0 - 1.0) * 0.5;
                let ny = (v * 2.0 - 1.0) * 0.5;
                let nz = (1.0 - nx * nx - ny * ny).max(0.0).sqrt();
                normal_rough.push([nx, ny, nz, 0.1 + 0.8 * v]);

                depth.push(if is_sky { 1.0e6 } else { 3.5 + u * 40.0 + shift });
                motion.push([(u - 0.5) * 4.0 + shift * 0.25, (0.5 - v) * 2.5]);

                let sem = if is_sky {
                    0
                } else {
                    match (x + y + frame_idx) % 5 {
                        0 => 1,  // Road
                        1 => 4,  // Building
                        2 => 10, // Vehicle
                        3 => 18, // Pedestrian
                        _ => 20, // Vegetation
                    }
                };
                semantic_id.push(sem);
                instance_id.push(if is_sky {
                    0
                } else {
                    0x8000_0000 | (1000 + frame_idx * 17 + (x / 4) + (y / 4) * 32)
                });
            }
        }

        SynthFrame {
            color_rgba,
            albedo_rgba,
            normal_rough,
            depth,
            motion,
            semantic_id,
            instance_id,
        }
    }

    fn sample_sdg_frame(w: u32, h: u32, frame_idx: u32) -> SdgExportFrame {
        SdgExportFrame {
            scene_id: "gundo_osm".to_string(),
            scene_idx: 2,
            variant_idx: 0,
            frame_index: 100 + frame_idx,
            time_s: frame_idx as f32 / 60.0,
            camera: SdgCamera {
                projection: SdgProjection::Pinhole,
                eye: ActorVec3::from_array([10.0 + frame_idx as f32, 3.0, -5.0]),
                forward: ActorVec3::from_array([0.0, 0.0, 1.0]),
                up: ActorVec3::from_array([0.0, 1.0, 0.0]),
                vfov_rad: 0.9,
                width: w,
                height: h,
            },
            actors: Vec::new(),
        }
    }

    fn read_f16_plane(plane: &[u8]) -> Vec<u16> {
        plane
            .as_chunks::<2>()
            .0
            .iter()
            .copied()
            .map(u16::from_le_bytes)
            .collect()
    }

    fn read_f32_plane(plane: &[u8]) -> Vec<f32> {
        plane
            .as_chunks::<4>()
            .0
            .iter()
            .copied()
            .map(f32::from_le_bytes)
            .collect()
    }

    fn read_u32_plane(plane: &[u8]) -> Vec<u32> {
        plane
            .as_chunks::<4>()
            .0
            .iter()
            .copied()
            .map(u32::from_le_bytes)
            .collect()
    }

    fn sample_meta(w: u32, h: u32, profile: ExvProfile) -> Meta {
        let info = ExvClipMetadata {
            fps: (60, 1),
            scene_id: "gundo_osm".to_string(),
            projection: "Pinhole".to_string(),
            profile,
            beauty_denoised: true,
            instance_manifest: vec![
                ("robotaxi_01".to_string(), 0x8000_03e8),
                ("pedestrian_04".to_string(), 0x8000_03f9),
            ],
        };
        build_exv_meta(w, h, &info).expect("build_exv_meta")
    }

    #[test]
    fn canonical_header_exposes_16_channels_and_rgb_triad() {
        for profile in ExvProfile::ALL {
            let hdr = build_exv_header(32, 24, profile).expect("build header");
            assert_eq!(hdr.channels.len(), EXV_CHANNEL_COUNT);
            assert_eq!(Header::rgb_triad(&hdr.channels), Some(EXV_RGB_INDICES));
            for (ch, &(want_name, want_dtype)) in hdr.channels.iter().zip(&EXV_CHANNELS) {
                assert_eq!(ch.name, want_name);
                assert_eq!(ch.dtype, want_dtype);
            }
            if profile.is_hybrid() {
                assert_eq!(hdr.codec_id, CodecId::HYBRID.0);
                let decoded = HybridParams::decode(&hdr.codec_params).expect("decode HybridParams");
                assert_eq!(decoded, ExvProfile::hybrid_params());
            }
        }
    }

    #[test]
    fn roundtrip_standard_profiles_verifies_16_channels_u32_exact_and_meta() {
        let (w, h) = (32u32, 24u32);
        let frames: Vec<SynthFrame> = (0..4).map(|k| make_synth_frame(w, h, k)).collect();

        for profile in ExvProfile::ALL.into_iter().filter(|p| !p.is_hybrid()) {
            let path = std::env::temp_dir().join(format!(
                "nv_demo_exv_rt_{}_{}.exv",
                profile.id(),
                std::process::id()
            ));
            let meta = sample_meta(w, h, profile);
            let rec = ExvRecorder::start(path.clone(), w, h, profile, meta)
                .expect("start ExvRecorder");

            for f in &frames {
                let input = ExvFrameInput {
                    width: w,
                    height: h,
                    color_rgba: &f.color_rgba,
                    albedo_rgba: &f.albedo_rgba,
                    normal_rough: &f.normal_rough,
                    depth: &f.depth,
                    motion: &f.motion,
                    semantic_id: &f.semantic_id,
                    instance_id: &f.instance_id,
                    sdg_frame: None,
                    sdg_occluder: None,
                };
                rec.submit_frame(&input).expect("submit_frame");
            }

            let report = rec.finish().expect("finish ExvRecorder");
            assert_eq!(report.path, path);
            assert_eq!(report.meta_frames_embedded, 0);
            assert_eq!(report.frames_encoded, frames.len() as u64);
            assert_eq!(report.frames_dropped, 0);
            assert!(report.bytes_written > 0);
            assert!(!path.with_extension("exv.tmp").exists(), "temp file must be gone");
            assert!(
                !path.with_extension("sdg.json").exists(),
                "no sibling .sdg.json sidecar should be emitted"
            );

            let bytes = std::fs::read(&path).expect("read .exv");
            std::fs::remove_file(&path).expect("clean .exv");
            assert_eq!(bytes.len() as u64, report.bytes_written);

            let idx = VideoIndex::parse_with_verify(&bytes, Verify::On)
                .expect("VideoIndex::parse_with_verify(On)");
            assert!(
                verify_checksums(&bytes, &idx, 16)
                    .expect("verify_checksums")
                    .is_empty(),
                "zero bad blobs expected"
            );
            let clip_box = idx
                .clip_box(&bytes)
                .expect("clip_box decode")
                .expect("clip_box present");
            assert_eq!(clip_box.get_str("schema"), Some("nv_demo.exv_meta.v1"));
            assert_eq!(clip_box.get_str("profile"), Some(profile.id()));
            assert_eq!(clip_box.get_i64("frames_encoded"), Some(frames.len() as i64));
            assert_eq!(clip_box.get_i64("meta_frames_embedded"), Some(0));
            assert!(
                idx.frame_meta(&bytes, 0)
                    .expect("frame_meta lookup")
                    .is_none(),
                "no per-frame SDG rows expected when sdg_frame is None"
            );
            assert_eq!(idx.frame_count(), frames.len());
            assert_eq!(idx.header.width, w);
            assert_eq!(idx.header.height, h);
            assert_eq!(idx.header.channels.len(), EXV_CHANNEL_COUNT);
            for (ch, &(name, dtype)) in idx.header.channels.iter().zip(&EXV_CHANNELS) {
                assert_eq!(ch.name, name);
                assert_eq!(ch.dtype, dtype);
            }

            // Verify embedded Meta + OpenEXR attributes + Cryptomatte manifest.
            let meta_back = idx.meta().expect("meta present");
            assert_eq!(meta_back.fps, Some((60, 1)));
            assert_eq!(meta_back.alpha, 2);
            assert_eq!(meta_back.pixel_aspect, Some((1, 1)));
            let exr_hdrs = meta_back.decode_exr_headers().expect("decode_exr_headers");
            assert_eq!(exr_hdrs.len(), 1);
            let exr = &exr_hdrs[0];
            let scene_attr = exr
                .typed_attribute::<exr_core::attr::ExrString>("nv_demo/scene_id")
                .expect("scene_id attr");
            assert_eq!(scene_attr.value.as_str().expect("utf8"), "gundo_osm");
            let profile_attr = exr
                .typed_attribute::<exr_core::attr::ExrString>("nv_demo/exv_profile")
                .expect("exv_profile attr");
            assert_eq!(profile_attr.value.as_str().expect("utf8"), profile.id());
            let sem_map_attr = exr
                .typed_attribute::<exr_core::attr::ExrString>("nv_demo/semantic_map")
                .expect("semantic_map attr");
            let sem_parsed: serde_json::Map<String, serde_json::Value> =
                serde_json::from_str(sem_map_attr.value.as_str().expect("utf8"))
                    .expect("parse semantic_map JSON");
            assert_eq!(sem_parsed.len(), usize::from(semantic::COUNT));
            assert_eq!(sem_parsed["18"], "Pedestrian");

            let crypto_id = instance_cryptomatte_id();
            let manifest_attr = exr
                .typed_attribute::<exr_core::attr::ExrString>(&format!(
                    "cryptomatte/{crypto_id}/manifest"
                ))
                .expect("cryptomatte manifest attr");
            let manifest_parsed: BTreeMap<String, String> =
                serde_json::from_str(manifest_attr.value.as_str().expect("utf8"))
                    .expect("parse cryptomatte manifest");
            assert_eq!(manifest_parsed["robotaxi_01"], "800003e8");
            assert_eq!(manifest_parsed["pedestrian_04"], "800003f9");

            // Decode every frame and verify all 16 channels.
            for (k, src) in frames.iter().enumerate() {
                let decoded = decode_frame(&bytes, &idx, k, Chans::All)
                    .expect("decode_frame")
                    .into_image()
                    .expect("into_image");
                assert_eq!(decoded.planes.len(), EXV_CHANNEL_COUNT);

                // U32 semantic.id & instance.id MUST be 100% bit-exact for EVERY profile!
                let sem_back = read_u32_plane(&decoded.planes[CH_SEMANTIC_ID]);
                let inst_back = read_u32_plane(&decoded.planes[CH_INSTANCE_ID]);
                assert_eq!(
                    sem_back, src.semantic_id,
                    "semantic.id mismatch on {profile:?} frame {k}"
                );
                assert_eq!(
                    inst_back, src.instance_id,
                    "instance.id mismatch on {profile:?} frame {k}"
                );

                let r_back = read_f16_plane(&decoded.planes[CH_R]);
                let g_back = read_f16_plane(&decoded.planes[CH_G]);
                let b_back = read_f16_plane(&decoded.planes[CH_B]);
                let a_back = read_f16_plane(&decoded.planes[CH_A]);
                let alb_r = read_f16_plane(&decoded.planes[CH_ALBEDO_R]);
                let alb_g = read_f16_plane(&decoded.planes[CH_ALBEDO_G]);
                let alb_b = read_f16_plane(&decoded.planes[CH_ALBEDO_B]);
                let nx_back = read_f16_plane(&decoded.planes[CH_NORMAL_X]);
                let ny_back = read_f16_plane(&decoded.planes[CH_NORMAL_Y]);
                let nz_back = read_f16_plane(&decoded.planes[CH_NORMAL_Z]);
                let rough_back = read_f16_plane(&decoded.planes[CH_ROUGHNESS]);
                let depth_back = read_f32_plane(&decoded.planes[CH_DEPTH_Z]);
                let mx_back = read_f32_plane(&decoded.planes[CH_MOTION_X]);
                let my_back = read_f32_plane(&decoded.planes[CH_MOTION_Y]);

                if profile.is_lossless() {
                    for i in 0..src.color_rgba.len() {
                        assert_eq!(r_back[i], f32_to_half(src.color_rgba[i][0]));
                        assert_eq!(g_back[i], f32_to_half(src.color_rgba[i][1]));
                        assert_eq!(b_back[i], f32_to_half(src.color_rgba[i][2]));
                        assert_eq!(a_back[i], f32_to_half(src.color_rgba[i][3]));
                        assert_eq!(alb_r[i], f32_to_half(src.albedo_rgba[i][0]));
                        assert_eq!(alb_g[i], f32_to_half(src.albedo_rgba[i][1]));
                        assert_eq!(alb_b[i], f32_to_half(src.albedo_rgba[i][2]));
                        assert_eq!(nx_back[i], f32_to_half(src.normal_rough[i][0]));
                        assert_eq!(ny_back[i], f32_to_half(src.normal_rough[i][1]));
                        assert_eq!(nz_back[i], f32_to_half(src.normal_rough[i][2]));
                        assert_eq!(rough_back[i], f32_to_half(src.normal_rough[i][3]));
                        assert_eq!(depth_back[i].to_bits(), src.depth[i].to_bits());
                        assert_eq!(mx_back[i].to_bits(), src.motion[i][0].to_bits());
                        assert_eq!(my_back[i].to_bits(), src.motion[i][1].to_bits());
                    }
                } else {
                    // Lossy perceptual HDR 9/7 (`q = 20`, `4:2:0` Beauty YCbCr, full-res float
                    // AOVs): every sample stays finite, and smooth interior regions away from the
                    // sharp 60x sky/ground horizon (`y == 4`) stay within tight relative bounds.
                    for y in 0..h {
                        for x in 0..w {
                            let i = (y * w + x) as usize;
                            let r = half_to_f32(r_back[i]);
                            let g = half_to_f32(g_back[i]);
                            let b = half_to_f32(b_back[i]);
                            let a = half_to_f32(a_back[i]);
                            let ar = half_to_f32(alb_r[i]);
                            let ag = half_to_f32(alb_g[i]);
                            let ab = half_to_f32(alb_b[i]);
                            let nx = half_to_f32(nx_back[i]);
                            let ny = half_to_f32(ny_back[i]);
                            let nz = half_to_f32(nz_back[i]);
                            let rough = half_to_f32(rough_back[i]);
                            let z = depth_back[i];
                            let mx = mx_back[i];
                            let my = my_back[i];

                            assert!(
                                r.is_finite()
                                    && g.is_finite()
                                    && b.is_finite()
                                    && a.is_finite()
                                    && ar.is_finite()
                                    && ag.is_finite()
                                    && ab.is_finite()
                                    && nx.is_finite()
                                    && ny.is_finite()
                                    && nz.is_finite()
                                    && rough.is_finite()
                                    && z.is_finite()
                                    && z > 0.0
                                    && mx.is_finite()
                                    && my.is_finite(),
                                "non-finite/non-positive sample on Compact frame {k} ({x},{y})"
                            );

                            if y <= 1 || y >= 6 {
                                let rel = |got: f32, want: f32| {
                                    (got - want).abs() / want.abs().max(1.0)
                                };
                                assert!(
                                    rel(r, src.color_rgba[i][0]) < 0.12
                                        && rel(g, src.color_rgba[i][1]) < 0.12
                                        && rel(b, src.color_rgba[i][2]) < 0.12
                                        && rel(a, src.color_rgba[i][3]) < 0.05,
                                    "Compact beauty drift frame {k} ({x},{y}): \
                                     got=({r},{g},{b},{a}) want={:?}",
                                    src.color_rgba[i]
                                );
                                assert!(
                                    rel(ar, src.albedo_rgba[i][0]) < 0.10
                                        && rel(ag, src.albedo_rgba[i][1]) < 0.10
                                        && rel(ab, src.albedo_rgba[i][2]) < 0.10,
                                    "Compact albedo drift frame {k} ({x},{y})"
                                );
                                assert!(
                                    rel(nx, src.normal_rough[i][0]) < 0.10
                                        && rel(ny, src.normal_rough[i][1]) < 0.10
                                        && rel(nz, src.normal_rough[i][2]) < 0.10
                                        && rel(rough, src.normal_rough[i][3]) < 0.10,
                                    "Compact normal/rough drift frame {k} ({x},{y})"
                                );
                                assert!(
                                    rel(z, src.depth[i]) < 0.12
                                        && rel(mx, src.motion[i][0]) < 0.12
                                        && rel(my, src.motion[i][1]) < 0.12,
                                    "Compact depth/motion drift frame {k} ({x},{y}): \
                                     z={z} vs {}, m=({mx},{my}) vs {:?}",
                                    src.depth[i],
                                    src.motion[i]
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn roundtrip_hybrid_profile_verifies_hybv_crc_checksums_and_13_lossless_aovs() {
        let (w, h) = (32u32, 24u32);
        let frames: Vec<SynthFrame> = (0..4).map(|k| make_synth_frame(w, h, k)).collect();
        let path = std::env::temp_dir().join(format!(
            "nv_demo_exv_hybrid_rt_{}.exv",
            std::process::id()
        ));
        let mp4_path = hybrid_temp_mp4_path(&path);

        // Build a multi-chunk mock ISO-BMFF payload (> 2 * 256 KiB) to exercise streaming CRC-32C.
        let mut mock_mp4 = Vec::with_capacity(600_000);
        mock_mp4.extend_from_slice(b"\x00\x00\x00\x18ftypisom\x00\x00\x02\x00isomhvc1");
        for i in 0..600_000u32 {
            mock_mp4.push((i.wrapping_mul(1_664_525).wrapping_add(1_013_904_223) >> 24) as u8);
        }
        std::fs::write(&mp4_path, &mock_mp4).expect("write mock hybrid mp4");

        let meta = sample_meta(w, h, ExvProfile::Hybrid);
        let rec = ExvRecorder::start(path.clone(), w, h, ExvProfile::Hybrid, meta)
            .expect("start Hybrid ExvRecorder");

        for (k, f) in frames.iter().enumerate() {
            let input = ExvFrameInput {
                width: w,
                height: h,
                color_rgba: &f.color_rgba,
                albedo_rgba: &f.albedo_rgba,
                normal_rough: &f.normal_rough,
                depth: &f.depth,
                motion: &f.motion,
                semantic_id: &f.semantic_id,
                instance_id: &f.instance_id,
                sdg_frame: Some(sample_sdg_frame(w, h, k as u32)),
                sdg_occluder: Some(ActiveSdgOccluder::Optix),
            };
            rec.submit_frame(&input).expect("submit Hybrid frame");
        }

        let report = rec.finish_hybrid(mp4_path.clone()).expect("finish_hybrid");
        assert_eq!(report.path, path);
        assert_eq!(report.meta_frames_embedded, 4);
        assert_eq!(report.frames_encoded, frames.len() as u64);
        assert!(report.bytes_written > mock_mp4.len() as u64);
        assert!(!path.with_extension("exv.tmp").exists(), "temp .exv.tmp must be gone");
        assert!(!mp4_path.exists(), "temp .hybrid.tmp.mp4 must be removed after embedding");
        assert!(
            !path.with_extension("sdg.json").exists(),
            "no sibling .sdg.json sidecar should be emitted for Hybrid"
        );

        let bytes = std::fs::read(&path).expect("read hybrid .exv");
        assert_eq!(bytes.len() as u64, report.bytes_written);

        // Full container parse + Verify::On CRC-32C validation (both FMET + HYBV) + checksum scan.
        let idx = VideoIndex::parse_with_verify(&bytes, Verify::On)
            .expect("VideoIndex::parse_with_verify(On)");
        assert_eq!(idx.frame_count(), frames.len());
        assert_eq!(idx.header.codec_id, CodecId::HYBRID.0);
        assert_eq!(idx.hybrid_params(), Some(&ExvProfile::hybrid_params()));
        assert!(
            verify_checksums(&bytes, &idx, 64)
                .expect("verify_checksums")
                .is_empty(),
            "zero bad blobs expected"
        );

        // Verify O(1) embedded clip_box and per-frame SDG RustBox metadata without pixel decode.
        let clip_box = idx
            .clip_box(&bytes)
            .expect("hybrid clip_box")
            .expect("hybrid clip_box present");
        assert_eq!(clip_box.get_str("profile"), Some("sdg_hybrid"));
        assert_eq!(clip_box.get_i64("frames_encoded"), Some(4));
        assert_eq!(clip_box.get_i64("meta_frames_embedded"), Some(4));

        let f2_meta = idx
            .frame_meta(&bytes, 2)
            .expect("hybrid frame 2 meta")
            .expect("hybrid frame 2 meta present");
        assert_eq!(f2_meta.get_i64("exv_frame_index"), Some(2));
        assert_eq!(f2_meta.get_str("occluder_used"), Some("OptiX"));
        assert_eq!(f2_meta.get_i64("camera.width"), Some(i64::from(w)));
        assert_eq!(
            f2_meta.get_path("camera.eye").and_then(|v| v.as_vec3()),
            Some([12.0, 3.0, -5.0])
        );
        assert_eq!(f2_meta.get_str("sdg.frame.scene_id"), Some("gundo_osm"));

        // Verify raw HYBV locator decoding right before index_off + exact video slice equality.
        let index_off = u64::from_le_bytes(
            bytes[bytes.len() - 8..]
                .try_into()
                .expect("8-byte index_off trailer"),
        ) as usize;
        let loc_raw = &bytes[index_off - HYBRID_LOCATOR_LEN..index_off];
        let loc = HybridLocator::decode(loc_raw).expect("HybridLocator::decode");
        assert_eq!(idx.hybrid_locator(), Some(loc));
        loc.verify_crc(&bytes).expect("HybridLocator::verify_crc");
        assert_eq!(
            loc.video_slice(&bytes).expect("video_slice"),
            mock_mp4.as_slice(),
            "embedded ISO-BMFF video bytes must match bit-for-bit"
        );

        // Flip 1 byte in the embedded video segment and prove Verify::On + verify_checksums catch it.
        let mut corrupt = bytes.clone();
        corrupt[loc.video_off as usize + 1234] ^= 0x01;
        assert!(
            VideoIndex::parse_with_verify(&corrupt, Verify::On).is_err(),
            "corrupted HYBV video byte must fail Verify::On"
        );
        let idx_quick = VideoIndex::parse_with_verify(&corrupt, Verify::Off).expect("quick parse");
        assert_eq!(
            verify_checksums(&corrupt, &idx_quick, 8)
                .expect("scan corrupt")
                .len(),
            1
        );

        // Decode all 13 non-RGB AOVs across every frame and assert 100% bit-exact lossless parity.
        let aov_hdr = idx.hybrid_aov_header().expect("hybrid_aov_header");
        let aov_channels = idx.hybrid_aov_channels().expect("hybrid_aov_channels");
        assert_eq!(aov_hdr.channels.len(), EXV_HYBRID_AOV_COUNT);
        assert_eq!(aov_channels, &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let aov_prof = Profile::lossless_wave();

        for (k, src) in frames.iter().enumerate() {
            let di = idx.decode_idx(k).expect("decode_idx");
            let fv = frame_view(&bytes, &idx, di).expect("frame_view");
            let aov_planes = FrameReader::open(fv.body, aov_hdr.clone())
                .expect("FrameReader::open")
                .decode_all(&aov_prof, None)
                .expect("decode_all lossless AOVs");
            assert_eq!(aov_planes.len(), EXV_HYBRID_AOV_COUNT);

            let a_back = read_f16_plane(&aov_planes[0]);
            let alb_r = read_f16_plane(&aov_planes[1]);
            let alb_g = read_f16_plane(&aov_planes[2]);
            let alb_b = read_f16_plane(&aov_planes[3]);
            let nx_back = read_f16_plane(&aov_planes[4]);
            let ny_back = read_f16_plane(&aov_planes[5]);
            let nz_back = read_f16_plane(&aov_planes[6]);
            let rough_back = read_f16_plane(&aov_planes[7]);
            let depth_back = read_f32_plane(&aov_planes[8]);
            let mx_back = read_f32_plane(&aov_planes[9]);
            let my_back = read_f32_plane(&aov_planes[10]);
            let sem_back = read_u32_plane(&aov_planes[11]);
            let inst_back = read_u32_plane(&aov_planes[12]);

            for i in 0..src.color_rgba.len() {
                assert_eq!(a_back[i], f32_to_half(src.color_rgba[i][3]));
                assert_eq!(alb_r[i], f32_to_half(src.albedo_rgba[i][0]));
                assert_eq!(alb_g[i], f32_to_half(src.albedo_rgba[i][1]));
                assert_eq!(alb_b[i], f32_to_half(src.albedo_rgba[i][2]));
                assert_eq!(nx_back[i], f32_to_half(src.normal_rough[i][0]));
                assert_eq!(ny_back[i], f32_to_half(src.normal_rough[i][1]));
                assert_eq!(nz_back[i], f32_to_half(src.normal_rough[i][2]));
                assert_eq!(rough_back[i], f32_to_half(src.normal_rough[i][3]));
                assert_eq!(depth_back[i].to_bits(), src.depth[i].to_bits());
                assert_eq!(mx_back[i].to_bits(), src.motion[i][0].to_bits());
                assert_eq!(my_back[i].to_bits(), src.motion[i][1].to_bits());
                assert_eq!(sem_back[i], src.semantic_id[i]);
                assert_eq!(inst_back[i], src.instance_id[i]);
            }
        }

        std::fs::remove_file(&path).expect("clean hybrid .exv");
    }

    #[test]
    fn hybrid_finish_contract_errors_clean_up_all_temp_files() {
        let p_hyb = std::env::temp_dir().join(format!(
            "nv_demo_exv_hyb_contract_{}.exv",
            std::process::id()
        ));
        let meta = sample_meta(8, 8, ExvProfile::Hybrid);
        let rec = ExvRecorder::start(p_hyb.clone(), 8, 8, ExvProfile::Hybrid, meta)
            .expect("start hybrid");
        assert!(rec.finish().is_err(), "finish() on Hybrid must fail loudly");
        assert!(!p_hyb.exists());
        assert!(!p_hyb.with_extension("exv.tmp").exists());

        let p_std = std::env::temp_dir().join(format!(
            "nv_demo_exv_std_contract_{}.exv",
            std::process::id()
        ));
        let stray_mp4 = hybrid_temp_mp4_path(&p_std);
        std::fs::write(&stray_mp4, b"mock").expect("write stray mp4");
        let meta_std = sample_meta(8, 8, ExvProfile::Fast);
        let rec_std = ExvRecorder::start(p_std.clone(), 8, 8, ExvProfile::Fast, meta_std)
            .expect("start fast");
        assert!(
            rec_std.finish_hybrid(stray_mp4.clone()).is_err(),
            "finish_hybrid() on non-Hybrid must fail loudly"
        );
        assert!(!p_std.exists());
        assert!(!p_std.with_extension("exv.tmp").exists());
        assert!(!stray_mp4.exists(), "stray temp mp4 must be removed");
    }

    #[test]
    fn compact_odd_dimensions_and_embedded_sdg_meta_roundtrip() {
        let (w, h) = (19u32, 13u32);
        let path = std::env::temp_dir().join(format!(
            "nv_demo_exv_odd_meta_{}.exv",
            std::process::id()
        ));
        let meta = sample_meta(w, h, ExvProfile::Compact);
        let rec = ExvRecorder::start(path.clone(), w, h, ExvProfile::Compact, meta)
            .expect("start odd-dim ExvRecorder");

        for k in 0..3u32 {
            let f = make_synth_frame(w, h, k);
            let input = ExvFrameInput {
                width: w,
                height: h,
                color_rgba: &f.color_rgba,
                albedo_rgba: &f.albedo_rgba,
                normal_rough: &f.normal_rough,
                depth: &f.depth,
                motion: &f.motion,
                semantic_id: &f.semantic_id,
                instance_id: &f.instance_id,
                sdg_frame: Some(sample_sdg_frame(w, h, k)),
                sdg_occluder: Some(ActiveSdgOccluder::Cpu),
            };
            rec.submit_frame(&input).expect("submit odd frame");
        }

        let report = rec.finish().expect("finish odd clip");
        assert_eq!(report.meta_frames_embedded, 3);
        assert!(
            !path.with_extension("sdg.json").exists(),
            "no sibling .sdg.json sidecar should be emitted"
        );

        let bytes = std::fs::read(&path).expect("read odd .exv");
        let idx = VideoIndex::parse_with_verify(&bytes, Verify::On)
            .expect("parse+verify odd .exv");
        assert_eq!(idx.frame_count(), 3);
        assert!(
            verify_checksums(&bytes, &idx, 16)
                .expect("verify_checksums")
                .is_empty()
        );

        // O(1) embedded clip_box & per-frame SDG RustBox query without pixel decode.
        let clip_box = idx
            .clip_box(&bytes)
            .expect("odd clip_box")
            .expect("odd clip_box present");
        assert_eq!(clip_box.get_i64("width"), Some(i64::from(w)));
        assert_eq!(clip_box.get_i64("height"), Some(i64::from(h)));
        assert_eq!(clip_box.get_i64("meta_frames_embedded"), Some(3));

        let f2 = idx
            .frame_meta(&bytes, 2)
            .expect("frame 2 meta")
            .expect("frame 2 meta present");
        assert_eq!(f2.get_i64("exv_frame_index"), Some(2));
        assert_eq!(f2.get_str("occluder_used"), Some("CPU"));
        assert_eq!(f2.get_str("scene_id"), Some("gundo_osm"));
        assert_eq!(
            f2.get_path("camera.eye").and_then(|v| v.as_vec3()),
            Some([12.0, 3.0, -5.0])
        );
        assert_eq!(f2.get_i64("sdg.frame.frame_index"), Some(102));

        let dec = decode_frame(&bytes, &idx, 2, Chans::All)
            .expect("decode odd frame 2")
            .into_image()
            .expect("into_image");
        let expected = make_synth_frame(w, h, 2);
        assert_eq!(read_u32_plane(&dec.planes[CH_SEMANTIC_ID]), expected.semantic_id);
        assert_eq!(read_u32_plane(&dec.planes[CH_INSTANCE_ID]), expected.instance_id);

        std::fs::remove_file(&path).expect("clean .exv");
    }

    #[test]
    fn zero_frame_finish_and_abort_on_drop_leave_no_files() {
        let p_empty = std::env::temp_dir().join(format!(
            "nv_demo_exv_empty_{}.exv",
            std::process::id()
        ));
        let meta = sample_meta(8, 8, ExvProfile::Fast);
        let rec = ExvRecorder::start(p_empty.clone(), 8, 8, ExvProfile::Fast, meta.clone())
            .expect("start");
        let err = rec.finish().expect_err("0-frame finish must error");
        assert!(err.contains("0 frames"), "unexpected error: {err}");
        assert!(!p_empty.exists());
        assert!(!p_empty.with_extension("exv.tmp").exists());

        let p_abort = std::env::temp_dir().join(format!(
            "nv_demo_exv_abort_{}.exv",
            std::process::id()
        ));
        let rec = ExvRecorder::start(p_abort.clone(), 8, 8, ExvProfile::Fast, meta)
            .expect("start");
        let slot = rec.acquire_slot().expect("acquire");
        rec.submit_slot(slot).expect("submit");
        drop(rec);
        assert!(!p_abort.exists(), "aborted recorder must not publish .exv");
        assert!(
            !p_abort.with_extension("exv.tmp").exists(),
            "aborted recorder must remove temp file"
        );
    }

    #[test]
    fn buffer_pool_recycles_slots_without_reallocating() {
        let p = std::env::temp_dir().join(format!(
            "nv_demo_exv_pool_{}.exv",
            std::process::id()
        ));
        let meta = sample_meta(16, 16, ExvProfile::Fast);
        let rec = ExvRecorder::start(p.clone(), 16, 16, ExvProfile::Fast, meta).expect("start");

        // Lease all 3 slots, verify pool is exhausted on non-blocking try_acquire, then drop one.
        let s0 = rec.acquire_slot().expect("slot 0");
        let s1 = rec.acquire_slot().expect("slot 1");
        let s2 = rec.acquire_slot().expect("slot 2");
        let ptr0 = s0.bufs.color_rgba.as_ptr();
        assert!(rec.try_acquire_slot().expect("try").is_none());

        // Early drop returns slot 0 immediately with the exact same backing allocation.
        drop(s0);
        let s0_again = rec
            .try_acquire_slot()
            .expect("try after drop")
            .expect("slot 0 recycled");
        assert_eq!(s0_again.bufs.color_rgba.as_ptr(), ptr0);

        // Submit through the encoder and verify recycling continues smoothly.
        rec.submit_slot(s0_again).expect("submit 0");
        rec.submit_slot(s1).expect("submit 1");
        rec.submit_slot(s2).expect("submit 2");
        let s3 = rec.acquire_slot().expect("recycled after encode");
        rec.submit_slot(s3).expect("submit 3");

        let report = rec.finish().expect("finish");
        assert_eq!(report.frames_encoded, 4);
        std::fs::remove_file(&p).expect("clean .exv");
    }

    #[test]
    fn profile_serde_and_output_path_sanitization() {
        for p in ExvProfile::ALL {
            let s = serde_json::to_string(&p).expect("serialize");
            let back: ExvProfile = serde_json::from_str(&s).expect("deserialize");
            assert_eq!(back, p);
            assert!(!p.label().is_empty());
            assert!(!p.description().is_empty());
        }
        let path = default_exv_output_path("SF House/of AI:v2");
        let name = path.file_name().and_then(|n| n.to_str()).expect("fname");
        assert!(name.starts_with("nv_demo_SF_House_of_AI_v2_"));
        assert!(name.ends_with(".exv"));
    }
}
