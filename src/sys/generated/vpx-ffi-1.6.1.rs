/* automatically generated by rust-bindgen */

pub const VPX_IMAGE_ABI_VERSION: u32 = 4;
pub const VPX_IMG_FMT_PLANAR: u32 = 256;
pub const VPX_IMG_FMT_UV_FLIP: u32 = 512;
pub const VPX_IMG_FMT_HAS_ALPHA: u32 = 1024;
pub const VPX_IMG_FMT_HIGHBITDEPTH: u32 = 2048;
pub const VPX_PLANE_PACKED: u32 = 0;
pub const VPX_PLANE_Y: u32 = 0;
pub const VPX_PLANE_U: u32 = 1;
pub const VPX_PLANE_V: u32 = 2;
pub const VPX_PLANE_ALPHA: u32 = 3;
pub const VPX_CODEC_ABI_VERSION: u32 = 7;
pub const VPX_CODEC_CAP_DECODER: u32 = 1;
pub const VPX_CODEC_CAP_ENCODER: u32 = 2;
pub const VPX_TS_MAX_PERIODICITY: u32 = 16;
pub const VPX_TS_MAX_LAYERS: u32 = 5;
pub const VPX_MAX_LAYERS: u32 = 12;
pub const VPX_SS_MAX_LAYERS: u32 = 5;
pub const VPX_SS_DEFAULT_LAYERS: u32 = 1;
pub const VPX_ENCODER_ABI_VERSION: u32 = 12;
pub const VPX_CODEC_CAP_PSNR: u32 = 65536;
pub const VPX_CODEC_CAP_OUTPUT_PARTITION: u32 = 131072;
pub const VPX_CODEC_CAP_HIGHBITDEPTH: u32 = 262144;
pub const VPX_CODEC_USE_PSNR: u32 = 65536;
pub const VPX_CODEC_USE_OUTPUT_PARTITION: u32 = 131072;
pub const VPX_CODEC_USE_HIGHBITDEPTH: u32 = 262144;
pub const VPX_FRAME_IS_KEY: u32 = 1;
pub const VPX_FRAME_IS_DROPPABLE: u32 = 2;
pub const VPX_FRAME_IS_INVISIBLE: u32 = 4;
pub const VPX_FRAME_IS_FRAGMENT: u32 = 8;
pub const VPX_ERROR_RESILIENT_DEFAULT: u32 = 1;
pub const VPX_ERROR_RESILIENT_PARTITIONS: u32 = 2;
pub const VPX_EFLAG_FORCE_KF: u32 = 1;
pub const VPX_DL_REALTIME: u32 = 1;
pub const VPX_DL_GOOD_QUALITY: u32 = 1000000;
pub const VPX_DL_BEST_QUALITY: u32 = 0;
pub const VP8_EFLAG_NO_REF_LAST: u32 = 65536;
pub const VP8_EFLAG_NO_REF_GF: u32 = 131072;
pub const VP8_EFLAG_NO_REF_ARF: u32 = 2097152;
pub const VP8_EFLAG_NO_UPD_LAST: u32 = 262144;
pub const VP8_EFLAG_NO_UPD_GF: u32 = 4194304;
pub const VP8_EFLAG_NO_UPD_ARF: u32 = 8388608;
pub const VP8_EFLAG_FORCE_GF: u32 = 524288;
pub const VP8_EFLAG_FORCE_ARF: u32 = 16777216;
pub const VP8_EFLAG_NO_UPD_ENTROPY: u32 = 1048576;
pub const VPX_MAXIMUM_WORK_BUFFERS: u32 = 8;
pub const VP9_MAXIMUM_REF_BUFFERS: u32 = 8;
pub const VPX_DECODER_ABI_VERSION: u32 = 10;
pub const VPX_CODEC_CAP_PUT_SLICE: u32 = 65536;
pub const VPX_CODEC_CAP_PUT_FRAME: u32 = 131072;
pub const VPX_CODEC_CAP_POSTPROC: u32 = 262144;
pub const VPX_CODEC_CAP_ERROR_CONCEALMENT: u32 = 524288;
pub const VPX_CODEC_CAP_INPUT_FRAGMENTS: u32 = 1048576;
pub const VPX_CODEC_CAP_FRAME_THREADING: u32 = 2097152;
pub const VPX_CODEC_CAP_EXTERNAL_FRAME_BUFFER: u32 = 4194304;
pub const VPX_CODEC_USE_POSTPROC: u32 = 65536;
pub const VPX_CODEC_USE_ERROR_CONCEALMENT: u32 = 131072;
pub const VPX_CODEC_USE_INPUT_FRAGMENTS: u32 = 262144;
pub const VPX_CODEC_USE_FRAME_THREADING: u32 = 524288;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_img_fmt {
    VPX_IMG_FMT_NONE = 0,
    VPX_IMG_FMT_RGB24 = 1,
    VPX_IMG_FMT_RGB32 = 2,
    VPX_IMG_FMT_RGB565 = 3,
    VPX_IMG_FMT_RGB555 = 4,
    VPX_IMG_FMT_UYVY = 5,
    VPX_IMG_FMT_YUY2 = 6,
    VPX_IMG_FMT_YVYU = 7,
    VPX_IMG_FMT_BGR24 = 8,
    VPX_IMG_FMT_RGB32_LE = 9,
    VPX_IMG_FMT_ARGB = 10,
    VPX_IMG_FMT_ARGB_LE = 11,
    VPX_IMG_FMT_RGB565_LE = 12,
    VPX_IMG_FMT_RGB555_LE = 13,
    VPX_IMG_FMT_YV12 = 769,
    VPX_IMG_FMT_I420 = 258,
    VPX_IMG_FMT_VPXYV12 = 771,
    VPX_IMG_FMT_VPXI420 = 260,
    VPX_IMG_FMT_I422 = 261,
    VPX_IMG_FMT_I444 = 262,
    VPX_IMG_FMT_I440 = 263,
    VPX_IMG_FMT_444A = 1286,
    VPX_IMG_FMT_I42016 = 2306,
    VPX_IMG_FMT_I42216 = 2309,
    VPX_IMG_FMT_I44416 = 2310,
    VPX_IMG_FMT_I44016 = 2311,
}
pub use self::vpx_img_fmt as vpx_img_fmt_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_color_space {
    VPX_CS_UNKNOWN = 0,
    VPX_CS_BT_601 = 1,
    VPX_CS_BT_709 = 2,
    VPX_CS_SMPTE_170 = 3,
    VPX_CS_SMPTE_240 = 4,
    VPX_CS_BT_2020 = 5,
    VPX_CS_RESERVED = 6,
    VPX_CS_SRGB = 7,
}
pub use self::vpx_color_space as vpx_color_space_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_color_range {
    VPX_CR_STUDIO_RANGE = 0,
    VPX_CR_FULL_RANGE = 1,
}
pub use self::vpx_color_range as vpx_color_range_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_image {
    pub fmt: vpx_img_fmt_t,
    pub cs: vpx_color_space_t,
    pub range: vpx_color_range_t,
    pub w: ::std::os::raw::c_uint,
    pub h: ::std::os::raw::c_uint,
    pub bit_depth: ::std::os::raw::c_uint,
    pub d_w: ::std::os::raw::c_uint,
    pub d_h: ::std::os::raw::c_uint,
    pub r_w: ::std::os::raw::c_uint,
    pub r_h: ::std::os::raw::c_uint,
    pub x_chroma_shift: ::std::os::raw::c_uint,
    pub y_chroma_shift: ::std::os::raw::c_uint,
    pub planes: [*mut ::std::os::raw::c_uchar; 4usize],
    pub stride: [::std::os::raw::c_int; 4usize],
    pub bps: ::std::os::raw::c_int,
    pub user_priv: *mut ::std::os::raw::c_void,
    pub img_data: *mut ::std::os::raw::c_uchar,
    pub img_data_owner: ::std::os::raw::c_int,
    pub self_allocd: ::std::os::raw::c_int,
    pub fb_priv: *mut ::std::os::raw::c_void,
}
pub type vpx_image_t = vpx_image;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_image_rect {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
    pub w: ::std::os::raw::c_uint,
    pub h: ::std::os::raw::c_uint,
}
pub type vpx_image_rect_t = vpx_image_rect;
extern "C" {
    pub fn vpx_img_alloc(
        img: *mut vpx_image_t,
        fmt: vpx_img_fmt_t,
        d_w: ::std::os::raw::c_uint,
        d_h: ::std::os::raw::c_uint,
        align: ::std::os::raw::c_uint,
    ) -> *mut vpx_image_t;
}
extern "C" {
    pub fn vpx_img_wrap(
        img: *mut vpx_image_t,
        fmt: vpx_img_fmt_t,
        d_w: ::std::os::raw::c_uint,
        d_h: ::std::os::raw::c_uint,
        align: ::std::os::raw::c_uint,
        img_data: *mut ::std::os::raw::c_uchar,
    ) -> *mut vpx_image_t;
}
extern "C" {
    pub fn vpx_img_set_rect(
        img: *mut vpx_image_t,
        x: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
        w: ::std::os::raw::c_uint,
        h: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vpx_img_flip(img: *mut vpx_image_t);
}
extern "C" {
    pub fn vpx_img_free(img: *mut vpx_image_t);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_codec_err_t {
    VPX_CODEC_OK = 0,
    VPX_CODEC_ERROR = 1,
    VPX_CODEC_MEM_ERROR = 2,
    VPX_CODEC_ABI_MISMATCH = 3,
    VPX_CODEC_INCAPABLE = 4,
    VPX_CODEC_UNSUP_BITSTREAM = 5,
    VPX_CODEC_UNSUP_FEATURE = 6,
    VPX_CODEC_CORRUPT_FRAME = 7,
    VPX_CODEC_INVALID_PARAM = 8,
    VPX_CODEC_LIST_END = 9,
}
pub type vpx_codec_caps_t = ::std::os::raw::c_long;
pub type vpx_codec_flags_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_iface {
    _unused: [u8; 0],
}
pub type vpx_codec_iface_t = vpx_codec_iface;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_priv {
    _unused: [u8; 0],
}
pub type vpx_codec_priv_t = vpx_codec_priv;
pub type vpx_codec_iter_t = *const ::std::os::raw::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpx_codec_ctx {
    pub name: *const ::std::os::raw::c_char,
    pub iface: *mut vpx_codec_iface_t,
    pub err: vpx_codec_err_t,
    pub err_detail: *const ::std::os::raw::c_char,
    pub init_flags: vpx_codec_flags_t,
    pub config: vpx_codec_ctx__bindgen_ty_1,
    pub priv_: *mut vpx_codec_priv_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union vpx_codec_ctx__bindgen_ty_1 {
    pub dec: *const vpx_codec_dec_cfg,
    pub enc: *const vpx_codec_enc_cfg,
    pub raw: *const ::std::os::raw::c_void,
    _bindgen_union_align: u64,
}
pub type vpx_codec_ctx_t = vpx_codec_ctx;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_bit_depth {
    VPX_BITS_8 = 8,
    VPX_BITS_10 = 10,
    VPX_BITS_12 = 12,
}
pub use self::vpx_bit_depth as vpx_bit_depth_t;
extern "C" {
    pub fn vpx_codec_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vpx_codec_version_str() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_version_extra_str() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_build_config() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_iface_name(iface: *mut vpx_codec_iface_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_err_to_string(err: vpx_codec_err_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_error(ctx: *mut vpx_codec_ctx_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_error_detail(ctx: *mut vpx_codec_ctx_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_destroy(ctx: *mut vpx_codec_ctx_t) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_caps(iface: *mut vpx_codec_iface_t) -> vpx_codec_caps_t;
}
extern "C" {
    pub fn vpx_codec_control_(
        ctx: *mut vpx_codec_ctx_t,
        ctrl_id: ::std::os::raw::c_int,
        ...
    ) -> vpx_codec_err_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8_com_control_id {
    VP8_SET_REFERENCE = 1,
    VP8_COPY_REFERENCE = 2,
    VP8_SET_POSTPROC = 3,
    VP8_SET_DBG_COLOR_REF_FRAME = 4,
    VP8_SET_DBG_COLOR_MB_MODES = 5,
    VP8_SET_DBG_COLOR_B_MODES = 6,
    VP8_SET_DBG_DISPLAY_MV = 7,
    VP9_GET_REFERENCE = 128,
    VP8_COMMON_CTRL_ID_MAX = 129,
    VP8_DECODER_CTRL_ID_START = 256,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8_postproc_level {
    VP8_NOFILTERING = 0,
    VP8_DEBLOCK = 1,
    VP8_DEMACROBLOCK = 2,
    VP8_ADDNOISE = 4,
    VP8_DEBUG_TXT_FRAME_INFO = 8,
    VP8_DEBUG_TXT_MBLK_MODES = 16,
    VP8_DEBUG_TXT_DC_DIFF = 32,
    VP8_DEBUG_TXT_RATE_INFO = 64,
    VP8_MFQE = 1024,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vp8_postproc_cfg {
    pub post_proc_flag: ::std::os::raw::c_int,
    pub deblocking_level: ::std::os::raw::c_int,
    pub noise_level: ::std::os::raw::c_int,
}
pub type vp8_postproc_cfg_t = vp8_postproc_cfg;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_ref_frame_type {
    VP8_LAST_FRAME = 1,
    VP8_GOLD_FRAME = 2,
    VP8_ALTR_FRAME = 4,
}
pub use self::vpx_ref_frame_type as vpx_ref_frame_type_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_ref_frame {
    pub frame_type: vpx_ref_frame_type_t,
    pub img: vpx_image_t,
}
pub type vpx_ref_frame_t = vpx_ref_frame;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vp9_ref_frame {
    pub idx: ::std::os::raw::c_int,
    pub img: vpx_image_t,
}
pub type vp9_ref_frame_t = vp9_ref_frame;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_fixed_buf {
    pub buf: *mut ::std::os::raw::c_void,
    pub sz: usize,
}
pub type vpx_fixed_buf_t = vpx_fixed_buf;
pub type vpx_codec_pts_t = i64;
pub type vpx_codec_frame_flags_t = u32;
pub type vpx_codec_er_flags_t = u32;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_codec_cx_pkt_kind {
    VPX_CODEC_CX_FRAME_PKT = 0,
    VPX_CODEC_STATS_PKT = 1,
    VPX_CODEC_FPMB_STATS_PKT = 2,
    VPX_CODEC_PSNR_PKT = 3,
    VPX_CODEC_CUSTOM_PKT = 256,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpx_codec_cx_pkt {
    pub kind: vpx_codec_cx_pkt_kind,
    pub data: vpx_codec_cx_pkt__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union vpx_codec_cx_pkt__bindgen_ty_1 {
    pub frame: vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1,
    pub twopass_stats: vpx_fixed_buf_t,
    pub firstpass_mb_stats: vpx_fixed_buf_t,
    pub psnr: vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt,
    pub raw: vpx_fixed_buf_t,
    pub pad: [::std::os::raw::c_char; 124usize],
    _bindgen_union_align: [u64; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 {
    pub buf: *mut ::std::os::raw::c_void,
    pub sz: usize,
    pub pts: vpx_codec_pts_t,
    pub duration: ::std::os::raw::c_ulong,
    pub flags: vpx_codec_frame_flags_t,
    pub partition_id: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt {
    pub samples: [::std::os::raw::c_uint; 4usize],
    pub sse: [u64; 4usize],
    pub psnr: [f64; 4usize],
}
pub type vpx_codec_cx_pkt_t = vpx_codec_cx_pkt;
pub type vpx_codec_enc_output_cx_pkt_cb_fn_t = ::std::option::Option<
    unsafe extern "C" fn(pkt: *mut vpx_codec_cx_pkt_t, user_data: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_enc_output_cx_cb_pair {
    pub output_cx_pkt: vpx_codec_enc_output_cx_pkt_cb_fn_t,
    pub user_priv: *mut ::std::os::raw::c_void,
}
pub type vpx_codec_priv_output_cx_pkt_cb_pair_t = vpx_codec_enc_output_cx_cb_pair;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_rational {
    pub num: ::std::os::raw::c_int,
    pub den: ::std::os::raw::c_int,
}
pub type vpx_rational_t = vpx_rational;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_enc_pass {
    VPX_RC_ONE_PASS = 0,
    VPX_RC_FIRST_PASS = 1,
    VPX_RC_LAST_PASS = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_rc_mode {
    VPX_VBR = 0,
    VPX_CBR = 1,
    VPX_CQ = 2,
    VPX_Q = 3,
}
pub const vpx_kf_mode_VPX_KF_DISABLED: vpx_kf_mode = vpx_kf_mode::VPX_KF_FIXED;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_kf_mode {
    VPX_KF_FIXED = 0,
    VPX_KF_AUTO = 1,
}
pub type vpx_enc_frame_flags_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_enc_cfg {
    pub g_usage: ::std::os::raw::c_uint,
    pub g_threads: ::std::os::raw::c_uint,
    pub g_profile: ::std::os::raw::c_uint,
    pub g_w: ::std::os::raw::c_uint,
    pub g_h: ::std::os::raw::c_uint,
    pub g_bit_depth: vpx_bit_depth_t,
    pub g_input_bit_depth: ::std::os::raw::c_uint,
    pub g_timebase: vpx_rational,
    pub g_error_resilient: vpx_codec_er_flags_t,
    pub g_pass: vpx_enc_pass,
    pub g_lag_in_frames: ::std::os::raw::c_uint,
    pub rc_dropframe_thresh: ::std::os::raw::c_uint,
    pub rc_resize_allowed: ::std::os::raw::c_uint,
    pub rc_scaled_width: ::std::os::raw::c_uint,
    pub rc_scaled_height: ::std::os::raw::c_uint,
    pub rc_resize_up_thresh: ::std::os::raw::c_uint,
    pub rc_resize_down_thresh: ::std::os::raw::c_uint,
    pub rc_end_usage: vpx_rc_mode,
    pub rc_twopass_stats_in: vpx_fixed_buf_t,
    pub rc_firstpass_mb_stats_in: vpx_fixed_buf_t,
    pub rc_target_bitrate: ::std::os::raw::c_uint,
    pub rc_min_quantizer: ::std::os::raw::c_uint,
    pub rc_max_quantizer: ::std::os::raw::c_uint,
    pub rc_undershoot_pct: ::std::os::raw::c_uint,
    pub rc_overshoot_pct: ::std::os::raw::c_uint,
    pub rc_buf_sz: ::std::os::raw::c_uint,
    pub rc_buf_initial_sz: ::std::os::raw::c_uint,
    pub rc_buf_optimal_sz: ::std::os::raw::c_uint,
    pub rc_2pass_vbr_bias_pct: ::std::os::raw::c_uint,
    pub rc_2pass_vbr_minsection_pct: ::std::os::raw::c_uint,
    pub rc_2pass_vbr_maxsection_pct: ::std::os::raw::c_uint,
    pub kf_mode: vpx_kf_mode,
    pub kf_min_dist: ::std::os::raw::c_uint,
    pub kf_max_dist: ::std::os::raw::c_uint,
    pub ss_number_layers: ::std::os::raw::c_uint,
    pub ss_enable_auto_alt_ref: [::std::os::raw::c_int; 5usize],
    pub ss_target_bitrate: [::std::os::raw::c_uint; 5usize],
    pub ts_number_layers: ::std::os::raw::c_uint,
    pub ts_target_bitrate: [::std::os::raw::c_uint; 5usize],
    pub ts_rate_decimator: [::std::os::raw::c_uint; 5usize],
    pub ts_periodicity: ::std::os::raw::c_uint,
    pub ts_layer_id: [::std::os::raw::c_uint; 16usize],
    pub layer_target_bitrate: [::std::os::raw::c_uint; 12usize],
    pub temporal_layering_mode: ::std::os::raw::c_int,
}
pub type vpx_codec_enc_cfg_t = vpx_codec_enc_cfg;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_svc_parameters {
    pub max_quantizers: [::std::os::raw::c_int; 12usize],
    pub min_quantizers: [::std::os::raw::c_int; 12usize],
    pub scaling_factor_num: [::std::os::raw::c_int; 12usize],
    pub scaling_factor_den: [::std::os::raw::c_int; 12usize],
    pub speed_per_layer: [::std::os::raw::c_int; 12usize],
    pub temporal_layering_mode: ::std::os::raw::c_int,
}
pub type vpx_svc_extra_cfg_t = vpx_svc_parameters;
extern "C" {
    pub fn vpx_codec_enc_init_ver(
        ctx: *mut vpx_codec_ctx_t,
        iface: *mut vpx_codec_iface_t,
        cfg: *const vpx_codec_enc_cfg_t,
        flags: vpx_codec_flags_t,
        ver: ::std::os::raw::c_int,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_enc_init_multi_ver(
        ctx: *mut vpx_codec_ctx_t,
        iface: *mut vpx_codec_iface_t,
        cfg: *mut vpx_codec_enc_cfg_t,
        num_enc: ::std::os::raw::c_int,
        flags: vpx_codec_flags_t,
        dsf: *mut vpx_rational_t,
        ver: ::std::os::raw::c_int,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_enc_config_default(
        iface: *mut vpx_codec_iface_t,
        cfg: *mut vpx_codec_enc_cfg_t,
        reserved: ::std::os::raw::c_uint,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_enc_config_set(
        ctx: *mut vpx_codec_ctx_t,
        cfg: *const vpx_codec_enc_cfg_t,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_global_headers(ctx: *mut vpx_codec_ctx_t) -> *mut vpx_fixed_buf_t;
}
extern "C" {
    pub fn vpx_codec_encode(
        ctx: *mut vpx_codec_ctx_t,
        img: *const vpx_image_t,
        pts: vpx_codec_pts_t,
        duration: ::std::os::raw::c_ulong,
        flags: vpx_enc_frame_flags_t,
        deadline: ::std::os::raw::c_ulong,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_set_cx_data_buf(
        ctx: *mut vpx_codec_ctx_t,
        buf: *const vpx_fixed_buf_t,
        pad_before: ::std::os::raw::c_uint,
        pad_after: ::std::os::raw::c_uint,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_cx_data(
        ctx: *mut vpx_codec_ctx_t,
        iter: *mut vpx_codec_iter_t,
    ) -> *const vpx_codec_cx_pkt_t;
}
extern "C" {
    pub fn vpx_codec_get_preview_frame(ctx: *mut vpx_codec_ctx_t) -> *const vpx_image_t;
}
extern "C" {
    pub static mut vpx_codec_vp8_cx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp8_cx() -> *mut vpx_codec_iface_t;
}
extern "C" {
    pub static mut vpx_codec_vp9_cx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp9_cx() -> *mut vpx_codec_iface_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8e_enc_control_id {
    VP8E_SET_ROI_MAP = 8,
    VP8E_SET_ACTIVEMAP = 9,
    VP8E_SET_SCALEMODE = 11,
    VP8E_SET_CPUUSED = 13,
    VP8E_SET_ENABLEAUTOALTREF = 14,
    VP8E_SET_NOISE_SENSITIVITY = 15,
    VP8E_SET_SHARPNESS = 16,
    VP8E_SET_STATIC_THRESHOLD = 17,
    VP8E_SET_TOKEN_PARTITIONS = 18,
    VP8E_GET_LAST_QUANTIZER = 19,
    VP8E_GET_LAST_QUANTIZER_64 = 20,
    VP8E_SET_ARNR_MAXFRAMES = 21,
    VP8E_SET_ARNR_STRENGTH = 22,
    VP8E_SET_ARNR_TYPE = 23,
    VP8E_SET_TUNING = 24,
    VP8E_SET_CQ_LEVEL = 25,
    VP8E_SET_MAX_INTRA_BITRATE_PCT = 26,
    VP8E_SET_FRAME_FLAGS = 27,
    VP9E_SET_MAX_INTER_BITRATE_PCT = 28,
    VP9E_SET_GF_CBR_BOOST_PCT = 29,
    VP8E_SET_TEMPORAL_LAYER_ID = 30,
    VP8E_SET_SCREEN_CONTENT_MODE = 31,
    VP9E_SET_LOSSLESS = 32,
    VP9E_SET_TILE_COLUMNS = 33,
    VP9E_SET_TILE_ROWS = 34,
    VP9E_SET_FRAME_PARALLEL_DECODING = 35,
    VP9E_SET_AQ_MODE = 36,
    VP9E_SET_FRAME_PERIODIC_BOOST = 37,
    VP9E_SET_NOISE_SENSITIVITY = 38,
    VP9E_SET_SVC = 39,
    VP9E_SET_SVC_PARAMETERS = 40,
    VP9E_SET_SVC_LAYER_ID = 41,
    VP9E_SET_TUNE_CONTENT = 42,
    VP9E_GET_SVC_LAYER_ID = 43,
    VP9E_REGISTER_CX_CALLBACK = 44,
    VP9E_SET_COLOR_SPACE = 45,
    VP9E_SET_TEMPORAL_LAYERING_MODE = 46,
    VP9E_SET_MIN_GF_INTERVAL = 47,
    VP9E_SET_MAX_GF_INTERVAL = 48,
    VP9E_GET_ACTIVEMAP = 49,
    VP9E_SET_COLOR_RANGE = 50,
    VP9E_SET_SVC_REF_FRAME_CONFIG = 51,
    VP9E_SET_RENDER_SIZE = 52,
    VP9E_SET_TARGET_LEVEL = 53,
    VP9E_GET_LEVEL = 54,
    VP9E_SET_ALT_REF_AQ = 55,
    VP8E_SET_GF_CBR_BOOST_PCT = 56,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_scaling_mode_1d {
    VP8E_NORMAL = 0,
    VP8E_FOURFIVE = 1,
    VP8E_THREEFIVE = 2,
    VP8E_ONETWO = 3,
}
pub use self::vpx_scaling_mode_1d as VPX_SCALING_MODE;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp9e_temporal_layering_mode {
    VP9E_TEMPORAL_LAYERING_MODE_NOLAYERING = 0,
    VP9E_TEMPORAL_LAYERING_MODE_BYPASS = 1,
    VP9E_TEMPORAL_LAYERING_MODE_0101 = 2,
    VP9E_TEMPORAL_LAYERING_MODE_0212 = 3,
}
pub use self::vp9e_temporal_layering_mode as VP9E_TEMPORAL_LAYERING_MODE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_roi_map {
    pub roi_map: *mut ::std::os::raw::c_uchar,
    pub rows: ::std::os::raw::c_uint,
    pub cols: ::std::os::raw::c_uint,
    pub delta_q: [::std::os::raw::c_int; 4usize],
    pub delta_lf: [::std::os::raw::c_int; 4usize],
    pub static_threshold: [::std::os::raw::c_uint; 4usize],
}
pub type vpx_roi_map_t = vpx_roi_map;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_active_map {
    pub active_map: *mut ::std::os::raw::c_uchar,
    pub rows: ::std::os::raw::c_uint,
    pub cols: ::std::os::raw::c_uint,
}
pub type vpx_active_map_t = vpx_active_map;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_scaling_mode {
    pub h_scaling_mode: VPX_SCALING_MODE,
    pub v_scaling_mode: VPX_SCALING_MODE,
}
pub type vpx_scaling_mode_t = vpx_scaling_mode;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8e_token_partitions {
    VP8_ONE_TOKENPARTITION = 0,
    VP8_TWO_TOKENPARTITION = 1,
    VP8_FOUR_TOKENPARTITION = 2,
    VP8_EIGHT_TOKENPARTITION = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp9e_tune_content {
    VP9E_CONTENT_DEFAULT = 0,
    VP9E_CONTENT_SCREEN = 1,
    VP9E_CONTENT_INVALID = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8e_tuning {
    VP8_TUNE_PSNR = 0,
    VP8_TUNE_SSIM = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_svc_layer_id {
    pub spatial_layer_id: ::std::os::raw::c_int,
    pub temporal_layer_id: ::std::os::raw::c_int,
}
pub type vpx_svc_layer_id_t = vpx_svc_layer_id;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_svc_ref_frame_config {
    pub frame_flags: [::std::os::raw::c_int; 5usize],
    pub lst_fb_idx: [::std::os::raw::c_int; 5usize],
    pub gld_fb_idx: [::std::os::raw::c_int; 5usize],
    pub alt_fb_idx: [::std::os::raw::c_int; 5usize],
}
pub type vpx_svc_ref_frame_config_t = vpx_svc_ref_frame_config;
extern "C" {
    pub static mut vpx_codec_vp8_dx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp8_dx() -> *mut vpx_codec_iface_t;
}
extern "C" {
    pub static mut vpx_codec_vp9_dx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp9_dx() -> *mut vpx_codec_iface_t;
}
pub const vp8_dec_control_id_VP8D_SET_DECRYPTOR: vp8_dec_control_id =
    vp8_dec_control_id::VPXD_SET_DECRYPTOR;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8_dec_control_id {
    VP8D_GET_LAST_REF_UPDATES = 256,
    VP8D_GET_FRAME_CORRUPTED = 257,
    VP8D_GET_LAST_REF_USED = 258,
    VPXD_SET_DECRYPTOR = 259,
    VP9D_GET_FRAME_SIZE = 260,
    VP9D_GET_DISPLAY_SIZE = 261,
    VP9D_GET_BIT_DEPTH = 262,
    VP9_SET_BYTE_ALIGNMENT = 263,
    VP9_INVERT_TILE_DECODE_ORDER = 264,
    VP9_SET_SKIP_LOOP_FILTER = 265,
    VP9_DECODE_SVC_SPATIAL_LAYER = 266,
    VP8_DECODER_CTRL_ID_MAX = 267,
}
pub type vpx_decrypt_cb = ::std::option::Option<
    unsafe extern "C" fn(
        decrypt_state: *mut ::std::os::raw::c_void,
        input: *const ::std::os::raw::c_uchar,
        output: *mut ::std::os::raw::c_uchar,
        count: ::std::os::raw::c_int,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_decrypt_init {
    pub decrypt_cb: vpx_decrypt_cb,
    pub decrypt_state: *mut ::std::os::raw::c_void,
}
pub type vp8_decrypt_init = vpx_decrypt_init;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_frame_buffer {
    pub data: *mut u8,
    pub size: usize,
    pub priv_: *mut ::std::os::raw::c_void,
}
pub type vpx_codec_frame_buffer_t = vpx_codec_frame_buffer;
pub type vpx_get_frame_buffer_cb_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        priv_: *mut ::std::os::raw::c_void,
        min_size: usize,
        fb: *mut vpx_codec_frame_buffer_t,
    ) -> ::std::os::raw::c_int,
>;
pub type vpx_release_frame_buffer_cb_fn_t = ::std::option::Option<
    unsafe extern "C" fn(priv_: *mut ::std::os::raw::c_void, fb: *mut vpx_codec_frame_buffer_t)
        -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_stream_info {
    pub sz: ::std::os::raw::c_uint,
    pub w: ::std::os::raw::c_uint,
    pub h: ::std::os::raw::c_uint,
    pub is_kf: ::std::os::raw::c_uint,
}
pub type vpx_codec_stream_info_t = vpx_codec_stream_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_dec_cfg {
    pub threads: ::std::os::raw::c_uint,
    pub w: ::std::os::raw::c_uint,
    pub h: ::std::os::raw::c_uint,
}
pub type vpx_codec_dec_cfg_t = vpx_codec_dec_cfg;
extern "C" {
    pub fn vpx_codec_dec_init_ver(
        ctx: *mut vpx_codec_ctx_t,
        iface: *mut vpx_codec_iface_t,
        cfg: *const vpx_codec_dec_cfg_t,
        flags: vpx_codec_flags_t,
        ver: ::std::os::raw::c_int,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_peek_stream_info(
        iface: *mut vpx_codec_iface_t,
        data: *const u8,
        data_sz: ::std::os::raw::c_uint,
        si: *mut vpx_codec_stream_info_t,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_stream_info(
        ctx: *mut vpx_codec_ctx_t,
        si: *mut vpx_codec_stream_info_t,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_decode(
        ctx: *mut vpx_codec_ctx_t,
        data: *const u8,
        data_sz: ::std::os::raw::c_uint,
        user_priv: *mut ::std::os::raw::c_void,
        deadline: ::std::os::raw::c_long,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_frame(
        ctx: *mut vpx_codec_ctx_t,
        iter: *mut vpx_codec_iter_t,
    ) -> *mut vpx_image_t;
}
pub type vpx_codec_put_frame_cb_fn_t = ::std::option::Option<
    unsafe extern "C" fn(user_priv: *mut ::std::os::raw::c_void, img: *const vpx_image_t),
>;
extern "C" {
    pub fn vpx_codec_register_put_frame_cb(
        ctx: *mut vpx_codec_ctx_t,
        cb: vpx_codec_put_frame_cb_fn_t,
        user_priv: *mut ::std::os::raw::c_void,
    ) -> vpx_codec_err_t;
}
pub type vpx_codec_put_slice_cb_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        user_priv: *mut ::std::os::raw::c_void,
        img: *const vpx_image_t,
        valid: *const vpx_image_rect_t,
        update: *const vpx_image_rect_t,
    ),
>;
extern "C" {
    pub fn vpx_codec_register_put_slice_cb(
        ctx: *mut vpx_codec_ctx_t,
        cb: vpx_codec_put_slice_cb_fn_t,
        user_priv: *mut ::std::os::raw::c_void,
    ) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_set_frame_buffer_functions(
        ctx: *mut vpx_codec_ctx_t,
        cb_get: vpx_get_frame_buffer_cb_fn_t,
        cb_release: vpx_release_frame_buffer_cb_fn_t,
        cb_priv: *mut ::std::os::raw::c_void,
    ) -> vpx_codec_err_t;
}
