use crate::{
    skp_l_shift, skp_l_shift_sat_32, skp_s_m_mul, skp_s_mla_w_b, skp_s_mla_w_w, skp_s_mul_w_b,
};

fn skp_silk_clz16(mut in16: i16) -> i32 {
    let mut out32 = 0;
    if in16 == 0 {
        return 16;
    }
    if in16 as i32 & 0xff00 != 0 {
        if in16 as i32 & 0xf000 != 0 {
            in16 = in16 >> 12;
        } else {
            out32 += 4;
            in16 = in16 >> 8;
        }
    } else if in16 as i32 & 0xfff0 != 0 {
        out32 += 8;
        in16 = in16 >> 4;
    } else {
        out32 += 12;
    }
    if in16 & 0xc != 0 {
        if in16 & 0x8 != 0 {
            out32 + 0
        } else {
            out32 + 1
        }
    } else if in16 & 0xe != 0 {
        out32 + 2
    } else {
        out32 + 3
    }
}

pub fn skp_silk_clz32(in32: i32) -> i32 {
    if in32 as u32 & 0xffff0000 != 0 {
        skp_silk_clz16((in32 >> 16) as i16)
    } else {
        skp_silk_clz16(in32 as i16) + 16 as i32
    }
}

pub fn skp_inverse32_var_q(b32: i32, q_res: i32) -> i32 {
    let b_head_rm = skp_silk_clz32(b32.abs()) - 1;
    let b32_nrm = b32 << b_head_rm;
    let b32_inv = (i32::MAX >> 2) / (b32_nrm >> 16);
    let mut result = b32_inv << 16;
    let err_q32 = -skp_s_mul_w_b!(b32_nrm, b32_inv) >> 3;
    result = skp_s_mla_w_w!(result, err_q32, b32_inv);
    let l_shift = 61 - b_head_rm - q_res;
    if l_shift <= 0 {
        skp_l_shift_sat_32!(result, -l_shift)
    } else if l_shift < 32 {
        result >> l_shift
    } else {
        0
    }
}

pub fn skp_div32_var_q(a32: i32, b32: i32, q_res: i32) -> i32 {
    let a_head_rm = skp_silk_clz32(a32.abs()) - 1;
    let mut a32_nrm = a32 << a_head_rm;
    let b_head_rm = skp_silk_clz32(b32.abs()) - 1;
    let b32_nrm = b32 << b_head_rm;
    let b32_inv = (i32::MAX >> 2) / (b32_nrm >> 16);
    let mut result = skp_s_mul_w_b!(a32_nrm, b32_inv);
    a32_nrm -= skp_l_shift!(skp_s_m_mul!(b32_nrm, result), 3);
    result = skp_s_mla_w_b!(result, a32_nrm, b32_inv);

    let l_shift = 29 + a_head_rm - b_head_rm - q_res;
    if l_shift <= 0 {
        skp_l_shift_sat_32!(result, -l_shift)
    } else if l_shift < 32 {
        result >> l_shift
    } else {
        0
    }
}
