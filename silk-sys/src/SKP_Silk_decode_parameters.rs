#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use crate::{SKP_Silk_dec_API::{SKP_Silk_decoder_state, SKP_Silk_decoder_control}, skp_silk_nlsf_msvq_decode::{skp_silk_nlsf_msvq_decode, SkpSilkNlsfCbStruct}, skp_silk_bwexpander::skp_silk_bwexpander, skp_silk_decode_pitch::skp_silk_decode_pitch, SKP_Silk_range_coder::{SKP_Silk_range_decoder, skp_silk_range_decoder_multi, skp_silk_range_coder_get_length, skp_silk_range_coder_check_after_decoding}, SKP_Silk_decoder_set_fs::SKP_Silk_decoder_set_fs, SKP_Silk_gain_quant::skp_silk_gains_dequant, skp_silk_nlsf2a_stable::skp_silk_nlsf2a_stable, skp_silk_decode_pulses::skp_silk_decode_pulses, skp_silk_tables_other::{SKP_SILK_SAMPLING_RATES_CDF, SKP_SILK_SAMPLING_RATES_OFFSET, SKP_SILK_SAMPLING_RATES_TABLE, SKP_SILK_NLSF_INTERPOLATION_FACTOR_CDF, SKP_SILK_NLSF_INTERPOLATION_FACTOR_OFFSET, SKP_SILK_LTP_SCALE_CDF, SKP_SILK_LTP_SCALE_OFFSET, SKP_SILK_LTP_SCALES_TABLE_Q14, SKP_SILK_SEED_CDF, SKP_SILK_SEED_OFFSET, SKP_SILK_VAD_FLAG_CDF, SKP_SILK_VAD_FLAG_OFFSET, SKP_SILK_FRAME_TERMINATION_CDF, SKP_SILK_FRAME_TERMINATION_OFFSET}, skp_silk_tables_type_offset::{SKP_SILK_TYPE_OFFSET_CDF, SKP_SILK_TYPE_OFFSET_CDF_OFFSET, SKP_SILK_TYPE_OFFSET_JOINT_CDF}, skp_silk_tables_gain::{SKP_SILK_GAIN_CDF, SKP_SILK_GAIN_CDF_OFFSET, SKP_SILK_DELTA_GAIN_CDF, SKP_SILK_DELTA_GAIN_CDF_OFFSET}, skp_silk_tables_pitch_lag::{SKP_SILK_PITCH_LAG_NB_CDF, SKP_SILK_PITCH_LAG_NB_CDF_OFFSET, SKP_SILK_PITCH_LAG_MB_CDF, SKP_SILK_PITCH_LAG_MB_CDF_OFFSET, SKP_SILK_PITCH_LAG_WB_CDF, SKP_SILK_PITCH_LAG_WB_CDF_OFFSET, SKP_SILK_PITCH_LAG_SWB_CDF, SKP_SILK_PITCH_LAG_SWB_CDF_OFFSET, SKP_SILK_PITCH_CONTOUR_NB_CDF, SKP_SILK_PITCH_CONTOUR_NB_CDF_OFFSET, SKP_SILK_PITCH_CONTOUR_CDF, SKP_SILK_PITCH_CONTOUR_CDF_OFFSET}, SKP_Silk_tables_LTP::{SKP_Silk_LTP_per_index_CDF, SKP_Silk_LTP_per_index_CDF_offset, SKP_Silk_LTP_vq_ptrs_Q14, SKP_Silk_LTP_gain_CDF_ptrs, SKP_Silk_LTP_gain_CDF_offsets}};
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
pub struct SKP_Silk_NLSF_CBS {
    pub nVectors: libc::c_int,
    pub CB_NLSF_Q15: *const libc::c_short,
    pub Rates_Q5: *const libc::c_short,
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
    pub fs_kHz: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SKP_Silk_CNG_struct {
    pub CNG_exc_buf_Q10: [libc::c_int; 480],
    pub CNG_smth_NLSF_Q15: [libc::c_int; 16],
    pub CNG_synth_state: [libc::c_int; 16],
    pub CNG_smth_Gain_Q16: libc::c_int,
    pub rand_seed: libc::c_int,
    pub fs_kHz: libc::c_int,
}
#[no_mangle]
pub fn SKP_Silk_decode_parameters(
    psDec: &mut SKP_Silk_decoder_state,
    psDecCtrl: &mut SKP_Silk_decoder_control,
    mut q: &mut [i32],
    fullDecoding: i32,
) {
    let mut Ix = 0;
    let mut pNLSF0_Q15 = [0; 16];
    let psRC = &mut psDec.sRC;
    if psDec.nFramesDecoded == 0 {
        Ix = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_SAMPLING_RATES_CDF,
            SKP_SILK_SAMPLING_RATES_OFFSET,
        );
        if Ix < 0 || Ix > 3 {
            psRC.error = -7;
            return;
        }
        let fs_kHz_dec = SKP_SILK_SAMPLING_RATES_TABLE[Ix as usize];
        SKP_Silk_decoder_set_fs(psDec, fs_kHz_dec);
    }
    let psRC = &mut psDec.sRC;
    if psDec.nFramesDecoded == 0 {
        Ix = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_TYPE_OFFSET_CDF,
            SKP_SILK_TYPE_OFFSET_CDF_OFFSET,
        );
    } else {
        Ix = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_TYPE_OFFSET_JOINT_CDF[psDec.typeOffsetPrev as usize],
            SKP_SILK_TYPE_OFFSET_CDF_OFFSET,
        );
    }
    psDecCtrl.sig_type = Ix >> 1;
    psDecCtrl.QuantOffsetType = Ix & 1;
    psDec.typeOffsetPrev = Ix;
    let mut GainsIndices = [0; 4];
    if (*psDec).nFramesDecoded == 0 {
        GainsIndices[0] = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_GAIN_CDF[(*psDecCtrl).sig_type as usize],
            SKP_SILK_GAIN_CDF_OFFSET,
        );
    } else {
        GainsIndices[0] = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_DELTA_GAIN_CDF,
            SKP_SILK_DELTA_GAIN_CDF_OFFSET,
        );
    }

    for i in 1..4 {
        GainsIndices[i] = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_DELTA_GAIN_CDF,
            SKP_SILK_DELTA_GAIN_CDF_OFFSET,
        );
    }
    skp_silk_gains_dequant(
        &mut psDecCtrl.Gains_Q16,
        &GainsIndices,
        &mut psDec.LastGainIndex,
        psDec.nFramesDecoded,
    );
    let psNLSF_CB:&SkpSilkNlsfCbStruct  = (*psDec).psNLSF_CB[(*psDecCtrl).sig_type as usize].unwrap();
    let mut NLSFIndices = [0; 10];
    skp_silk_range_decoder_multi(
        &mut NLSFIndices,
        psRC,
        psNLSF_CB.start_ptr,
        psNLSF_CB.middle_ix,
        psNLSF_CB.n_stages as usize,
    );
    let mut pNLSF_Q15 = [0; 16];
    skp_silk_nlsf_msvq_decode(
        &mut pNLSF_Q15,
        psNLSF_CB,
        &NLSFIndices,
        (*psDec).LPC_order,
    );
    (*psDecCtrl).NLSFInterpCoef_Q2 = SKP_Silk_range_decoder(
        psRC,
        &SKP_SILK_NLSF_INTERPOLATION_FACTOR_CDF,
        SKP_SILK_NLSF_INTERPOLATION_FACTOR_OFFSET,
    );
    if (*psDec).first_frame_after_reset == 1 {
        (*psDecCtrl).NLSFInterpCoef_Q2 = 4;
    }
    if fullDecoding != 0 {
        skp_silk_nlsf2a_stable(
            &mut ((*psDecCtrl).PredCoef_Q12[1]),
            &pNLSF_Q15,
            (*psDec).LPC_order as usize,
        );
        if (*psDecCtrl).NLSFInterpCoef_Q2 < 4 {
            for i in 0..psDec.LPC_order as usize {
                pNLSF0_Q15[i] = psDec.prevNLSF_Q15[i]
                    + (psDecCtrl.NLSFInterpCoef_Q2
                        * (pNLSF_Q15[i] - psDec.prevNLSF_Q15[i])
                        >> 2);
            }
            skp_silk_nlsf2a_stable(
                &mut ((*psDecCtrl).PredCoef_Q12[0]),
                &pNLSF0_Q15,
                (*psDec).LPC_order as usize,
            );
        } else {
            for i in 0..psDec.LPC_order as usize {
                psDecCtrl.PredCoef_Q12[0][i] = psDecCtrl.PredCoef_Q12[1][i];
            }
        }
    }
    for i in 0..psDec.LPC_order as usize {
        psDec.prevNLSF_Q15[i] = pNLSF_Q15[i];
    }
    if (*psDec).lossCnt != 0 {
        skp_silk_bwexpander(
            &mut ((*psDecCtrl).PredCoef_Q12[0]),
            (*psDec).LPC_order as usize,
            63570,
        );
        skp_silk_bwexpander(
            &mut ((*psDecCtrl).PredCoef_Q12[1]),
            (*psDec).LPC_order as usize,
            63570,
        );
    }
    let mut Ixs = [0; 4];
    if (*psDecCtrl).sig_type == 0 {
        if (*psDec).fs_kHz == 8 {
            Ixs[0] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PITCH_LAG_NB_CDF,
                SKP_SILK_PITCH_LAG_NB_CDF_OFFSET,
            );
        } else if (*psDec).fs_kHz == 12 {
            Ixs[0] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PITCH_LAG_MB_CDF,
                SKP_SILK_PITCH_LAG_MB_CDF_OFFSET,
            );
        } else if (*psDec).fs_kHz == 16 {
            Ixs[0] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PITCH_LAG_WB_CDF,
                SKP_SILK_PITCH_LAG_WB_CDF_OFFSET,
            );
        } else {
            Ixs[0] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PITCH_LAG_SWB_CDF,
                SKP_SILK_PITCH_LAG_SWB_CDF_OFFSET,
            );
        }
        if (*psDec).fs_kHz == 8 {
            Ixs[1] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PITCH_CONTOUR_NB_CDF,
                SKP_SILK_PITCH_CONTOUR_NB_CDF_OFFSET,
            );
        } else {
            Ixs[1] = SKP_Silk_range_decoder(
                psRC,
                &SKP_SILK_PITCH_CONTOUR_CDF,
                SKP_SILK_PITCH_CONTOUR_CDF_OFFSET,
            );
        }
        skp_silk_decode_pitch(
            Ixs[0] as usize,
            Ixs[1] as usize,
            &mut psDecCtrl.pitchL,
            psDec.fs_kHz,
        );
        (*psDecCtrl).PERIndex = SKP_Silk_range_decoder(
            psRC,
            &SKP_Silk_LTP_per_index_CDF,
            SKP_Silk_LTP_per_index_CDF_offset,
        );
        let cbk_ptr_Q14 = SKP_Silk_LTP_vq_ptrs_Q14[(*psDecCtrl).PERIndex as usize];
        for k in 0..4 {
            Ix = SKP_Silk_range_decoder(
                psRC,
                SKP_Silk_LTP_gain_CDF_ptrs[(*psDecCtrl).PERIndex as usize],
                SKP_Silk_LTP_gain_CDF_offsets[(*psDecCtrl).PERIndex as usize],
            );
            for i in 0..5 {
                psDecCtrl.LTPCoef_Q14[k * 5 + i] = cbk_ptr_Q14[Ix as usize][i];
            }
        }
        Ix = SKP_Silk_range_decoder(
            psRC,
            &SKP_SILK_LTP_SCALE_CDF,
            SKP_SILK_LTP_SCALE_OFFSET,
        );
        (*psDecCtrl)
            .LTP_scale_Q14 = SKP_SILK_LTP_SCALES_TABLE_Q14[Ix as usize] as i32;
    } else {
        for i in 0..4 {
            psDecCtrl.pitchL[i] = 0;
        }
        for i in 0..5 * 4 {
            psDecCtrl.LTPCoef_Q14[i] = 0;
        }
        (*psDecCtrl).PERIndex = 0;
        (*psDecCtrl).LTP_scale_Q14 = 0;
    }
    Ix = SKP_Silk_range_decoder(
        psRC,
        &SKP_SILK_SEED_CDF,
        SKP_SILK_SEED_OFFSET,
    );
    (*psDecCtrl).Seed = Ix;
    skp_silk_decode_pulses(psRC, psDecCtrl, q, (*psDec).frame_length);
    (*psDec).vadFlag = SKP_Silk_range_decoder(
        psRC,
        &SKP_SILK_VAD_FLAG_CDF,
        SKP_SILK_VAD_FLAG_OFFSET,
    );
    (*psDec).FrameTermination = SKP_Silk_range_decoder(
        psRC,
        &SKP_SILK_FRAME_TERMINATION_CDF,
        SKP_SILK_FRAME_TERMINATION_OFFSET,
    );
    let mut nBytesUsed = 0;
    skp_silk_range_coder_get_length(psRC, &mut nBytesUsed);
    (*psDec).nBytesLeft = (*psRC).bufferLength - nBytesUsed;
    if (*psDec).nBytesLeft < 0 {
        (*psRC).error = -6;
    }
    if (*psDec).nBytesLeft == 0 {
        skp_silk_range_coder_check_after_decoding(psRC);
    }
}
