//! Zero-dependency ISO BMFF (`.mp4`) muxer for iPhone / QuickTime–compatible H.264 SDR and
//! HEVC 10-bit HDR10 (PQ) streams.
//!
//! **What.**
//! - Converts NVENC Annex-B bitstreams (`00 00 01` / `00 00 00 01` start codes) into 4-byte
//!   length-prefixed AVCC/HVCC samples streamed directly into a 64-bit `mdat` box.
//! - Extracts codec parameter sets (`SPS`/`PPS` for H.264 `avcC`; `VPS`/`SPS`/`PPS` for HEVC
//!   `hvcC`) and builds a complete `moov` atom on [`Mp4Muxer::finish`].
//! - Writes Apple/iOS-compatible sample entries (`avc1` with BT.709 `colr`, or `hvc1` with
//!   BT.2020 PQ `colr` + `mdcv` + `clli`) and per-frame `stts` durations at `90_000` Hz so
//!   variable-frame-rate recordings play back at true real-time speed without cartoon speedup.
//! - Writes atomically: bytes go to a hidden `.<name>.<pid>-<seq>.partial` file in the target
//!   directory, which is flushed, `sync_all`ed and renamed onto the final path only by a
//!   successful [`Mp4Muxer::finish`]. Dropping the muxer without finishing, or a failed finish
//!   (including a recording with zero samples), removes the partial file, so an unplayable file
//!   never appears at the final path.
//!
//! **Used by** `gpu_export.rs` (`VideoRecorder`).

use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Standard MPEG video timescale (90 kHz: 60 fps = 1500 ticks, 30 fps = 3000 ticks).
pub const MP4_TIMESCALE: u32 = 90_000;

/// HDR10 mastering display peak luminance in `mdcv` units of 0.0001 cd/m^2 (1000 cd/m^2).
const MDCV_MAX_LUMINANCE: u32 = 10_000_000;
/// HDR10 mastering display black level in `mdcv` units of 0.0001 cd/m^2 (0.005 cd/m^2).
const MDCV_MIN_LUMINANCE: u32 = 50;
/// Nominal `clli` MaxCLL (cd/m^2). Not measured from the content: the muxer never sees pixels.
const CLLI_MAX_CLL: u16 = 1000;
/// Nominal `clli` MaxFALL (cd/m^2). Not measured from the content: the muxer never sees pixels.
const CLLI_MAX_FALL: u16 = 400;

/// Error returned by [`Mp4Muxer::finish`] when no video sample was ever written.
pub const EMPTY_MP4_ERROR: &str = "MP4 recording has no video samples; no file was written";

/// Per-process sequence number that keeps concurrent partial file names unique.
static PARTIAL_SEQ: AtomicU64 = AtomicU64::new(0);

/// Owner of the in-progress partial file: removes it on drop unless [`Self::commit`] renamed it.
///
/// Declared after the writer in [`Mp4Muxer`] so the file handle is closed before removal.
struct PartialFile {
    tmp: PathBuf,
    dst: PathBuf,
    armed: bool,
}

impl PartialFile {
    /// Builds a unique hidden sibling path of `dst` (same directory, so the rename is atomic).
    fn sibling_of(dst: &Path) -> Result<PathBuf, String> {
        let name = dst
            .file_name()
            .ok_or_else(|| format!("MP4 output path '{}' has no file name", dst.display()))?;
        let seq = PARTIAL_SEQ.fetch_add(1, Ordering::Relaxed);
        let mut tmp_name = OsString::from(".");
        tmp_name.push(name);
        tmp_name.push(format!(".{}-{seq}.partial", std::process::id()));
        Ok(dst.with_file_name(tmp_name))
    }

    /// Atomically moves the finished partial file onto the final path.
    fn commit(&mut self) -> Result<(), String> {
        std::fs::rename(&self.tmp, &self.dst).map_err(|e| {
            format!(
                "failed to rename MP4 '{}' -> '{}': {e}",
                self.tmp.display(),
                self.dst.display()
            )
        })?;
        self.armed = false;
        Ok(())
    }
}

impl Drop for PartialFile {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        if let Err(e) = std::fs::remove_file(&self.tmp)
            && e.kind() != std::io::ErrorKind::NotFound
        {
            // Drop cannot return the error; report it like the other export-worker failures.
            eprintln!(
                "nv-demo: failed to remove partial MP4 '{}': {e}",
                self.tmp.display()
            );
        }
    }
}

/// Iterator over raw NAL unit payloads (start codes excluded) of an Annex-B byte stream.
///
/// Borrows the input and never allocates, so the per-frame record path stays heap-free.
struct AnnexbNals<'a> {
    data: &'a [u8],
    /// `(start_code_pos, payload_pos)` of the next NAL unit, if any.
    next: Option<(usize, usize)>,
}

fn annexb_nals(data: &[u8]) -> AnnexbNals<'_> {
    AnnexbNals {
        data,
        next: find_start_code(data, 0),
    }
}

/// Finds the first `00 00 01` or `00 00 00 01` start code at or after `from`.
fn find_start_code(data: &[u8], from: usize) -> Option<(usize, usize)> {
    let mut i = from;
    while let Some(rest) = data.get(i..) {
        match rest {
            [0, 0, 1, ..] => return Some((i, i + 3)),
            [0, 0, 0, 1, ..] => return Some((i, i + 4)),
            [_, _, _, ..] => i += 1,
            _ => return None,
        }
    }
    None
}

impl<'a> Iterator for AnnexbNals<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        loop {
            let (_, start) = self.next?;
            let following = find_start_code(self.data, start);
            let end = following.map_or(self.data.len(), |(code_pos, _)| code_pos);
            self.next = following;
            if let Some(nal) = self.data.get(start..end)
                && !nal.is_empty()
            {
                return Some(nal);
            }
        }
    }
}

/// Codec & color profile written to the MP4 container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoProfile {
    /// 8-bit H.264 High (`avc1`) with BT.709 limited-range `colr` signaling.
    SdrH264,
    /// 10-bit HEVC Main10 (`hvc1`) with HDR10 BT.2020 SMPTE ST 2084 PQ (`colr` + `mdcv` + `clli`).
    Hdr10Pq,
}

/// Streaming ISO BMFF (`.mp4`) writer backed by a partial file renamed on [`Self::finish`].
pub struct Mp4Muxer {
    // Field order is load-bearing: `writer` drops (closing the file) before `partial` removes it.
    writer: BufWriter<File>,
    partial: PartialFile,
    profile: VideoProfile,
    width: u16,
    height: u16,
    mdat_header_offset: u64,
    mdat_bytes_written: u64,
    /// Extracted H.264/HEVC parameter sets (first seen).
    vps: Option<Vec<u8>>,
    sps: Option<Vec<u8>>,
    pps: Option<Vec<u8>>,
    /// Per-sample byte sizes, file offsets, durations (90 kHz ticks), and 1-based sync indices.
    sample_sizes: Vec<u32>,
    sample_offsets: Vec<u64>,
    sample_durations: Vec<u32>,
    sync_samples: Vec<u32>,
    total_duration_ticks: u64,
    /// Duration of access units that produced no sample, carried into the next sample.
    pending_ticks: u64,
    /// Reused AVCC sample assembly buffer (no per-frame allocation once warmed up).
    sample_buf: Vec<u8>,
}

impl Mp4Muxer {
    /// Creates a partial file next to `path`, writes the `ftyp` box and a 64-bit `mdat` header,
    /// and prepares for frames. Nothing appears at `path` until [`Self::finish`] succeeds.
    pub fn create(
        path: &Path,
        width: u32,
        height: u32,
        profile: VideoProfile,
    ) -> Result<Self, String> {
        let w = u16::try_from(width)
            .map_err(|_| format!("MP4 width {width} exceeds u16::MAX"))?;
        let h = u16::try_from(height)
            .map_err(|_| format!("MP4 height {height} exceeds u16::MAX"))?;
        if w == 0 || h == 0 {
            return Err("MP4 dimensions must be non-zero".to_string());
        }

        let tmp = PartialFile::sibling_of(path)?;
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .map_err(|e| format!("failed to create MP4 file '{}': {e}", tmp.display()))?;

        let ftyp = build_ftyp(profile);
        let mdat_header_offset = ftyp.len() as u64;

        // Build `Self` before writing so any later error drops the writer, then the partial file.
        let mut mux = Self {
            writer: BufWriter::new(file),
            partial: PartialFile {
                tmp,
                dst: path.to_path_buf(),
                armed: true,
            },
            profile,
            width: w,
            height: h,
            mdat_header_offset,
            mdat_bytes_written: 0,
            vps: None,
            sps: None,
            pps: None,
            sample_sizes: Vec::new(),
            sample_offsets: Vec::new(),
            sample_durations: Vec::new(),
            sync_samples: Vec::new(),
            total_duration_ticks: 0,
            pending_ticks: 0,
            sample_buf: Vec::new(),
        };

        mux.writer
            .write_all(&ftyp)
            .map_err(|e| format!("failed to write MP4 ftyp: {e}"))?;
        // 64-bit extended-size mdat header: size=1, type="mdat", largesize=placeholder (patched on finish).
        let mut mdat_hdr = [0u8; 16];
        mdat_hdr[0..4].copy_from_slice(&1u32.to_be_bytes());
        mdat_hdr[4..8].copy_from_slice(b"mdat");
        mdat_hdr[8..16].copy_from_slice(&16u64.to_be_bytes());
        mux.writer
            .write_all(&mdat_hdr)
            .map_err(|e| format!("failed to write MP4 mdat header: {e}"))?;

        Ok(mux)
    }

    /// Appends one Annex-B encoded access unit with duration `duration_ticks` (in 90 kHz ticks).
    ///
    /// An access unit without a slice NAL (empty, or parameter sets / AUD only) adds no sample;
    /// its duration is carried into the next sample (or the last one on finish) so the container
    /// duration still matches the recorded time.
    pub fn write_annexb_frame(
        &mut self,
        annexb: &[u8],
        duration_ticks: u32,
    ) -> Result<(), String> {
        let duration = duration_ticks.max(1);
        let mut is_sync = false;
        self.sample_buf.clear();

        for nal in annexb_nals(annexb) {
            let Some(&hdr) = nal.first() else {
                continue;
            };
            match self.profile {
                VideoProfile::SdrH264 => {
                    let nal_type = hdr & 0x1F;
                    match nal_type {
                        7 => {
                            if self.sps.is_none() {
                                self.sps = Some(nal.to_vec());
                            }
                            continue;
                        }
                        8 => {
                            if self.pps.is_none() {
                                self.pps = Some(nal.to_vec());
                            }
                            continue;
                        }
                        9 => continue, // Access Unit Delimiter not required in AVCC samples
                        5 => is_sync = true,
                        _ => {}
                    }
                }
                VideoProfile::Hdr10Pq => {
                    if nal.len() < 2 {
                        continue;
                    }
                    let nal_type = (hdr >> 1) & 0x3F;
                    match nal_type {
                        32 => {
                            if self.vps.is_none() {
                                self.vps = Some(nal.to_vec());
                            }
                            continue;
                        }
                        33 => {
                            if self.sps.is_none() {
                                self.sps = Some(nal.to_vec());
                            }
                            continue;
                        }
                        34 => {
                            if self.pps.is_none() {
                                self.pps = Some(nal.to_vec());
                            }
                            continue;
                        }
                        35 => continue, // AUD
                        19..=21 => is_sync = true, // IDR_W_RADL / IDR_N_LP / CRA_NUT
                        _ => {}
                    }
                }
            }

            let nal_len = u32::try_from(nal.len())
                .map_err(|_| format!("NAL unit size {} exceeds u32::MAX", nal.len()))?;
            self.sample_buf.extend_from_slice(&nal_len.to_be_bytes());
            self.sample_buf.extend_from_slice(nal);
        }

        if self.sample_buf.is_empty() {
            self.pending_ticks += u64::from(duration);
            return Ok(());
        }

        let sample_size = u32::try_from(self.sample_buf.len())
            .map_err(|_| format!("MP4 sample size {} exceeds u32::MAX", self.sample_buf.len()))?;
        let carried = u64::from(duration) + self.pending_ticks;
        let duration = u32::try_from(carried)
            .map_err(|_| format!("MP4 sample duration {carried} ticks exceeds u32::MAX"))?;
        let offset = self.mdat_header_offset + 16 + self.mdat_bytes_written;
        self.writer
            .write_all(&self.sample_buf)
            .map_err(|e| format!("failed to write MP4 sample: {e}"))?;
        self.mdat_bytes_written += u64::from(sample_size);
        self.pending_ticks = 0;

        let sample_num_1based = (self.sample_sizes.len() as u32).saturating_add(1);
        self.sample_sizes.push(sample_size);
        self.sample_offsets.push(offset);
        self.sample_durations.push(duration);
        if is_sync {
            self.sync_samples.push(sample_num_1based);
        }
        self.total_duration_ticks += u64::from(duration);
        Ok(())
    }

    /// Number of video samples written so far.
    pub fn frame_count(&self) -> u64 {
        self.sample_sizes.len() as u64
    }

    /// Writes the `moov` box, patches the `mdat` size header, flushes and `sync_all`s the partial
    /// file, then renames it onto the final path.
    ///
    /// Returns [`EMPTY_MP4_ERROR`] when no sample was written. On any error the partial file is
    /// removed and nothing is left at the final path.
    pub fn finish(mut self) -> Result<u64, String> {
        let count = self.sample_sizes.len() as u64;
        if count == 0 {
            return Err(EMPTY_MP4_ERROR.to_string());
        }
        if self.pending_ticks > 0 {
            let pending = self.pending_ticks;
            if let Some(last) = self.sample_durations.last_mut() {
                let carried = u64::from(*last) + pending;
                *last = u32::try_from(carried)
                    .map_err(|_| format!("MP4 sample duration {carried} ticks exceeds u32::MAX"))?;
            }
            self.total_duration_ticks += pending;
            self.pending_ticks = 0;
        }

        let moov = self.build_moov()?;
        self.writer
            .write_all(&moov)
            .map_err(|e| format!("failed to write MP4 moov: {e}"))?;
        self.writer
            .flush()
            .map_err(|e| format!("failed to flush MP4 before patch: {e}"))?;

        let mdat_total = 16u64 + self.mdat_bytes_written;
        self.writer
            .seek(SeekFrom::Start(self.mdat_header_offset + 8))
            .map_err(|e| format!("failed to seek MP4 mdat size: {e}"))?;
        self.writer
            .write_all(&mdat_total.to_be_bytes())
            .map_err(|e| format!("failed to patch MP4 mdat size: {e}"))?;
        let file = self
            .writer
            .into_inner()
            .map_err(|e| format!("failed to flush finalized MP4: {}", e.error()))?;
        file.sync_all()
            .map_err(|e| format!("failed to sync finalized MP4: {e}"))?;
        // Close the handle before the rename so no open handle pins the partial file.
        drop(file);
        self.partial.commit()?;
        Ok(count)
    }

    fn build_moov(&self) -> Result<Vec<u8>, String> {
        let duration = self.total_duration_ticks;
        let mvhd = build_mvhd(MP4_TIMESCALE, duration);
        let tkhd = build_tkhd(duration, self.width, self.height);
        let mdhd = build_mdhd(MP4_TIMESCALE, duration);
        let hdlr = build_hdlr();
        let vmhd = build_vmhd();
        let dinf = build_dinf();

        let stsd = match self.profile {
            VideoProfile::SdrH264 => {
                let sps = self
                    .sps
                    .as_deref()
                    .ok_or_else(|| "missing H.264 SPS for avcC".to_string())?;
                let pps = self
                    .pps
                    .as_deref()
                    .ok_or_else(|| "missing H.264 PPS for avcC".to_string())?;
                build_stsd_avc1(self.width, self.height, sps, pps)?
            }
            VideoProfile::Hdr10Pq => {
                let vps = self
                    .vps
                    .as_deref()
                    .ok_or_else(|| "missing HEVC VPS for hvcC".to_string())?;
                let sps = self
                    .sps
                    .as_deref()
                    .ok_or_else(|| "missing HEVC SPS for hvcC".to_string())?;
                let pps = self
                    .pps
                    .as_deref()
                    .ok_or_else(|| "missing HEVC PPS for hvcC".to_string())?;
                build_stsd_hvc1(self.width, self.height, vps, sps, pps)
            }
        };

        let stts = build_stts(&self.sample_durations);
        let stsc = build_stsc_one_sample_per_chunk();
        let stsz = build_stsz(&self.sample_sizes);
        let co64 = build_co64(&self.sample_offsets);

        let mut stbl_body = Vec::new();
        stbl_body.extend_from_slice(&stsd);
        stbl_body.extend_from_slice(&stts);
        if !self.sync_samples.is_empty() {
            stbl_body.extend_from_slice(&build_stss(&self.sync_samples));
        }
        stbl_body.extend_from_slice(&stsc);
        stbl_body.extend_from_slice(&stsz);
        stbl_body.extend_from_slice(&co64);
        let stbl = make_box(b"stbl", &stbl_body);

        let mut minf_body = Vec::new();
        minf_body.extend_from_slice(&vmhd);
        minf_body.extend_from_slice(&dinf);
        minf_body.extend_from_slice(&stbl);
        let minf = make_box(b"minf", &minf_body);

        let mut mdia_body = Vec::new();
        mdia_body.extend_from_slice(&mdhd);
        mdia_body.extend_from_slice(&hdlr);
        mdia_body.extend_from_slice(&minf);
        let mdia = make_box(b"mdia", &mdia_body);

        let mut trak_body = Vec::new();
        trak_body.extend_from_slice(&tkhd);
        trak_body.extend_from_slice(&mdia);
        let trak = make_box(b"trak", &trak_body);

        let mut moov_body = Vec::new();
        moov_body.extend_from_slice(&mvhd);
        moov_body.extend_from_slice(&trak);
        Ok(make_box(b"moov", &moov_body))
    }
}

/// Removes H.264/HEVC emulation prevention bytes (`00 00 03` -> `00 00`).
fn strip_epb(nal: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(nal.len());
    let mut zeros = 0u32;
    for &b in nal {
        if zeros >= 2 && b == 3 {
            zeros = 0;
            continue;
        }
        out.push(b);
        if b == 0 {
            zeros += 1;
        } else {
            zeros = 0;
        }
    }
    out
}

fn make_box(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let size = (8 + payload.len()) as u32;
    let mut v = Vec::with_capacity(size as usize);
    v.extend_from_slice(&size.to_be_bytes());
    v.extend_from_slice(fourcc);
    v.extend_from_slice(payload);
    v
}

fn make_full_box(fourcc: &[u8; 4], version: u8, flags: u32, payload: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + payload.len());
    let vf = (u32::from(version) << 24) | (flags & 0x00FF_FFFF);
    body.extend_from_slice(&vf.to_be_bytes());
    body.extend_from_slice(payload);
    make_box(fourcc, &body)
}

fn build_ftyp(profile: VideoProfile) -> Vec<u8> {
    let mut body = Vec::with_capacity(24);
    match profile {
        VideoProfile::SdrH264 => {
            body.extend_from_slice(b"isom");
            body.extend_from_slice(&0x0000_0200u32.to_be_bytes());
            body.extend_from_slice(b"isom");
            body.extend_from_slice(b"iso2");
            body.extend_from_slice(b"avc1");
            body.extend_from_slice(b"mp41");
        }
        VideoProfile::Hdr10Pq => {
            body.extend_from_slice(b"mp42");
            body.extend_from_slice(&0u32.to_be_bytes());
            body.extend_from_slice(b"mp42");
            body.extend_from_slice(b"mp41");
            body.extend_from_slice(b"isom");
            body.extend_from_slice(b"hvc1");
        }
    }
    make_box(b"ftyp", &body)
}

/// Writes the full-box version-0 (32-bit) or version-1 (64-bit) `creation_time`,
/// `modification_time` pair and returns the version: version 1 is needed once `duration`
/// exceeds `u32::MAX` (ISO/IEC 14496-12 `mvhd` / `tkhd` / `mdhd`).
fn push_times(body: &mut Vec<u8>, duration: u64) -> u8 {
    if u32::try_from(duration).is_ok() {
        body.extend_from_slice(&0u32.to_be_bytes()); // creation_time
        body.extend_from_slice(&0u32.to_be_bytes()); // modification_time
        0
    } else {
        body.extend_from_slice(&0u64.to_be_bytes()); // creation_time
        body.extend_from_slice(&0u64.to_be_bytes()); // modification_time
        1
    }
}

/// Writes `duration` with the width selected by [`push_times`] for `version`.
fn push_duration(body: &mut Vec<u8>, version: u8, duration: u64) {
    match u32::try_from(duration) {
        Ok(d32) if version == 0 => body.extend_from_slice(&d32.to_be_bytes()),
        _ => body.extend_from_slice(&duration.to_be_bytes()),
    }
}

fn build_mvhd(timescale: u32, duration: u64) -> Vec<u8> {
    let mut body = Vec::with_capacity(108);
    let version = push_times(&mut body, duration);
    body.extend_from_slice(&timescale.to_be_bytes());
    push_duration(&mut body, version, duration);
    body.extend_from_slice(&0x0001_0000u32.to_be_bytes()); // rate 1.0
    body.extend_from_slice(&0x0100u16.to_be_bytes()); // volume 1.0
    body.extend_from_slice(&[0u8; 10]); // reserved
    // Unity 3x3 matrix
    for v in [0x0001_0000u32, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000] {
        body.extend_from_slice(&v.to_be_bytes());
    }
    body.extend_from_slice(&[0u8; 24]); // pre_defined
    body.extend_from_slice(&2u32.to_be_bytes()); // next_track_ID
    make_full_box(b"mvhd", version, 0, &body)
}

fn build_tkhd(duration: u64, width: u16, height: u16) -> Vec<u8> {
    let mut body = Vec::with_capacity(92);
    let version = push_times(&mut body, duration);
    body.extend_from_slice(&1u32.to_be_bytes()); // track_ID = 1
    body.extend_from_slice(&0u32.to_be_bytes()); // reserved
    push_duration(&mut body, version, duration);
    body.extend_from_slice(&[0u8; 8]); // reserved
    body.extend_from_slice(&0u16.to_be_bytes()); // layer
    body.extend_from_slice(&0u16.to_be_bytes()); // alternate_group
    body.extend_from_slice(&0u16.to_be_bytes()); // volume (video = 0)
    body.extend_from_slice(&0u16.to_be_bytes()); // reserved
    for v in [0x0001_0000u32, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000] {
        body.extend_from_slice(&v.to_be_bytes());
    }
    body.extend_from_slice(&(u32::from(width) << 16).to_be_bytes());
    body.extend_from_slice(&(u32::from(height) << 16).to_be_bytes());
    // flags = 0x000003 (track_enabled | track_in_movie)
    make_full_box(b"tkhd", version, 0x0000_0003, &body)
}

fn build_mdhd(timescale: u32, duration: u64) -> Vec<u8> {
    let mut body = Vec::with_capacity(32);
    let version = push_times(&mut body, duration);
    body.extend_from_slice(&timescale.to_be_bytes());
    push_duration(&mut body, version, duration);
    body.extend_from_slice(&0x55C4u16.to_be_bytes()); // language = "und"
    body.extend_from_slice(&0u16.to_be_bytes()); // pre_defined
    make_full_box(b"mdhd", version, 0, &body)
}

fn build_hdlr() -> Vec<u8> {
    let mut body = Vec::with_capacity(32);
    body.extend_from_slice(&0u32.to_be_bytes()); // pre_defined
    body.extend_from_slice(b"vide");
    body.extend_from_slice(&[0u8; 12]); // reserved
    body.extend_from_slice(b"VideoHandler\0");
    make_full_box(b"hdlr", 0, 0, &body)
}

fn build_vmhd() -> Vec<u8> {
    let body = [0u8; 8]; // graphicsmode=0, opcolor=[0,0,0]
    make_full_box(b"vmhd", 0, 1, &body)
}

fn build_dinf() -> Vec<u8> {
    let url = make_full_box(b"url ", 0, 1, &[]); // self-contained flag
    let mut dref_body = Vec::with_capacity(4 + url.len());
    dref_body.extend_from_slice(&1u32.to_be_bytes());
    dref_body.extend_from_slice(&url);
    let dref = make_full_box(b"dref", 0, 0, &dref_body);
    make_box(b"dinf", &dref)
}

fn build_visual_sample_entry(
    fourcc: &[u8; 4],
    width: u16,
    height: u16,
    extra_boxes: &[u8],
) -> Vec<u8> {
    let mut body = Vec::with_capacity(78 + extra_boxes.len());
    body.extend_from_slice(&[0u8; 6]); // reserved
    body.extend_from_slice(&1u16.to_be_bytes()); // data_reference_index
    body.extend_from_slice(&[0u8; 16]); // pre_defined + reserved
    body.extend_from_slice(&width.to_be_bytes());
    body.extend_from_slice(&height.to_be_bytes());
    body.extend_from_slice(&0x0048_0000u32.to_be_bytes()); // horizresolution 72 dpi
    body.extend_from_slice(&0x0048_0000u32.to_be_bytes()); // vertresolution 72 dpi
    body.extend_from_slice(&0u32.to_be_bytes()); // reserved
    body.extend_from_slice(&1u16.to_be_bytes()); // frame_count
    body.extend_from_slice(&[0u8; 32]); // compressorname
    body.extend_from_slice(&0x0018u16.to_be_bytes()); // depth = 24
    body.extend_from_slice(&0xFFFFu16.to_be_bytes()); // pre_defined = -1
    body.extend_from_slice(extra_boxes);
    make_box(fourcc, &body)
}

fn build_colr_nclx(primaries: u16, transfer: u16, matrix: u16, full_range: bool) -> Vec<u8> {
    let mut body = Vec::with_capacity(11);
    body.extend_from_slice(b"nclx");
    body.extend_from_slice(&primaries.to_be_bytes());
    body.extend_from_slice(&transfer.to_be_bytes());
    body.extend_from_slice(&matrix.to_be_bytes());
    body.push(if full_range { 0x80 } else { 0x00 });
    make_box(b"colr", &body)
}

/// Mastering Display Color Volume (`mdcv`, SMPTE ST 2086) for a BT.2020 mastering display with
/// [`MDCV_MAX_LUMINANCE`] peak (1000 cd/m^2) and [`MDCV_MIN_LUMINANCE`] black level (0.005 cd/m^2).
fn build_mdcv_bt2020_1000nit() -> Vec<u8> {
    // Chromaticities in 0.00002 units: G(0.170, 0.797), B(0.131, 0.046), R(0.708, 0.292), D65(0.3127, 0.3290)
    let mut body = Vec::with_capacity(24);
    for v in [
        8500u16, 39850, // Green
        6550, 2300, // Blue
        35400, 14600, // Red
        15635, 16450, // White point D65
    ] {
        body.extend_from_slice(&v.to_be_bytes());
    }
    body.extend_from_slice(&MDCV_MAX_LUMINANCE.to_be_bytes());
    body.extend_from_slice(&MDCV_MIN_LUMINANCE.to_be_bytes());
    make_box(b"mdcv", &body)
}

/// Content Light Level Information (`clli`): MaxCLL and MaxFALL in cd/m^2.
fn build_clli(max_cll: u16, max_fall: u16) -> Vec<u8> {
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&max_cll.to_be_bytes());
    body.extend_from_slice(&max_fall.to_be_bytes());
    make_box(b"clli", &body)
}

/// MSB-first bit reader over an RBSP for the few Exp-Golomb SPS fields `avcC` needs.
struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl BitReader<'_> {
    fn bit(&mut self) -> Option<u32> {
        let byte = *self.data.get(self.bit_pos / 8)?;
        let bit = (byte >> (7 - (self.bit_pos % 8))) & 1;
        self.bit_pos += 1;
        Some(u32::from(bit))
    }

    /// Unsigned Exp-Golomb `ue(v)` (ITU-T H.264 9.1).
    fn ue(&mut self) -> Option<u32> {
        let mut zeros = 0u32;
        while self.bit()? == 0 {
            zeros += 1;
            if zeros >= 32 {
                return None;
            }
        }
        let mut info = 0u32;
        for _ in 0..zeros {
            info = (info << 1) | self.bit()?;
        }
        Some((1u32 << zeros) - 1 + info)
    }
}

/// Profiles whose SPS carries `chroma_format_idc` and bit depths (H.264 7.3.2.1.1; same list as
/// FFmpeg `ff_avc_decode_sps`).
const AVC_PROFILES_WITH_CHROMA_INFO: [u8; 12] =
    [100, 110, 122, 244, 44, 83, 86, 118, 128, 138, 139, 134];

/// Parses `(chroma_format_idc, bit_depth_luma_minus8, bit_depth_chroma_minus8)` from an H.264
/// SPS NAL (header byte included); profiles without these fields imply 4:2:0 8-bit.
fn parse_avc_chroma_depth(sps: &[u8]) -> Result<(u8, u8, u8), String> {
    let rbsp = strip_epb(sps);
    let profile_idc = *rbsp
        .get(1)
        .ok_or_else(|| "H.264 SPS too short for avcC".to_string())?;
    if !AVC_PROFILES_WITH_CHROMA_INFO.contains(&profile_idc) {
        return Ok((1, 0, 0));
    }
    let truncated = || "H.264 SPS truncated before bit depth fields".to_string();
    // Skip NAL header, profile_idc, constraint flags and level_idc (4 bytes).
    let mut br = BitReader {
        data: &rbsp,
        bit_pos: 32,
    };
    br.ue().ok_or_else(truncated)?; // seq_parameter_set_id
    let chroma = br.ue().ok_or_else(truncated)?;
    if chroma == 3 {
        br.bit().ok_or_else(truncated)?; // separate_colour_plane_flag
    }
    let luma_m8 = br.ue().ok_or_else(truncated)?;
    let chroma_m8 = br.ue().ok_or_else(truncated)?;
    // The avcC fields are 2 / 3 / 3 bits wide.
    match (u8::try_from(chroma), u8::try_from(luma_m8), u8::try_from(chroma_m8)) {
        (Ok(c), Ok(l), Ok(cd)) if c <= 3 && l <= 7 && cd <= 7 => Ok((c, l, cd)),
        _ => Err(format!(
            "H.264 SPS has invalid chroma_format_idc {chroma} or bit depths {luma_m8}/{chroma_m8}"
        )),
    }
}

fn build_stsd_avc1(width: u16, height: u16, sps: &[u8], pps: &[u8]) -> Result<Vec<u8>, String> {
    let &[_, profile_idc, profile_compat, level_idc, ..] = sps else {
        return Err(format!("H.264 SPS of {} bytes is too short for avcC", sps.len()));
    };
    let sps_len = u16::try_from(sps.len())
        .map_err(|_| format!("H.264 SPS of {} bytes exceeds avcC u16 length", sps.len()))?;
    let pps_len = u16::try_from(pps.len())
        .map_err(|_| format!("H.264 PPS of {} bytes exceeds avcC u16 length", pps.len()))?;

    let mut avcc_body = Vec::with_capacity(15 + sps.len() + pps.len());
    avcc_body.push(1); // configurationVersion
    avcc_body.push(profile_idc);
    avcc_body.push(profile_compat);
    avcc_body.push(level_idc);
    avcc_body.push(0xFF); // 6 bits reserved (1) + lengthSizeMinusOne (3 => 4 bytes)
    avcc_body.push(0xE1); // 3 bits reserved (1) + numOfSequenceParameterSets (1)
    avcc_body.extend_from_slice(&sps_len.to_be_bytes());
    avcc_body.extend_from_slice(sps);
    avcc_body.push(1); // numOfPictureParameterSets
    avcc_body.extend_from_slice(&pps_len.to_be_bytes());
    avcc_body.extend_from_slice(pps);
    // ISO/IEC 14496-15 extension for non-Baseline/Main/Extended profiles (as FFmpeg
    // `ff_isom_write_avcc` writes it): reserved bits set, no SPS extension NAL units.
    if !matches!(profile_idc, 66 | 77 | 88) {
        let (chroma, luma_m8, chroma_m8) = parse_avc_chroma_depth(sps)?;
        avcc_body.push(0xFC | chroma);
        avcc_body.push(0xF8 | luma_m8);
        avcc_body.push(0xF8 | chroma_m8);
        avcc_body.push(0); // numOfSequenceParameterSetExt
    }
    let avcc = make_box(b"avcC", &avcc_body);

    // BT.709 limited-range `colr` box (primaries=1, transfer=1, matrix=1, full_range=false).
    let colr = build_colr_nclx(1, 1, 1, false);

    let mut extra = Vec::with_capacity(avcc.len() + colr.len());
    extra.extend_from_slice(&avcc);
    extra.extend_from_slice(&colr);

    let entry = build_visual_sample_entry(b"avc1", width, height, &extra);
    let mut stsd_body = Vec::with_capacity(4 + entry.len());
    stsd_body.extend_from_slice(&1u32.to_be_bytes());
    stsd_body.extend_from_slice(&entry);
    Ok(make_full_box(b"stsd", 0, 0, &stsd_body))
}

fn build_stsd_hvc1(width: u16, height: u16, vps: &[u8], sps: &[u8], pps: &[u8]) -> Vec<u8> {
    // Extract 12-byte profile_tier_level from SPS RBSP (starts at byte 3 after 2B NAL header + 1B VPS/sublayer byte).
    let rbsp = strip_epb(sps);
    let (ptl_byte0, ptl_compat, ptl_constraint, general_level_idc) = if rbsp.len() >= 15 {
        let mut compat = [0u8; 4];
        compat.copy_from_slice(&rbsp[4..8]);
        let mut constraint = [0u8; 6];
        constraint.copy_from_slice(&rbsp[8..14]);
        (rbsp[3], compat, constraint, rbsp[14])
    } else {
        // Fallback: Main 10 profile (idc = 2), Level 5.1 (153)
        (0x02u8, [0x20, 0x00, 0x00, 0x00], [0x90, 0, 0, 0, 0, 0], 153u8)
    };

    let mut hvcc_body = Vec::with_capacity(48 + vps.len() + sps.len() + pps.len());
    hvcc_body.push(1); // configurationVersion
    hvcc_body.push(ptl_byte0); // general_profile_space(2) | tier(1) | profile_idc(5)
    hvcc_body.extend_from_slice(&ptl_compat);
    hvcc_body.extend_from_slice(&ptl_constraint);
    hvcc_body.push(general_level_idc);
    hvcc_body.extend_from_slice(&0xF000u16.to_be_bytes()); // min_spatial_segmentation_idc = 0
    hvcc_body.push(0xFC); // parallelismType = 0
    hvcc_body.push(0xFD); // chromaFormat = 1 (4:2:0)
    hvcc_body.push(0xFA); // bitDepthLumaMinus8 = 2 (10-bit)
    hvcc_body.push(0xFA); // bitDepthChromaMinus8 = 2 (10-bit)
    hvcc_body.extend_from_slice(&0u16.to_be_bytes()); // avgFrameRate = 0 (VFR / unspecified)
    hvcc_body.push(0x0F); // constantFrameRate=0, numTemporalLayers=1, temporalIdNested=1, lengthSizeMinusOne=3 (4B)
    hvcc_body.push(3); // numOfArrays: VPS, SPS, PPS

    for (nal_type, data) in [(32u8, vps), (33u8, sps), (34u8, pps)] {
        hvcc_body.push(0x80 | (nal_type & 0x3F)); // array_completeness=1
        hvcc_body.extend_from_slice(&1u16.to_be_bytes()); // numNalus = 1
        hvcc_body.extend_from_slice(&(data.len() as u16).to_be_bytes());
        hvcc_body.extend_from_slice(data);
    }
    let hvcc = make_box(b"hvcC", &hvcc_body);

    // BT.2020 SMPTE ST 2084 PQ limited-range `colr` + HDR10 static metadata (`mdcv`, `clli`).
    let colr = build_colr_nclx(9, 16, 9, false);
    let mdcv = build_mdcv_bt2020_1000nit();
    let clli = build_clli(CLLI_MAX_CLL, CLLI_MAX_FALL);

    let mut extra = Vec::with_capacity(hvcc.len() + colr.len() + mdcv.len() + clli.len());
    extra.extend_from_slice(&hvcc);
    extra.extend_from_slice(&colr);
    extra.extend_from_slice(&mdcv);
    extra.extend_from_slice(&clli);

    // Apple QuickTime / iOS strictly require `hvc1` (in-band parameter sets in `hvcC`).
    let entry = build_visual_sample_entry(b"hvc1", width, height, &extra);
    let mut stsd_body = Vec::with_capacity(4 + entry.len());
    stsd_body.extend_from_slice(&1u32.to_be_bytes());
    stsd_body.extend_from_slice(&entry);
    make_full_box(b"stsd", 0, 0, &stsd_body)
}

fn build_stts(durations: &[u32]) -> Vec<u8> {
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for &d in durations {
        if let Some(last) = runs.last_mut()
            && last.1 == d
        {
            last.0 += 1;
            continue;
        }
        runs.push((1, d));
    }
    let mut body = Vec::with_capacity(4 + runs.len() * 8);
    body.extend_from_slice(&(runs.len() as u32).to_be_bytes());
    for (count, delta) in runs {
        body.extend_from_slice(&count.to_be_bytes());
        body.extend_from_slice(&delta.to_be_bytes());
    }
    make_full_box(b"stts", 0, 0, &body)
}

fn build_stss(sync_samples: &[u32]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + sync_samples.len() * 4);
    body.extend_from_slice(&(sync_samples.len() as u32).to_be_bytes());
    for &idx in sync_samples {
        body.extend_from_slice(&idx.to_be_bytes());
    }
    make_full_box(b"stss", 0, 0, &body)
}

fn build_stsc_one_sample_per_chunk() -> Vec<u8> {
    let mut body = Vec::with_capacity(16);
    body.extend_from_slice(&1u32.to_be_bytes()); // entry_count = 1
    body.extend_from_slice(&1u32.to_be_bytes()); // first_chunk = 1
    body.extend_from_slice(&1u32.to_be_bytes()); // samples_per_chunk = 1
    body.extend_from_slice(&1u32.to_be_bytes()); // sample_description_index = 1
    make_full_box(b"stsc", 0, 0, &body)
}

fn build_stsz(sizes: &[u32]) -> Vec<u8> {
    let mut body = Vec::with_capacity(8 + sizes.len() * 4);
    body.extend_from_slice(&0u32.to_be_bytes()); // variable sample sizes
    body.extend_from_slice(&(sizes.len() as u32).to_be_bytes());
    for &s in sizes {
        body.extend_from_slice(&s.to_be_bytes());
    }
    make_full_box(b"stsz", 0, 0, &body)
}

fn build_co64(offsets: &[u64]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + offsets.len() * 8);
    body.extend_from_slice(&(offsets.len() as u32).to_be_bytes());
    for &off in offsets {
        body.extend_from_slice(&off.to_be_bytes());
    }
    make_full_box(b"co64", 0, 0, &body)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::AtomicU32;

    /// SPS (High, 4:2:0 8-bit) + PPS + IDR in one access unit.
    const H264_IDR_AU: [u8; 25] = [
        0, 0, 0, 1, 0x67, 0x64, 0x00, 0x2A, 0xAC, // SPS
        0, 0, 0, 1, 0x68, 0xEE, 0x3C, 0x80, // PPS
        0, 0, 0, 1, 0x65, 0x88, 0x84, 0x01, // IDR
    ];
    const H264_P_AU: [u8; 8] = [0, 0, 0, 1, 0x41, 0x9A, 0x22, 0x33];
    const H264_PARAMS_ONLY_AU: [u8; 17] = [
        0, 0, 0, 1, 0x67, 0x64, 0x00, 0x2A, 0xAC, // SPS
        0, 0, 0, 1, 0x68, 0xEE, 0x3C, 0x80, // PPS
    ];

    /// Unique per process and per call, so parallel `cargo test` runs never share a file.
    fn unique_mp4_path(tag: &str) -> PathBuf {
        static SEQ: AtomicU32 = AtomicU32::new(0);
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "nv_demo_mp4_{tag}_{}_{seq}.mp4",
            std::process::id()
        ))
    }

    fn be32(buf: &[u8], at: usize) -> u32 {
        u32::from_be_bytes(buf[at..at + 4].try_into().unwrap())
    }

    fn be64(buf: &[u8], at: usize) -> u64 {
        u64::from_be_bytes(buf[at..at + 8].try_into().unwrap())
    }

    #[derive(Clone, Copy, Debug)]
    struct BoxRef {
        fourcc: [u8; 4],
        /// First payload byte (after the size / type / largesize header).
        body: usize,
        end: usize,
    }

    /// Walks sibling boxes in `[start, end)` and asserts they tile the range exactly.
    fn walk(buf: &[u8], start: usize, end: usize) -> Vec<BoxRef> {
        let mut out = Vec::new();
        let mut pos = start;
        while pos < end {
            let size32 = be32(buf, pos) as usize;
            let fourcc: [u8; 4] = buf[pos + 4..pos + 8].try_into().unwrap();
            let (size, body) = if size32 == 1 {
                (be64(buf, pos + 8) as usize, pos + 16)
            } else {
                (size32, pos + 8)
            };
            assert!(size >= body - pos, "box {fourcc:?} size {size} smaller than header");
            let box_end = pos + size;
            assert!(box_end <= end, "box {fourcc:?} overruns its parent ({box_end} > {end})");
            out.push(BoxRef {
                fourcc,
                body,
                end: box_end,
            });
            pos = box_end;
        }
        assert_eq!(pos, end, "boxes must tile their parent exactly");
        out
    }

    fn child(buf: &[u8], parent: BoxRef, fourcc: &[u8; 4]) -> BoxRef {
        find(&walk(buf, parent.body, parent.end), fourcc)
    }

    fn find(boxes: &[BoxRef], fourcc: &[u8; 4]) -> BoxRef {
        *boxes
            .iter()
            .find(|b| &b.fourcc == fourcc)
            .unwrap_or_else(|| panic!("missing box {}", String::from_utf8_lossy(fourcc)))
    }

    /// Returns the `stbl` box of the single video track.
    fn stbl(buf: &[u8]) -> BoxRef {
        let top = walk(buf, 0, buf.len());
        let moov = find(&top, b"moov");
        let trak = child(buf, moov, b"trak");
        let mdia = child(buf, trak, b"mdia");
        let minf = child(buf, mdia, b"minf");
        child(buf, minf, b"stbl")
    }

    /// Children of the single visual sample entry inside `stsd` (78-byte entry header skipped).
    fn sample_entry_children(buf: &[u8], stbl: BoxRef, fourcc: &[u8; 4]) -> Vec<BoxRef> {
        let stsd = child(buf, stbl, b"stsd");
        assert_eq!(be32(buf, stsd.body + 4), 1, "stsd entry_count");
        let entries = walk(buf, stsd.body + 8, stsd.end);
        let entry = find(&entries, fourcc);
        walk(buf, entry.body + 78, entry.end)
    }

    /// Asserts top-level layout, `mdat` patch, `stsz`/`co64` consistency with the bytes written,
    /// and returns `(stts runs, mvhd duration)`.
    fn check_layout(buf: &[u8], expected_samples: &[&[u8]]) -> (Vec<(u32, u32)>, u64) {
        let top = walk(buf, 0, buf.len());
        let order: Vec<[u8; 4]> = top.iter().map(|b| b.fourcc).collect();
        assert_eq!(order, [*b"ftyp", *b"mdat", *b"moov"]);
        let mdat = top[1];
        assert_eq!(be32(buf, mdat.body - 16), 1, "mdat must use the 64-bit largesize form");
        let payload_len: usize = expected_samples.iter().map(|s| s.len()).sum();
        assert_eq!(mdat.end - mdat.body, payload_len, "patched mdat largesize");

        let stbl = stbl(buf);
        let stsz = child(buf, stbl, b"stsz");
        assert_eq!(be32(buf, stsz.body + 4), 0, "variable sample sizes");
        let n = be32(buf, stsz.body + 8) as usize;
        assert_eq!(n, expected_samples.len());
        let co64 = child(buf, stbl, b"co64");
        assert_eq!(be32(buf, co64.body + 4) as usize, n);
        let stsc = child(buf, stbl, b"stsc");
        assert_eq!(be32(buf, stsc.body + 4), 1);
        assert_eq!(be32(buf, stsc.body + 12), 1, "one sample per chunk");

        let mut expected_off = mdat.body;
        for (i, sample) in expected_samples.iter().enumerate() {
            let size = be32(buf, stsz.body + 12 + 4 * i) as usize;
            let off = be64(buf, co64.body + 8 + 8 * i) as usize;
            assert_eq!(size, sample.len(), "stsz[{i}]");
            assert_eq!(off, expected_off, "co64[{i}]");
            assert_eq!(&buf[off..off + size], *sample, "sample {i} bytes");
            expected_off += size;
        }
        assert_eq!(expected_off, mdat.end, "last sample must end at the end of mdat");

        let stts = child(buf, stbl, b"stts");
        let runs: Vec<(u32, u32)> = (0..be32(buf, stts.body + 4) as usize)
            .map(|i| {
                (
                    be32(buf, stts.body + 8 + 8 * i),
                    be32(buf, stts.body + 12 + 8 * i),
                )
            })
            .collect();
        let moov = find(&top, b"moov");
        let mvhd = child(buf, moov, b"mvhd");
        assert_eq!(buf[mvhd.body], 0, "mvhd version 0 for short recordings");
        (runs, u64::from(be32(buf, mvhd.body + 16)))
    }

    #[test]
    fn annexb_nals_handles_3_and_4_byte_start_codes_without_allocating() {
        let stream = [
            0xFF, // leading garbage before the first start code
            0, 0, 0, 1, 0x67, 0x64, 0x00, 0x2A, // SPS
            0, 0, 1, 0x68, 0xEE, 0x3C, // PPS
            0, 0, 1, 0, 0, 0, 1, // empty NAL between two start codes
            0x65, 0xAA, 0xBB, // IDR
        ];
        let nals: Vec<&[u8]> = annexb_nals(&stream).collect();
        let expected: [&[u8]; 3] = [
            &[0x67, 0x64, 0x00, 0x2A],
            &[0x68, 0xEE, 0x3C],
            &[0x65, 0xAA, 0xBB],
        ];
        assert_eq!(nals, expected);
        assert_eq!(annexb_nals(&[]).count(), 0);
        assert_eq!(annexb_nals(&[0, 0, 1]).count(), 0);
    }

    #[test]
    fn muxer_writes_consistent_h264_sdr_layout_with_vfr_stts() {
        let path = unique_mp4_path("sdr");
        let mut mux = Mp4Muxer::create(&path, 896, 512, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_IDR_AU, 1500).unwrap();
        mux.write_annexb_frame(&H264_P_AU, 6000).unwrap(); // 15 fps dip
        assert_eq!(mux.finish().unwrap(), 2);

        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        let idr = [0, 0, 0, 4, 0x65, 0x88, 0x84, 0x01];
        let p = [0, 0, 0, 4, 0x41, 0x9A, 0x22, 0x33];
        let (runs, duration) = check_layout(&bytes, &[&idr, &p]);
        assert_eq!(runs, [(1, 1500), (1, 6000)]);
        assert_eq!(duration, 7500);

        let stbl = stbl(&bytes);
        let stss = child(&bytes, stbl, b"stss");
        assert_eq!(be32(&bytes, stss.body + 4), 1);
        assert_eq!(be32(&bytes, stss.body + 8), 1, "the IDR is sample 1");

        let entry = sample_entry_children(&bytes, stbl, b"avc1");
        let avcc = find(&entry, b"avcC");
        find(&entry, b"colr");
        let avcc_bytes = &bytes[avcc.body..avcc.end];
        assert_eq!(&avcc_bytes[..4], &[1, 0x64, 0x00, 0x2A]);
        // High profile trailer: chroma_format_idc = 1, 8-bit luma / chroma, no SPS extensions.
        assert_eq!(&avcc_bytes[avcc_bytes.len() - 4..], &[0xFD, 0xF8, 0xF8, 0x00]);
    }

    #[test]
    fn avcc_omits_trailer_for_main_profile() {
        let sps = [0x67, 0x4D, 0x00, 0x1F, 0xAC];
        let stsd = build_stsd_avc1(64, 64, &sps, &[0x68, 0xEE]).unwrap();
        let entry = walk(&stsd, 16, stsd.len());
        let children = walk(&stsd, entry[0].body + 78, entry[0].end);
        let avcc = find(&children, b"avcC");
        // version, profile, compat, level, 0xFF, 0xE1, len(2), SPS(5), 1, len(2), PPS(2)
        assert_eq!(avcc.end - avcc.body, 6 + 2 + 5 + 1 + 2 + 2);
    }

    #[test]
    fn avcc_trailer_parses_high10_bit_depths() {
        // High 10 (110): sps_id 0 '1', chroma 1 '010', luma_m8 2 '011', chroma_m8 2 '011'.
        let sps = [0x67, 110, 0x00, 0x2A, 0b1010_0110, 0b1100_0000];
        assert_eq!(parse_avc_chroma_depth(&sps).unwrap(), (1, 2, 2));
        assert!(parse_avc_chroma_depth(&[0x67, 100, 0x00, 0x2A]).is_err());
        assert!(build_stsd_avc1(64, 64, &[0x67, 100], &[0x68]).is_err());
    }

    #[test]
    fn muxer_writes_hevc_pq10_with_hvc1_and_hdr10_boxes() {
        let path = unique_mp4_path("pq10");
        let mut mux = Mp4Muxer::create(&path, 1280, 720, VideoProfile::Hdr10Pq).unwrap();
        // Minimal HEVC VPS (32 -> 0x40), SPS (33 -> 0x42), PPS (34 -> 0x44), IDR_W_RADL (19 -> 0x26)
        let frame0 = [
            0, 0, 0, 1, 0x40, 0x01, 0x0C, 0x01, // VPS
            0, 0, 0, 1, 0x42, 0x01, 0x01, 0x02, 0x20, 0x00, 0x00, 0x00, 0x90, 0, 0, 0, 0, 0,
            0x99, // SPS (Main10, Level 5.1)
            0, 0, 0, 1, 0x44, 0x01, 0xC0, 0xF3, // PPS
            0, 0, 0, 1, 0x26, 0x01, 0xAF, 0x01, // IDR
        ];
        mux.write_annexb_frame(&frame0, 1500).unwrap();
        assert_eq!(mux.finish().unwrap(), 1);

        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        let idr = [0, 0, 0, 4, 0x26, 0x01, 0xAF, 0x01];
        let (runs, duration) = check_layout(&bytes, &[&idr]);
        assert_eq!(runs, [(1, 1500)]);
        assert_eq!(duration, 1500);

        let entry = sample_entry_children(&bytes, stbl(&bytes), b"hvc1");
        find(&entry, b"hvcC");
        find(&entry, b"colr");
        let mdcv = find(&entry, b"mdcv");
        assert_eq!(mdcv.end - mdcv.body, 24);
        assert_eq!(be32(&bytes, mdcv.body + 16), MDCV_MAX_LUMINANCE);
        assert_eq!(be32(&bytes, mdcv.body + 20), MDCV_MIN_LUMINANCE);
        let clli = find(&entry, b"clli");
        assert_eq!(
            &bytes[clli.body..clli.end],
            [CLLI_MAX_CLL.to_be_bytes(), CLLI_MAX_FALL.to_be_bytes()].concat()
        );
    }

    #[test]
    fn nothing_appears_at_final_path_before_finish_or_after_drop() {
        let path = unique_mp4_path("drop");
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_IDR_AU, 1500).unwrap();
        let tmp = mux.partial.tmp.clone();
        assert_eq!(tmp.parent(), path.parent(), "partial file must share the directory");
        assert!(tmp.exists());
        assert!(!path.exists(), "final path must not exist while recording");
        drop(mux);
        assert!(!path.exists());
        assert!(!tmp.exists(), "partial file must be removed on drop");
    }

    #[test]
    fn failed_finish_removes_partial_and_leaves_no_file() {
        let path = unique_mp4_path("nosps");
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_P_AU, 1500).unwrap(); // slice without SPS / PPS
        let tmp = mux.partial.tmp.clone();
        let err = mux.finish().unwrap_err();
        assert!(err.contains("SPS"), "{err}");
        assert!(!path.exists());
        assert!(!tmp.exists());
    }

    #[test]
    fn finish_with_zero_samples_is_an_error_and_leaves_no_file() {
        let path = unique_mp4_path("empty");
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_PARAMS_ONLY_AU, 1500).unwrap();
        let tmp = mux.partial.tmp.clone();
        assert_eq!(mux.frame_count(), 0);
        assert_eq!(mux.finish().unwrap_err(), EMPTY_MP4_ERROR);
        assert!(!path.exists());
        assert!(!tmp.exists());
    }

    #[test]
    fn finish_replaces_an_existing_file_at_the_final_path() {
        let path = unique_mp4_path("replace");
        std::fs::write(&path, b"stale").unwrap();
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_IDR_AU, 1500).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"stale", "untouched until finish");
        assert_eq!(mux.finish().unwrap(), 1);
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(&bytes[4..8], b"ftyp");
    }

    #[test]
    fn sampleless_access_units_carry_their_duration() {
        let path = unique_mp4_path("carry");
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_PARAMS_ONLY_AU, 1500).unwrap(); // -> next sample
        mux.write_annexb_frame(&H264_IDR_AU[9..], 1500).unwrap(); // PPS + IDR
        mux.write_annexb_frame(&[], 1500).unwrap(); // empty AU -> next sample
        mux.write_annexb_frame(&H264_P_AU, 1500).unwrap();
        mux.write_annexb_frame(&[], 1500).unwrap(); // trailing empty AU -> last sample
        assert_eq!(mux.finish().unwrap(), 2);

        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        let idr = [0, 0, 0, 4, 0x65, 0x88, 0x84, 0x01];
        let p = [0, 0, 0, 4, 0x41, 0x9A, 0x22, 0x33];
        let (runs, duration) = check_layout(&bytes, &[&idr, &p]);
        assert_eq!(runs, [(1, 3000), (1, 4500)]);
        assert_eq!(duration, 5 * 1500);
    }

    #[test]
    fn long_recordings_use_64_bit_duration_boxes() {
        let path = unique_mp4_path("long");
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_IDR_AU, 1500).unwrap();
        let long = u64::from(u32::MAX) + 10;
        mux.total_duration_ticks = long;
        let moov = mux.build_moov().unwrap();
        drop(mux);
        assert!(!path.exists());

        let moov_box = walk(&moov, 0, moov.len())[0];
        let mvhd = child(&moov, moov_box, b"mvhd");
        assert_eq!(moov[mvhd.body], 1, "mvhd version");
        assert_eq!(mvhd.end - mvhd.body, 112);
        assert_eq!(be64(&moov, mvhd.body + 24), long);
        let trak = child(&moov, moov_box, b"trak");
        let tkhd = child(&moov, trak, b"tkhd");
        assert_eq!(moov[tkhd.body], 1, "tkhd version");
        assert_eq!(tkhd.end - tkhd.body, 96);
        assert_eq!(be64(&moov, tkhd.body + 28), long);
        let mdhd = child(&moov, child(&moov, trak, b"mdia"), b"mdhd");
        assert_eq!(moov[mdhd.body], 1, "mdhd version");
        assert_eq!(mdhd.end - mdhd.body, 36);
        assert_eq!(be64(&moov, mdhd.body + 24), long);
    }

    #[test]
    fn sample_buffer_is_reused_across_frames() {
        let path = unique_mp4_path("reuse");
        let mut mux = Mp4Muxer::create(&path, 64, 64, VideoProfile::SdrH264).unwrap();
        mux.write_annexb_frame(&H264_IDR_AU, 1500).unwrap();
        let ptr = mux.sample_buf.as_ptr();
        let cap = mux.sample_buf.capacity();
        for _ in 0..8 {
            mux.write_annexb_frame(&H264_P_AU, 1500).unwrap();
        }
        assert_eq!(mux.sample_buf.as_ptr(), ptr, "sample buffer must not be reallocated");
        assert_eq!(mux.sample_buf.capacity(), cap);
    }
}
