#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_dec_API::{SkpSilkDecoderStruct, SKP_Silk_decoder_control}, skp_silk_nlsf2a_stable::skp_silk_nlsf2a_stable, skp_silk_lpc_synthesis_order16::skp_silk_lpc_synthesis_order16, skp_silk_lpc_synthesis_filter::skp_silk_lpc_synthesis_filter, skp_rand, skp_sat_16, skp_r_shift_round, skp_s_mul_w_w};
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SKP_Silk_resampler_state_struct {
    pub sIIR: [libc::c_int; 6],
    pub sFIR: [libc::c_int; 16],
    pub sDown2: [libc::c_int; 2],
    pub resampler_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up2_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSize: libc::c_int,
    pub invRatio_Q16: libc::c_int,
    pub FIR_Fracs: libc::c_int,
    pub input2x: libc::c_int,
    pub Coefs: *const libc::c_short,
    pub sDownPre: [libc::c_int; 2],
    pub sUpPost: [libc::c_int; 2],
    pub down_pre_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub up_post_function: Option::<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_short,
            *const libc::c_short,
            libc::c_int,
        ) -> (),
    >,
    pub batchSizePrePost: libc::c_int,
    pub ratio_Q16: libc::c_int,
    pub nPreDownsamplers: libc::c_int,
    pub nPostUpsamplers: libc::c_int,
    pub magic_number: libc::c_int,
}
pub type SKP_Silk_resampler_state_struct = _SKP_Silk_resampler_state_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_range_coder_state {
    pub bufferLength: libc::c_int,
    pub bufferIx: libc::c_int,
    pub base_Q32: libc::c_uint,
    pub range_Q16: libc::c_uint,
    pub error: libc::c_int,
    pub buffer: [libc::c_uchar; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkpSilkNlsfCbStruct {
    pub nStages: libc::c_int,
    pub CBStages: *const SKP_Silk_NLSF_CBS,
    pub NDeltaMin_Q15: *const libc::c_int,
    pub CDF: *const libc::c_ushort,
    pub StartPtr: *const *const libc::c_ushort,
    pub MiddleIx: *const libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_PLC_struct {
    pub pitchL_Q8: libc::c_int,
    pub LTPCoef_Q14: [libc::c_short; 5],
    pub prevLPC_Q12: [libc::c_short; 16],
    pub last_frame_lost: libc::c_int,
    pub rand_seed: libc::c_int,
    pub randScale_Q14: libc::c_short,
    pub conc_energy: libc::c_int,
    pub conc_energy_shift: libc::c_int,
    pub prevLTP_scale_Q14: libc::c_short,
    pub prevGain_Q16: [libc::c_int; 4],
    pub fs_k_hz: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_CNG_struct {
    pub CNG_exc_buf_Q10: [libc::c_int; 480],
    pub CNG_smth_NLSF_Q15: [libc::c_int; 16],
    pub CNG_synth_state: [libc::c_int; 16],
    pub CNG_smth_Gain_Q16: libc::c_int,
    pub rand_seed: libc::c_int,
    pub fs_k_hz: libc::c_int,
}

impl Default for SKP_Silk_CNG_struct {
    fn default() -> Self {
        Self { CNG_exc_buf_Q10: [0;480], CNG_smth_NLSF_Q15: Default::default(), CNG_synth_state: Default::default(), CNG_smth_Gain_Q16: Default::default(), rand_seed: Default::default(), fs_k_hz: Default::default() }
    }
}

fn skp_silk_cng_exc(
    residual: &mut [i16],
    exc_buf_q10: &[i32],
    gain_q16: i32,
    length: i32,
    rand_seed: &mut i32,
) {
    let mut exc_mask = 255;
    while exc_mask > length {
        exc_mask = exc_mask >> 1;
    }
    let mut seed = *rand_seed;
    for i in 0..length as usize {
        seed = skp_rand!( seed );
        let idx = (seed >> 24 & exc_mask) as usize;
        residual[i] = skp_sat_16!(skp_r_shift_round!(skp_s_mul_w_w!(exc_buf_q10[idx], gain_q16 ), 10 ),i32) as i16;
    }
    *rand_seed = seed;
}

pub fn skp_silk_cng_reset(ps_dec: &mut SkpSilkDecoderStruct) {
    let nlsf_step_q15 = 0x7fff / (ps_dec.LPC_order + 1);
    let mut nlsf_acc_q15 = 0;
    for i in 0..ps_dec.LPC_order as usize {
        nlsf_acc_q15 += nlsf_step_q15;
        ps_dec.sCNG.CNG_smth_NLSF_Q15[i] = nlsf_acc_q15;
    }
    ps_dec.sCNG.CNG_smth_Gain_Q16 = 0;
    ps_dec.sCNG.rand_seed = 3176576;
}

#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_CNG(
    mut psDec: &mut SkpSilkDecoderStruct,
    mut psDecCtrl: *mut SKP_Silk_decoder_control,
    mut signal: *mut libc::c_short,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut subfr: libc::c_int = 0;
    let mut tmp_32: libc::c_int = 0;
    let mut Gain_Q26: libc::c_int = 0;
    let mut max_Gain_Q16: libc::c_int = 0;
    let mut LPC_buf: [libc::c_short; 16] = [0; 16];
    let mut CNG_sig: [libc::c_short; 480] = [0; 480];
    if (*psDec).fs_k_hz != psDec.sCNG.fs_k_hz {
        skp_silk_cng_reset(psDec);
        psDec.sCNG.fs_k_hz = psDec.fs_k_hz;
    }
    let psCNG = &mut (*psDec).sCNG;
    if (*psDec).lossCnt == 0 as libc::c_int && (*psDec).vadFlag == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*psDec).LPC_order {
            (*psCNG).CNG_smth_NLSF_Q15[i as usize]
                += ((*psDec).prevNLSF_Q15[i as usize]
                    - (*psCNG).CNG_smth_NLSF_Q15[i as usize] >> 16 as libc::c_int)
                    * 16348 as libc::c_int as libc::c_short as libc::c_int
                    + (((*psDec).prevNLSF_Q15[i as usize]
                        - (*psCNG).CNG_smth_NLSF_Q15[i as usize] & 0xffff as libc::c_int)
                        * 16348 as libc::c_int as libc::c_short as libc::c_int
                        >> 16 as libc::c_int);
            i += 1;
        }
        max_Gain_Q16 = 0 as libc::c_int;
        subfr = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if (*psDecCtrl).Gains_Q16[i as usize] > max_Gain_Q16 {
                max_Gain_Q16 = (*psDecCtrl).Gains_Q16[i as usize];
                subfr = i;
            }
            i += 1;
        }
        memmove(
            &mut *((*psCNG).CNG_exc_buf_Q10)
                .as_mut_ptr()
                .offset((*psDec).subfr_length as isize) as *mut libc::c_int
                as *mut libc::c_void,
            ((*psCNG).CNG_exc_buf_Q10).as_mut_ptr() as *const libc::c_void,
            (((4 as libc::c_int - 1 as libc::c_int) * (*psDec).subfr_length)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            ((*psCNG).CNG_exc_buf_Q10).as_mut_ptr() as *mut libc::c_void,
            &mut *((*psDec).exc_Q10)
                .as_mut_ptr()
                .offset((subfr * (*psDec).subfr_length) as isize) as *mut libc::c_int
                as *const libc::c_void,
            ((*psDec).subfr_length as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            (*psCNG).CNG_smth_Gain_Q16
                += ((*psDecCtrl).Gains_Q16[i as usize] - (*psCNG).CNG_smth_Gain_Q16
                    >> 16 as libc::c_int)
                    * 4634 as libc::c_int as libc::c_short as libc::c_int
                    + (((*psDecCtrl).Gains_Q16[i as usize] - (*psCNG).CNG_smth_Gain_Q16
                        & 0xffff as libc::c_int)
                        * 4634 as libc::c_int as libc::c_short as libc::c_int
                        >> 16 as libc::c_int);
            i += 1;
        }
    }
    if (*psDec).lossCnt != 0 {
        skp_silk_cng_exc(
            &mut CNG_sig,
            &psCNG.CNG_exc_buf_Q10,
            (*psCNG).CNG_smth_Gain_Q16,
            length,
            &mut psCNG.rand_seed,
        );
        skp_silk_nlsf2a_stable(
            &mut LPC_buf,
            &psCNG.CNG_smth_NLSF_Q15,
            (*psDec).LPC_order as usize,
        );
        Gain_Q26 = (1 as libc::c_int) << 26 as libc::c_int;
        if (*psDec).LPC_order == 16 as libc::c_int {
            skp_silk_lpc_synthesis_order16(
                &CNG_sig.clone(),
                &LPC_buf,
                Gain_Q26,
                &mut psCNG.CNG_synth_state,
                &mut CNG_sig,
                length as usize,
            );
        } else {
            skp_silk_lpc_synthesis_filter(
                &CNG_sig.clone(),
                &LPC_buf,
                Gain_Q26,
                &mut psCNG.CNG_synth_state,
                &mut CNG_sig,
                length as usize,
                psDec.LPC_order as usize,
            );
        }
        i = 0 as libc::c_int;
        while i < length {
            tmp_32 = *signal.offset(i as isize) as libc::c_int
                + CNG_sig[i as usize] as libc::c_int;
            *signal
                .offset(
                    i as isize,
                ) = (if tmp_32 > 0x7fff as libc::c_int {
                0x7fff as libc::c_int
            } else if tmp_32 < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
                0x8000 as libc::c_int as libc::c_short as libc::c_int
            } else {
                tmp_32
            }) as libc::c_short;
            i += 1;
        }
    } else {
        memset(
            ((*psCNG).CNG_synth_state).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ((*psDec).LPC_order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    };
}
