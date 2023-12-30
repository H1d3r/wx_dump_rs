use crate::{
    skp_silk_bwexpander::skp_silk_bwexpander,
    skp_silk_decode_pitch::skp_silk_decode_pitch,
    skp_silk_decode_pulses::skp_silk_decode_pulses,
    skp_silk_nlsf2a_stable::skp_silk_nlsf2a_stable,
    skp_silk_nlsf_msvq_decode::skp_silk_nlsf_msvq_decode,
    skp_silk_tables_gain::{
        SKP_SILK_DELTA_GAIN_CDF, SKP_SILK_DELTA_GAIN_CDF_OFFSET, SKP_SILK_GAIN_CDF,
        SKP_SILK_GAIN_CDF_OFFSET,
    },
    skp_silk_tables_other::{
        SKP_SILK_FRAME_TERMINATION_CDF, SKP_SILK_FRAME_TERMINATION_OFFSET,
        SKP_SILK_LTP_SCALES_TABLE_Q14, SKP_SILK_LTP_SCALE_CDF, SKP_SILK_LTP_SCALE_OFFSET,
        SKP_SILK_NLSF_INTERPOLATION_FACTOR_CDF, SKP_SILK_NLSF_INTERPOLATION_FACTOR_OFFSET,
        SKP_SILK_SAMPLING_RATES_CDF, SKP_SILK_SAMPLING_RATES_OFFSET, SKP_SILK_SAMPLING_RATES_TABLE,
        SKP_SILK_SEED_CDF, SKP_SILK_SEED_OFFSET, SKP_SILK_VAD_FLAG_CDF, SKP_SILK_VAD_FLAG_OFFSET,
    },
    skp_silk_tables_pitch_lag::{
        SKP_SILK_PITCH_CONTOUR_CDF, SKP_SILK_PITCH_CONTOUR_CDF_OFFSET,
        SKP_SILK_PITCH_CONTOUR_NB_CDF, SKP_SILK_PITCH_CONTOUR_NB_CDF_OFFSET,
        SKP_SILK_PITCH_LAG_MB_CDF, SKP_SILK_PITCH_LAG_MB_CDF_OFFSET, SKP_SILK_PITCH_LAG_NB_CDF,
        SKP_SILK_PITCH_LAG_NB_CDF_OFFSET, SKP_SILK_PITCH_LAG_SWB_CDF,
        SKP_SILK_PITCH_LAG_SWB_CDF_OFFSET, SKP_SILK_PITCH_LAG_WB_CDF,
        SKP_SILK_PITCH_LAG_WB_CDF_OFFSET,
    },
    skp_silk_tables_type_offset::{
        SKP_SILK_TYPE_OFFSET_CDF, SKP_SILK_TYPE_OFFSET_CDF_OFFSET, SKP_SILK_TYPE_OFFSET_JOINT_CDF,
    },
    skp_silk_dec_api::{SkpSilkDecoderControl, SkpSilkDecoderStruct},
    skp_silk_decoder_set_fs::skp_silk_decoder_set_fs,
    SKP_Silk_gain_quant::skp_silk_gains_dequant,
    SKP_Silk_range_coder::{
        skp_silk_range_coder_check_after_decoding, skp_silk_range_coder_get_length,
        skp_silk_range_decoder_multi, SKP_Silk_range_decoder,
    },
    skp_silk_tables_ltp::{
        SKP_SILK_LTP_GAIN_CDF_OFFSETS, SKP_SILK_LTP_GAIN_CDF_PTRS, SKP_SILK_LTP_PER_INDEX_CDF,
        SKP_SILK_LTP_PER_INDEX_CDF_OFFSET, SKP_SILK_LTP_VQ_PTRS_Q14,
    },
};

#[no_mangle]
pub fn skp_silk_decode_parameters(
    ps_dec: &mut SkpSilkDecoderStruct,
    ps_dec_ctrl: &mut SkpSilkDecoderControl,
    q: &mut [i32],
    full_decoding: i32,
) {
    let mut ix;
    let mut p_nlsf0_q15 = [0; 16];
    let ps_r_c = &mut ps_dec.s_r_c;
    if ps_dec.n_frames_decoded == 0 {
        ix = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_SAMPLING_RATES_CDF,
            SKP_SILK_SAMPLING_RATES_OFFSET,
        );
        if ix < 0 || ix > 3 {
            ps_r_c.error = -7;
            return;
        }
        let fs_k_hz_dec = SKP_SILK_SAMPLING_RATES_TABLE[ix as usize];
        skp_silk_decoder_set_fs(ps_dec, fs_k_hz_dec);
    }
    let ps_r_c = &mut ps_dec.s_r_c;
    if ps_dec.n_frames_decoded == 0 {
        ix = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_TYPE_OFFSET_CDF,
            SKP_SILK_TYPE_OFFSET_CDF_OFFSET,
        );
    } else {
        ix = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_TYPE_OFFSET_JOINT_CDF[ps_dec.type_offset_prev as usize],
            SKP_SILK_TYPE_OFFSET_CDF_OFFSET,
        );
    }
    ps_dec_ctrl.sig_type = ix >> 1;
    ps_dec_ctrl.quant_offset_type = ix & 1;
    ps_dec.type_offset_prev = ix;
    let mut gains_indices = [0; 4];
    if ps_dec.n_frames_decoded == 0 {
        gains_indices[0] = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_GAIN_CDF[ps_dec_ctrl.sig_type as usize],
            SKP_SILK_GAIN_CDF_OFFSET,
        );
    } else {
        gains_indices[0] = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_DELTA_GAIN_CDF,
            SKP_SILK_DELTA_GAIN_CDF_OFFSET,
        );
    }

    for i in 1..4 {
        gains_indices[i] = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_DELTA_GAIN_CDF,
            SKP_SILK_DELTA_GAIN_CDF_OFFSET,
        );
    }
    skp_silk_gains_dequant(
        &mut ps_dec_ctrl.gains_q16,
        &gains_indices,
        &mut ps_dec.last_gain_index,
        ps_dec.n_frames_decoded,
    );
    let ps_nlsf_cb = ps_dec.ps_nlsf_cb[ps_dec_ctrl.sig_type as usize].unwrap();
    let mut nlsf_indices = [0; 10];
    skp_silk_range_decoder_multi(
        &mut nlsf_indices,
        ps_r_c,
        ps_nlsf_cb.start_ptr,
        ps_nlsf_cb.middle_ix,
        ps_nlsf_cb.n_stages as usize,
    );
    let mut p_nlsf_q15 = [0; 16];
    skp_silk_nlsf_msvq_decode(&mut p_nlsf_q15, ps_nlsf_cb, &nlsf_indices, ps_dec.lpc_order);
    ps_dec_ctrl.nlsf_interp_coef_q2 = SKP_Silk_range_decoder(
        ps_r_c,
        &SKP_SILK_NLSF_INTERPOLATION_FACTOR_CDF,
        SKP_SILK_NLSF_INTERPOLATION_FACTOR_OFFSET,
    );
    if ps_dec.first_frame_after_reset == 1 {
        ps_dec_ctrl.nlsf_interp_coef_q2 = 4;
    }
    if full_decoding != 0 {
        skp_silk_nlsf2a_stable(
            &mut ps_dec_ctrl.pred_coef_q12[1],
            &p_nlsf_q15,
            ps_dec.lpc_order as usize,
        );
        if ps_dec_ctrl.nlsf_interp_coef_q2 < 4 {
            for i in 0..ps_dec.lpc_order as usize {
                p_nlsf0_q15[i] = ps_dec.prev_nlsf_q15[i]
                    + (ps_dec_ctrl.nlsf_interp_coef_q2 * (p_nlsf_q15[i] - ps_dec.prev_nlsf_q15[i])
                        >> 2);
            }
            skp_silk_nlsf2a_stable(
                &mut (ps_dec_ctrl.pred_coef_q12[0]),
                &p_nlsf0_q15,
                ps_dec.lpc_order as usize,
            );
        } else {
            for i in 0..ps_dec.lpc_order as usize {
                ps_dec_ctrl.pred_coef_q12[0][i] = ps_dec_ctrl.pred_coef_q12[1][i];
            }
        }
    }
    for i in 0..ps_dec.lpc_order as usize {
        ps_dec.prev_nlsf_q15[i] = p_nlsf_q15[i];
    }
    if ps_dec.loss_cnt != 0 {
        skp_silk_bwexpander(
            &mut (ps_dec_ctrl.pred_coef_q12[0]),
            ps_dec.lpc_order as usize,
            63570,
        );
        skp_silk_bwexpander(
            &mut (ps_dec_ctrl.pred_coef_q12[1]),
            ps_dec.lpc_order as usize,
            63570,
        );
    }
    let mut ixs = [0; 4];
    if ps_dec_ctrl.sig_type == 0 {
        if ps_dec.fs_k_hz == 8 {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_NB_CDF,
                SKP_SILK_PITCH_LAG_NB_CDF_OFFSET,
            );
        } else if ps_dec.fs_k_hz == 12 {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_MB_CDF,
                SKP_SILK_PITCH_LAG_MB_CDF_OFFSET,
            );
        } else if ps_dec.fs_k_hz == 16 {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_WB_CDF,
                SKP_SILK_PITCH_LAG_WB_CDF_OFFSET,
            );
        } else {
            ixs[0] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_LAG_SWB_CDF,
                SKP_SILK_PITCH_LAG_SWB_CDF_OFFSET,
            );
        }
        if ps_dec.fs_k_hz == 8 {
            ixs[1] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_CONTOUR_NB_CDF,
                SKP_SILK_PITCH_CONTOUR_NB_CDF_OFFSET,
            );
        } else {
            ixs[1] = SKP_Silk_range_decoder(
                ps_r_c,
                &SKP_SILK_PITCH_CONTOUR_CDF,
                SKP_SILK_PITCH_CONTOUR_CDF_OFFSET,
            );
        }
        skp_silk_decode_pitch(
            ixs[0] as usize,
            ixs[1] as usize,
            &mut ps_dec_ctrl.pitch_l,
            ps_dec.fs_k_hz,
        );
        ps_dec_ctrl.per_index = SKP_Silk_range_decoder(
            ps_r_c,
            &SKP_SILK_LTP_PER_INDEX_CDF,
            SKP_SILK_LTP_PER_INDEX_CDF_OFFSET,
        );
        let cbk_ptr_q14 = SKP_SILK_LTP_VQ_PTRS_Q14[ps_dec_ctrl.per_index as usize];
        for k in 0..4 {
            ix = SKP_Silk_range_decoder(
                ps_r_c,
                SKP_SILK_LTP_GAIN_CDF_PTRS[ps_dec_ctrl.per_index as usize],
                SKP_SILK_LTP_GAIN_CDF_OFFSETS[ps_dec_ctrl.per_index as usize],
            );
            for i in 0..5 {
                ps_dec_ctrl.ltp_coef_q14[k * 5 + i] = cbk_ptr_q14[ix as usize][i];
            }
        }
        ix = SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_LTP_SCALE_CDF, SKP_SILK_LTP_SCALE_OFFSET);
        ps_dec_ctrl.ltp_scale_q14 = SKP_SILK_LTP_SCALES_TABLE_Q14[ix as usize] as i32;
    } else {
        for i in 0..4 {
            ps_dec_ctrl.pitch_l[i] = 0;
        }
        for i in 0..5 * 4 {
            ps_dec_ctrl.ltp_coef_q14[i] = 0;
        }
        ps_dec_ctrl.per_index = 0;
        ps_dec_ctrl.ltp_scale_q14 = 0;
    }
    ix = SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_SEED_CDF, SKP_SILK_SEED_OFFSET);
    ps_dec_ctrl.seed = ix;
    skp_silk_decode_pulses(ps_r_c, ps_dec_ctrl, q, ps_dec.frame_length);
    ps_dec.vad_flag =
        SKP_Silk_range_decoder(ps_r_c, &SKP_SILK_VAD_FLAG_CDF, SKP_SILK_VAD_FLAG_OFFSET);
    ps_dec.frame_termination = SKP_Silk_range_decoder(
        ps_r_c,
        &SKP_SILK_FRAME_TERMINATION_CDF,
        SKP_SILK_FRAME_TERMINATION_OFFSET,
    );
    let mut n_bytes_used = 0;
    skp_silk_range_coder_get_length(ps_r_c, &mut n_bytes_used);
    ps_dec.n_bytes_left = ps_r_c.bufferLength - n_bytes_used;
    if ps_dec.n_bytes_left < 0 {
        ps_r_c.error = -6;
    }
    if ps_dec.n_bytes_left == 0 {
        skp_silk_range_coder_check_after_decoding(ps_r_c);
    }
}
