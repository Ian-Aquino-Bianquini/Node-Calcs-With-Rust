use nodejs_sys::{
    napi_env, 
    napi_set_named_property, 
    napi_value, 
    napi_create_function, 
    napi_callback_info,
    napi_get_cb_info,
    napi_get_value_double,
    napi_create_double
};

use std::ffi::CString;

pub unsafe extern "C" fn somafn(env: napi_env, info: napi_callback_info) -> napi_value{
    let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 2 as usize;
    napi_get_cb_info(env, info, &mut argc, buffer.as_mut_ptr(), std::ptr::null_mut(), std::ptr::null_mut());
    let mut x = 0 as f64;
    let mut y = 0 as f64;
    napi_get_value_double(env, buffer[0], &mut x);
    napi_get_value_double(env, buffer[1], &mut y);

    let mut local:napi_value = std::mem::zeroed();
    napi_create_double(env, x + y,&mut local);
    local
} 

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(env: napi_env, exports: napi_value) -> nodejs_sys::napi_value {
    let soma = CString::new("soma").expect("CString::new failed");
    let mut local:napi_value = std::mem::zeroed();
    napi_create_function(env, soma.as_ptr(), 2, Some(somafn), std::ptr::null_mut(), &mut local);
    napi_set_named_property(env, exports, soma.as_ptr(), local);
    exports
}

