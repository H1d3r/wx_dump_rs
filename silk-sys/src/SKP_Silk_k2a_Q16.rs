#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn SKP_Silk_k2a_Q16(
    mut A_Q24: *mut libc::c_int,
    mut rc_Q16: *const libc::c_int,
    order: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut Atmp: [libc::c_int; 16] = [0; 16];
    k = 0 as libc::c_int;
    while k < order {
        n = 0 as libc::c_int;
        while n < k {
            Atmp[n as usize] = *A_Q24.offset(n as isize);
            n += 1;
            n;
        }
        n = 0 as libc::c_int;
        while n < k {
            *A_Q24
                .offset(
                    n as isize,
                ) = *A_Q24.offset(n as isize)
                + ((Atmp[(k - n - 1 as libc::c_int) as usize] >> 16 as libc::c_int)
                    * *rc_Q16.offset(k as isize) as libc::c_short as libc::c_int
                    + ((Atmp[(k - n - 1 as libc::c_int) as usize]
                        & 0xffff as libc::c_int)
                        * *rc_Q16.offset(k as isize) as libc::c_short as libc::c_int
                        >> 16 as libc::c_int))
                + Atmp[(k - n - 1 as libc::c_int) as usize]
                    * (if 16 as libc::c_int == 1 as libc::c_int {
                        (*rc_Q16.offset(k as isize) >> 1 as libc::c_int)
                            + (*rc_Q16.offset(k as isize) & 1 as libc::c_int)
                    } else {
                        (*rc_Q16.offset(k as isize)
                            >> 16 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                            >> 1 as libc::c_int
                    });
            n += 1;
            n;
        }
        *A_Q24.offset(k as isize) = -(*rc_Q16.offset(k as isize) << 8 as libc::c_int);
        k += 1;
        k;
    }
}
