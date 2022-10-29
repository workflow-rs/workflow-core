pub fn buffer_as_slice<'data, T: 'data>(
    data : &'data [u8],
    byte_offset: usize,
    elements: usize
) -> &'data [T] {
    unsafe {
        std::slice::from_raw_parts::<T>(
            std::mem::transmute::<_,*const T>(
                data.as_ptr().offset(byte_offset as isize),
            ),
        elements)
    }
}

pub fn buffer_as_slice_mut<'data, T: 'data>(
    data : &'data mut [u8],
    byte_offset: usize,
    elements: usize
) -> &mut [T] {
    unsafe {
        std::slice::from_raw_parts_mut::<T>(
            std::mem::transmute::<_,*mut T>(
                data.as_mut_ptr().offset(byte_offset as isize),
            ),
        elements)
    }
}


pub fn struct_as_slice_u8<'data, T: 'data>(
    data : &T,
) -> &'data [u8] {
    unsafe {
        std::slice::from_raw_parts::<u8>(
            std::mem::transmute::<_,*const u8>(
                data
            ),
        std::mem::size_of::<T>())
    }
}

pub fn substring(str:&str, start:usize, length:usize)->String{
    str[start..length.min(str.len())].to_string()
}

pub fn substr(str:&str, start:usize, length:usize, append:Option<&str>)->String{
    let len = str.len();
    let str = str[start..length.min(len)].to_string();
    if len > length{
        str+append.unwrap_or("...")
    }else{
        str
    }
}


