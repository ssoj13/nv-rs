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
//!
//! **Used by** `gpu_export.rs` (`VideoRecorder`).

use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

/// Standard MPEG video timescale (90 kHz: 60 fps = 1500 ticks, 30 fps = 3000 ticks).
pub const MP4_TIMESCALE: u32 = 90_000;

/// Codec & color profile written to the MP4 container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoProfile {
    /// 8-bit H.264 High (`avc1`) with BT.709 limited-range `colr` signaling.
    SdrH264,
    /// 10-bit HEVC Main10 (`hvc1`) with HDR10 BT.2020 SMPTE ST 2084 PQ (`colr` + `mdcv` + `clli`).
    Hdr10Pq,
}

/// Streaming ISO BMFF (`.mp4`) writer backed by a single output file.
pub struct Mp4Muxer {
    writer: BufWriter<File>,
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
}

impl Mp4Muxer {
    /// Creates `path`, writes the `ftyp` box and a 64-bit `mdat` header, and prepares for frames.
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

        let file = File::create(path)
            .map_err(|e| format!("failed to create MP4 file '{}': {e}", path.display()))?;
        let mut writer = BufWriter::new(file);

        let ftyp = build_ftyp(profile);
        writer
            .write_all(&ftyp)
            .map_err(|e| format!("failed to write MP4 ftyp: {e}"))?;

        let mdat_header_offset = ftyp.len() as u64;
        // 64-bit extended-size mdat header: size=1, type="mdat", largesize=placeholder (patched on finish).
        let mut mdat_hdr = [0u8; 16];
        mdat_hdr[0..4].copy_from_slice(&1u32.to_be_bytes());
        mdat_hdr[4..8].copy_from_slice(b"mdat");
        mdat_hdr[8..16].copy_from_slice(&16u64.to_be_bytes());
        writer
            .write_all(&mdat_hdr)
            .map_err(|e| format!("failed to write MP4 mdat header: {e}"))?;

        Ok(Self {
            writer,
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
        })
    }

    /// Appends one Annex-B encoded access unit with duration `duration_ticks` (in 90 kHz ticks).
    pub fn write_annexb_frame(
        &mut self,
        annexb: &[u8],
        duration_ticks: u32,
    ) -> Result<(), String> {
        let duration = duration_ticks.max(1);
        let nals = split_annexb_nals(annexb);
        if nals.is_empty() {
            return Ok(());
        }

        let mut is_sync = false;
        let mut sample_payload = Vec::with_capacity(annexb.len() + 16);

        for nal in nals {
            if nal.is_empty() {
                continue;
            }
            match self.profile {
                VideoProfile::SdrH264 => {
                    let nal_type = nal[0] & 0x1F;
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
                    let nal_type = (nal[0] >> 1) & 0x3F;
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
            sample_payload.extend_from_slice(&nal_len.to_be_bytes());
            sample_payload.extend_from_slice(nal);
        }

        if sample_payload.is_empty() {
            return Ok(());
        }

        let sample_size = u32::try_from(sample_payload.len())
            .map_err(|_| format!("MP4 sample size {} exceeds u32::MAX", sample_payload.len()))?;
        let offset = self.mdat_header_offset + 16 + self.mdat_bytes_written;
        self.writer
            .write_all(&sample_payload)
            .map_err(|e| format!("failed to write MP4 sample: {e}"))?;
        self.mdat_bytes_written += u64::from(sample_size);

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

    /// Patches the `mdat` size header, writes the `moov` box, and flushes the file.
    pub fn finish(mut self) -> Result<u64, String> {
        let count = self.sample_sizes.len() as u64;
        if count == 0 {
            self.writer
                .flush()
                .map_err(|e| format!("failed to flush empty MP4: {e}"))?;
            return Ok(0);
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
        self.writer
            .flush()
            .map_err(|e| format!("failed to flush finalized MP4: {e}"))?;
        Ok(count)
    }

    fn build_moov(&self) -> Result<Vec<u8>, String> {
        let dur_u32 = u32::try_from(self.total_duration_ticks).unwrap_or(u32::MAX);
        let mvhd = build_mvhd(MP4_TIMESCALE, dur_u32);
        let tkhd = build_tkhd(dur_u32, self.width, self.height);
        let mdhd = build_mdhd(MP4_TIMESCALE, dur_u32);
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
                build_stsd_avc1(self.width, self.height, sps, pps)
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

/// Splits an Annex-B byte stream into raw NAL unit slices (excluding start codes).
pub fn split_annexb_nals(data: &[u8]) -> Vec<&[u8]> {
    let mut starts = Vec::new();
    let mut i = 0usize;
    while i + 2 < data.len() {
        if data[i] == 0 && data[i + 1] == 0 {
            if data[i + 2] == 1 {
                starts.push((i, i + 3));
                i += 3;
                continue;
            }
            if i + 3 < data.len() && data[i + 2] == 0 && data[i + 3] == 1 {
                starts.push((i, i + 4));
                i += 4;
                continue;
            }
        }
        i += 1;
    }

    let mut out = Vec::with_capacity(starts.len());
    for idx in 0..starts.len() {
        let payload_start = starts[idx].1;
        let payload_end = if idx + 1 < starts.len() {
            starts[idx + 1].0
        } else {
            data.len()
        };
        if payload_start < payload_end {
            out.push(&data[payload_start..payload_end]);
        }
    }
    out
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

fn build_mvhd(timescale: u32, duration: u32) -> Vec<u8> {
    let mut body = Vec::with_capacity(96);
    body.extend_from_slice(&0u32.to_be_bytes()); // creation_time
    body.extend_from_slice(&0u32.to_be_bytes()); // modification_time
    body.extend_from_slice(&timescale.to_be_bytes());
    body.extend_from_slice(&duration.to_be_bytes());
    body.extend_from_slice(&0x0001_0000u32.to_be_bytes()); // rate 1.0
    body.extend_from_slice(&0x0100u16.to_be_bytes()); // volume 1.0
    body.extend_from_slice(&[0u8; 10]); // reserved
    // Unity 3x3 matrix
    for v in [0x0001_0000u32, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000] {
        body.extend_from_slice(&v.to_be_bytes());
    }
    body.extend_from_slice(&[0u8; 24]); // pre_defined
    body.extend_from_slice(&2u32.to_be_bytes()); // next_track_ID
    make_full_box(b"mvhd", 0, 0, &body)
}

fn build_tkhd(duration: u32, width: u16, height: u16) -> Vec<u8> {
    let mut body = Vec::with_capacity(80);
    body.extend_from_slice(&0u32.to_be_bytes()); // creation_time
    body.extend_from_slice(&0u32.to_be_bytes()); // modification_time
    body.extend_from_slice(&1u32.to_be_bytes()); // track_ID = 1
    body.extend_from_slice(&0u32.to_be_bytes()); // reserved
    body.extend_from_slice(&duration.to_be_bytes());
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
    make_full_box(b"tkhd", 0, 0x0000_0003, &body)
}

fn build_mdhd(timescale: u32, duration: u32) -> Vec<u8> {
    let mut body = Vec::with_capacity(20);
    body.extend_from_slice(&0u32.to_be_bytes()); // creation_time
    body.extend_from_slice(&0u32.to_be_bytes()); // modification_time
    body.extend_from_slice(&timescale.to_be_bytes());
    body.extend_from_slice(&duration.to_be_bytes());
    body.extend_from_slice(&0x55C4u16.to_be_bytes()); // language = "und"
    body.extend_from_slice(&0u16.to_be_bytes()); // pre_defined
    make_full_box(b"mdhd", 0, 0, &body)
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

/// Mastering Display Color Volume (`mdcv`) for BT.2020 1000-nit mastering display (0.0001 cd/m^2 min).
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
    body.extend_from_slice(&10_000_000u32.to_be_bytes()); // 1000.0 cd/m^2 (0.0001 units)
    body.extend_from_slice(&50u32.to_be_bytes()); // 0.005 cd/m^2
    make_box(b"mdcv", &body)
}

/// Content Light Level Information (`clli`): MaxCLL = 1000 nits, MaxFALL = 400 nits.
fn build_clli(max_cll: u16, max_fall: u16) -> Vec<u8> {
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&max_cll.to_be_bytes());
    body.extend_from_slice(&max_fall.to_be_bytes());
    make_box(b"clli", &body)
}

fn build_stsd_avc1(width: u16, height: u16, sps: &[u8], pps: &[u8]) -> Vec<u8> {
    let profile_idc = sps.get(1).copied().unwrap_or(100); // High
    let profile_compat = sps.get(2).copied().unwrap_or(0);
    let level_idc = sps.get(3).copied().unwrap_or(42);

    let mut avcc_body = Vec::with_capacity(11 + sps.len() + pps.len());
    avcc_body.push(1); // configurationVersion
    avcc_body.push(profile_idc);
    avcc_body.push(profile_compat);
    avcc_body.push(level_idc);
    avcc_body.push(0xFF); // 6 bits reserved (1) + lengthSizeMinusOne (3 => 4 bytes)
    avcc_body.push(0xE1); // 3 bits reserved (1) + numOfSequenceParameterSets (1)
    avcc_body.extend_from_slice(&(sps.len() as u16).to_be_bytes());
    avcc_body.extend_from_slice(sps);
    avcc_body.push(1); // numOfPictureParameterSets
    avcc_body.extend_from_slice(&(pps.len() as u16).to_be_bytes());
    avcc_body.extend_from_slice(pps);
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
    make_full_box(b"stsd", 0, 0, &stsd_body)
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
    let clli = build_clli(1000, 400);

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

    #[test]
    fn split_annexb_handles_3_and_4_byte_start_codes() {
        let stream = [
            0, 0, 0, 1, 0x67, 0x64, 0x00, 0x2A, // SPS
            0, 0, 1, 0x68, 0xEE, 0x3C, // PPS
            0, 0, 0, 1, 0x65, 0xAA, 0xBB, // IDR
        ];
        let nals = split_annexb_nals(&stream);
        assert_eq!(nals.len(), 3);
        assert_eq!(nals[0], &[0x67, 0x64, 0x00, 0x2A]);
        assert_eq!(nals[1], &[0x68, 0xEE, 0x3C]);
        assert_eq!(nals[2], &[0x65, 0xAA, 0xBB]);
    }

    #[test]
    fn muxer_writes_valid_h264_sdr_mp4_with_vfr_stts() {
        let path = std::env::temp_dir().join("nv_demo_test_sdr.mp4");
        let mut mux = Mp4Muxer::create(&path, 896, 512, VideoProfile::SdrH264).unwrap();
        let frame0 = [
            0, 0, 0, 1, 0x67, 0x64, 0x00, 0x2A, 0xAC, // SPS
            0, 0, 0, 1, 0x68, 0xEE, 0x3C, 0x80, // PPS
            0, 0, 0, 1, 0x65, 0x88, 0x84, 0x01, // IDR
        ];
        let frame1 = [0, 0, 0, 1, 0x41, 0x9A, 0x22, 0x33]; // P-slice
        mux.write_annexb_frame(&frame0, 1500).unwrap();
        mux.write_annexb_frame(&frame1, 6000).unwrap(); // 15 fps dip
        assert_eq!(mux.finish().unwrap(), 2);

        let bytes = std::fs::read(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(&bytes[4..8], b"ftyp");
        assert!(bytes.windows(4).any(|w| w == b"mdat"));
        assert!(bytes.windows(4).any(|w| w == b"moov"));
        assert!(bytes.windows(4).any(|w| w == b"avc1"));
        assert!(bytes.windows(4).any(|w| w == b"avcC"));
        assert!(bytes.windows(4).any(|w| w == b"colr"));
        assert!(bytes.windows(4).any(|w| w == b"stts"));
        assert!(bytes.windows(4).any(|w| w == b"stss"));
    }

    #[test]
    fn muxer_writes_valid_hevc_pq10_mp4_with_hvc1_and_hdr10_boxes() {
        let path = std::env::temp_dir().join("nv_demo_test_pq10.mp4");
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
        let _ = std::fs::remove_file(&path);
        assert_eq!(&bytes[4..8], b"ftyp");
        assert!(bytes.windows(4).any(|w| w == b"hvc1"));
        assert!(bytes.windows(4).any(|w| w == b"hvcC"));
        assert!(bytes.windows(4).any(|w| w == b"colr"));
        assert!(bytes.windows(4).any(|w| w == b"mdcv"));
        assert!(bytes.windows(4).any(|w| w == b"clli"));
    }
}
