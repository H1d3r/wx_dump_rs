use crate::wx_util::{get_wechat_data_by_real_addr, get_wechat_modules, WeChatInfo};

pub fn memory_search(
    wechat_info: &WeChatInfo,
    data: &[u8],
    real_addr: bool,
) -> anyhow::Result<Vec<usize>> {
    let vec = get_wechat_data_by_real_addr(
        wechat_info,
        wechat_info.module.modBaseAddr as usize,
        wechat_info.module.modBaseSize as usize,
    )?;
    let r = (0..vec.len() - data.len())
        .filter(|&i| &vec[i..i + data.len()] == data)
        .map(|i| {
            if real_addr {
                wechat_info.module.modBaseAddr as usize + i
            } else {
                i
            }
        })
        .collect();
    Ok(r)
}

pub fn memory_search_from_wechat_all_data(
    wechat_info: &WeChatInfo,
    data: &[u8],
    real_addr: bool,
) -> anyhow::Result<()> {
    for module in get_wechat_modules(wechat_info.process)? {
        let vec = get_wechat_data_by_real_addr(
            wechat_info,
            module.modBaseAddr as usize,
            module.modBaseSize as usize,
        );
        match vec {
            Ok(vec) => {
                let r: Vec<usize> = (0..vec.len() - data.len())
                .filter(|&i| &vec[i..i + data.len()] == data)
                .map(|i| {
                    if real_addr {
                        module.modBaseAddr as usize + i
                    } else {
                        i
                    }
                })
                .collect();
            if r.len() > 0 {
                println!("module: {}",String::from_utf8(module.szModule.split(|n| *n == 0).next().unwrap().to_vec())?);
                println!("{:?}", r);
            }
            },
            Err(err) => {
                println!("获取内存失败。module: {}。err: {err:?}",String::from_utf8(module.szModule.split(|n| *n == 0).next().unwrap().to_vec())?);
                println!("addr start: {:?},size: {:?},end: {:?}",module.modBaseAddr as usize,module.modBaseSize as usize,module.modBaseAddr as usize + module.modBaseSize as usize);
                continue;
            },
        }
    }
    Ok(())
}
pub fn get_memory(
    wechat_info: &WeChatInfo,
    index: usize,
    len: usize,
    real_addr: bool,
) -> anyhow::Result<Vec<u8>> {
    let vec = if real_addr {
        get_wechat_data_by_real_addr(wechat_info, index, len)?
    } else {
        get_wechat_data_by_real_addr(
            wechat_info,
            wechat_info.module.modBaseAddr as usize + index,
            len,
        )?
    };
    Ok(vec)
}
