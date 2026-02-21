//! cross platform utilities for permanently loading libraries
//!
//! Main use case here is just permanently having mono/dobby/etc loaded, unlike with libloading,
//! where they get unloaded when the library goes out of scope.
//!
//! # Safety
//! this is incredibly unsafe
//!
//! # Examples
//!
//! ```
//! use std::path::PathBuf;
//! use std::mem;
//!
//! use unity_rs::libs::load_lib;
//!
//! let path = PathBuf::from("path/to/lib");
//! let lib = load_lib(&path)?;
//!
//! let func: extern fn() = unsafe { mem::transmute(lib.get_fn_ptr("func_name")?) };
use std::collections::HashMap;
use std::{
    ffi::c_void,
    marker::PhantomData,
    ops::Deref,
    path::{Path, PathBuf},
};
use thiserror::Error;
static EXPORTS: &[(&str, &str)] = &[("il2cpp_init", "GdnPlVZEdxf"), ("il2cpp_init_utf16", "xeESwBbuab_"), ("il2cpp_shutdown", "nKyghkcwxJe"), ("il2cpp_set_config_dir", "XiKOzOdJsYE"), ("il2cpp_set_data_dir", "AbJEjDq_zgF"), ("il2cpp_set_temp_dir", "JerpDnxOVno"), ("il2cpp_set_commandline_arguments", "NkyFkhKjXDP"), ("il2cpp_set_commandline_arguments_utf16", "UfNXctJaZQO"), ("il2cpp_set_config_utf16", "MJdRIVcUmWY"), ("il2cpp_set_config", "igjWeppGXcn"), ("il2cpp_set_memory_callbacks", "NPuDEZOXejP"), ("il2cpp_memory_pool_set_region_size", "EAFNJ_FIJYK"), ("il2cpp_memory_pool_get_region_size", "gdwVBiglIWC"), ("il2cpp_get_corlib", "NGbgctWOdID"), ("il2cpp_add_internal_call", "kdoNiyv_Qhd"), ("il2cpp_resolve_icall", "UWZdnspJd_I"), ("il2cpp_alloc", "KaCKWSaBPCP"), ("il2cpp_free", "PRfrmVSTTrU"), ("il2cpp_array_class_get", "LAiuXZUDdNh"), ("il2cpp_array_length", "QRIjzueyGZT"), ("il2cpp_array_get_byte_length", "jzmYhGGuWvJ"), ("il2cpp_array_new", "Nlz_loEWfav"), ("il2cpp_array_new_specific", "KBFdoRXhdON"), ("il2cpp_array_new_full", "cJrJgWxPWSl"), ("il2cpp_bounded_array_class_get", "fyViQqbTTPe"), ("il2cpp_array_element_size", "rYLHFyGgORz"), ("il2cpp_assembly_get_image", "WrPjnXSxwvx"), ("il2cpp_class_for_each", "bU_ybtImqeh"), ("il2cpp_class_enum_basetype", "GgOmkULknVl"), ("il2cpp_class_is_inited", "gnQolWebGMV"), ("il2cpp_class_is_generic", "zLPRtXMqDka"), ("il2cpp_class_is_inflated", "vkWLZpdNfvo"), ("il2cpp_class_is_assignable_from", "XkjrtGAMQyu"), ("il2cpp_class_is_subclass_of", "bXykbnZeM__"), ("il2cpp_class_has_parent", "rFwPWfVVfiU"), ("il2cpp_class_from_il2cpp_type", "OCipriHLgON"), ("il2cpp_class_from_name", "lQyvUscAGEB"), ("il2cpp_class_from_system_type", "UgeTybrtfli"), ("il2cpp_class_get_element_class", "rLtGEybL_mo"), ("il2cpp_class_get_events", "WYodECZGcFP"), ("il2cpp_class_get_fields", "QZg_eWEoySR"), ("il2cpp_class_get_nested_types", "gcfwVglwRWR"), ("il2cpp_class_get_interfaces", "wfeudlBuJmW"), ("il2cpp_class_get_properties", "ONYXmIhnfVE"), ("il2cpp_class_get_property_from_name", "fHtXWY_hYew"), ("il2cpp_class_get_field_from_name", "pwnfMkbaSCR"), ("il2cpp_class_get_methods", "XnXhgwwvBVH"), ("il2cpp_class_get_method_from_name", "LmZVehSCtkU"), ("il2cpp_class_get_name", "dswOlGpdlQB"), ("il2cpp_type_get_name_chunked", "XvW_nQkLeKA"), ("il2cpp_class_get_namespace", "WBDCidKvO_s"), ("il2cpp_class_get_parent", "WnhlxMYyqXH"), ("il2cpp_class_get_declaring_type", "WrizOXPSfDX"), ("il2cpp_class_instance_size", "cDrnGzyYYCV"), ("il2cpp_class_num_fields", "yZLIuPWSRcp"), ("il2cpp_class_is_valuetype", "NjDEOSOBNto"), ("il2cpp_class_value_size", "RtljfJxmxEU"), ("il2cpp_class_is_blittable", "HXApjJi_zkt"), ("il2cpp_class_get_flags", "PfeRozEYgvw"), ("il2cpp_class_is_abstract", "WzhlplRFtZf"), ("il2cpp_class_is_interface", "WssGtOLCvEB"), ("il2cpp_class_array_element_size", "EumfCrOhMGs"), ("il2cpp_class_from_type", "smBQHSuYocJ"), ("il2cpp_class_get_type", "iFUPHlNvxHm"), ("il2cpp_class_get_type_token", "gKusixTMGyH"), ("il2cpp_class_has_attribute", "NvlrWGlVUQv"), ("il2cpp_class_has_references", "sXmkDWLesBx"), ("il2cpp_class_is_enum", "UlxOBFbttaI"), ("il2cpp_class_get_image", "DhWncAFKTmf"), ("il2cpp_class_get_assemblyname", "utbv_l_vaJh"), ("il2cpp_class_get_rank", "YlMdMSGIETk"), ("il2cpp_class_get_data_size", "eHFwUICBnYH"), ("il2cpp_class_get_static_field_data", "aNHKESETrgt"), ("il2cpp_class_get_bitmap_size", "WkzPjgTvSyM"), ("il2cpp_class_get_bitmap", "VhiRbDWWYeu"), ("il2cpp_stats_dump_to_file", "qxUbeLx_JRW"), ("il2cpp_stats_get_value", "mpPYSsTepIF"), ("il2cpp_domain_get", "dUunpQADaAk"), ("il2cpp_domain_assembly_open", "bzhujLBvtDQ"), ("il2cpp_domain_get_assemblies", "kSNWlvRcuDN"), ("il2cpp_raise_exception", "YDFMwomdxHH"), ("il2cpp_exception_from_name_msg", "lfkDsxPTfxx"), ("il2cpp_get_exception_argument_null", "fOphHxtGNbi"), ("il2cpp_format_exception", "PWiBDLAcqEE"), ("il2cpp_format_stack_trace", "hYTrHKAtiVN"), ("il2cpp_unhandled_exception", "yDsipRDieRK"), ("il2cpp_native_stack_trace", "Tup_trWcPft"), ("il2cpp_field_get_flags", "kPkuadGKLtY"), ("il2cpp_field_get_name", "YsrafFiNMPb"), ("il2cpp_field_get_parent", "SCpPaWMGYuU"), ("il2cpp_field_get_offset", "YnxEeoyzDsw"), ("il2cpp_field_get_type", "vrXvLlMNtUF"), ("il2cpp_field_get_value", "jkItflHvEgX"), ("il2cpp_field_get_value_object", "nTqDKByJShq"), ("il2cpp_field_has_attribute", "GBoRqEfHSur"), ("il2cpp_field_set_value", "uEATVRQMULc"), ("il2cpp_field_static_get_value", "iL_PocEDOUL"), ("il2cpp_field_static_set_value", "bWETBqDYXSG"), ("il2cpp_field_set_value_object", "EZWtZRckXge"), ("il2cpp_field_is_literal", "ZpVlhnPyPjF"), ("il2cpp_gc_collect", "cxFieV_KvfU"), ("il2cpp_gc_collect_a_little", "FLnvWMirTvD"), ("il2cpp_gc_start_incremental_collection", "WsXbVfkwBFl"), ("il2cpp_gc_disable", "OrADuiKfRaV"), ("il2cpp_gc_enable", "OnAc_HRKlnn"), ("il2cpp_gc_is_disabled", "CGhT_qkYMhz"), ("il2cpp_gc_set_mode", "knvVyiMUlFH"), ("il2cpp_gc_get_max_time_slice_ns", "xmHvAlsCgvg"), ("il2cpp_gc_set_max_time_slice_ns", "IAHkloIMjeA"), ("il2cpp_gc_is_incremental", "ezlpJ_GUuck"), ("il2cpp_gc_get_used_size", "_jtTAs_IieI"), ("il2cpp_gc_get_heap_size", "jlxgOdsyfel"), ("il2cpp_gc_wbarrier_set_field", "DDuUjGcKUaj"), ("il2cpp_gc_has_strict_wbarriers", "XIBLQ_EpJtY"), ("il2cpp_gc_set_external_allocation_tracker", "YMqyW_Qt_Ld"), ("il2cpp_gc_set_external_wbarrier_tracker", "yG_WSutBToN"), ("il2cpp_gc_foreach_heap", "dNQnXjbmJKE"), ("il2cpp_stop_gc_world", "mIgMYhkAJMu"), ("il2cpp_start_gc_world", "pqnyVuh_ujd"), ("il2cpp_gc_alloc_fixed", "iVJSVIQCwWl"), ("il2cpp_gc_free_fixed", "xvhyUdIbVgy"), ("il2cpp_gchandle_new", "HRWlelkrVPt"), ("il2cpp_gchandle_new_weakref", "mZCAEcHuXyK"), ("il2cpp_gchandle_get_target", "TxCTtfvbDXh"), ("il2cpp_gchandle_free", "bETTzCiChhf"), ("il2cpp_gchandle_foreach_get_target", "OhgKQzlOETr"), ("il2cpp_object_header_size", "HJPxFieAabM"), ("il2cpp_array_object_header_size", "KrZKSmKtPJy"), ("il2cpp_offset_of_array_length_in_array_object_header", "ipNioOb_TTJ"), ("il2cpp_offset_of_array_bounds_in_array_object_header", "OpdiQkwodGd"), ("il2cpp_allocation_granularity", "jDHAPfbJGb_"), ("il2cpp_unity_liveness_allocate_struct", "YxvMvG_EOgK"), ("il2cpp_unity_liveness_calculation_from_root", "fxW_ygbYUpI"), ("il2cpp_unity_liveness_calculation_from_statics", "cmnrTHTRjeg"), ("il2cpp_unity_liveness_finalize", "ckudBVjX_tc"), ("il2cpp_unity_liveness_free_struct", "rQ_xDeHwdph"), ("il2cpp_method_get_return_type", "iOGphJxpUYi"), ("il2cpp_method_get_declaring_type", "SEwCkKakqHn"), ("il2cpp_method_get_name", "jgmSLpUuObu"), ("il2cpp_method_get_from_reflection", "bY_ngrNPSBp"), ("il2cpp_method_get_object", "uJokkJUjpKT"), ("il2cpp_method_is_generic", "RaDdigmblJd"), ("il2cpp_method_is_inflated", "MHOEcxWMXnX"), ("il2cpp_method_is_instance", "FYYVhKttIeM"), ("il2cpp_method_get_param_count", "SSrrPJtgRsy"), ("il2cpp_method_get_param", "OiCJwDuUwiM"), ("il2cpp_method_get_class", "gOZrNrFqRag"), ("il2cpp_method_has_attribute", "ZXROjjfaFd_"), ("il2cpp_method_get_flags", "niJNnKvNPvP"), ("il2cpp_method_get_token", "YMPcFDDspTK"), ("il2cpp_method_get_param_name", "TH_aVezXhJW"), ("il2cpp_property_get_flags", "MIEDlUbeuMx"), ("il2cpp_property_get_get_method", "CStyEE_NTSO"), ("il2cpp_property_get_set_method", "ErevOpxaNmP"), ("il2cpp_property_get_name", "ZQdUTOLZwVf"), ("il2cpp_property_get_parent", "sbOVZuGEH_y"), ("il2cpp_object_get_class", "qbFQR_fYihi"), ("il2cpp_object_get_size", "dQqlXFwQrJp"), ("il2cpp_object_get_virtual_method", "uqHFVbHaEcE"), ("il2cpp_object_new", "WCEoPQXKscc"), ("il2cpp_object_unbox", "syjaQBHbEba"), ("il2cpp_value_box", "NrgBtNRZraW"), ("il2cpp_monitor_enter", "awVxxaDkin_"), ("il2cpp_monitor_try_enter", "BcAaeBubdxr"), ("il2cpp_monitor_exit", "CKZFssaXtSR"), ("il2cpp_monitor_pulse", "BuJvkFvGnJJ"), ("il2cpp_monitor_pulse_all", "XSZeuqqCyAD"), ("il2cpp_monitor_wait", "nNNrsaoljaS"), ("il2cpp_monitor_try_wait", "mDSBbarFRGX"), ("il2cpp_runtime_invoke", "KsaFALsaHZl"), ("il2cpp_runtime_invoke_convert_args", "CcKieJOuNhD"), ("il2cpp_runtime_class_init", "HpawpbBmsnf"), ("il2cpp_runtime_object_init", "JEyEEDZogZe"), ("il2cpp_runtime_object_init_exception", "kFJQOElRJQz"), ("il2cpp_runtime_unhandled_exception_policy_set", "yuiRVqneHRq"), ("il2cpp_string_length", "DDuIaqSaJPb"), ("il2cpp_string_chars", "CAqjfDvGfOU"), ("il2cpp_string_new", "moZkrEOxnod"), ("il2cpp_string_new_len", "HiNmwaRQnUU"), ("il2cpp_string_new_utf16", "FQBIXIelhkZ"), ("il2cpp_string_new_wrapper", "sodVkROQWPR"), ("il2cpp_string_intern", "browFOTyQoT"), ("il2cpp_string_is_interned", "TmLQdbMHyHl"), ("il2cpp_thread_current", "XHlseHzpXli"), ("il2cpp_thread_attach", "yXKCkTHabfE"), ("il2cpp_thread_detach", "mUkuUnAcYiz"), ("il2cpp_thread_get_all_attached_threads", "rpORpEUDUUG"), ("il2cpp_is_vm_thread", "YTwnbYaBvtF"), ("il2cpp_current_thread_walk_frame_stack", "_kIiiFONOWj"), ("il2cpp_thread_walk_frame_stack", "txCdvPkSQ_U"), ("il2cpp_current_thread_get_top_frame", "aBVnkQmVagn"), ("il2cpp_thread_get_top_frame", "QfgAeisWpKk"), ("il2cpp_current_thread_get_frame_at", "ljruylFjhzQ"), ("il2cpp_thread_get_frame_at", "vfdtQi_BLnN"), ("il2cpp_current_thread_get_stack_depth", "_jfwrZMaxrS"), ("il2cpp_thread_get_stack_depth", "zxvFVDsgBzv"), ("il2cpp_override_stack_backtrace", "AgtoKcmHcDU"), ("il2cpp_type_get_object", "tRAcfCA_cJ_"), ("il2cpp_type_get_type", "YqifqTyuPPh"), ("il2cpp_type_get_class_or_element_class", "T__hQLBTfcK"), ("il2cpp_type_get_name", "hLEJNjZDdWu"), ("il2cpp_type_is_byref", "CLNqWpxdFVp"), ("il2cpp_type_get_attrs", "Uz_cHOJRUgo"), ("il2cpp_type_equals", "AridOgtMmI_"), ("il2cpp_type_get_assembly_qualified_name", "rrU_vIEwXbL"), ("il2cpp_type_get_reflection_name", "snWGrBGxadY"), ("il2cpp_type_is_static", "ffYPiNKgTWN"), ("il2cpp_type_is_pointer_type", "BgeV_bMfWIG"), ("il2cpp_image_get_assembly", "BNuYOmYhPYR"), ("il2cpp_image_get_name", "ikzAkyTEACE"), ("il2cpp_image_get_filename", "oZQRiHguhsf"), ("il2cpp_image_get_entry_point", "btgtJbGbhiv"), ("il2cpp_image_get_class_count", "CQzpQfpTRri"), ("il2cpp_image_get_class", "zUdXVkYrdqv"), ("il2cpp_capture_memory_snapshot", "mplYajYvgG_"), ("il2cpp_free_captured_memory_snapshot", "EFaaD_TxFmT"), ("il2cpp_set_find_plugin_callback", "mpxpzMXGXjG"), ("il2cpp_register_log_callback", "oHdI_cT_Dzm"), ("il2cpp_debugger_set_agent_options", "yTOgoQPzSCr"), ("il2cpp_is_debugger_attached", "hJkADbjlltQ"), ("il2cpp_register_debugger_agent_transport", "hKyInMXWsOU"), ("il2cpp_debug_get_method_info", "FqpgSyKjrdo"), ("il2cpp_unity_install_unitytls_interface", "JigYUjFcHix"), ("il2cpp_custom_attrs_from_class", "wDJQiJNSxdU"), ("il2cpp_custom_attrs_from_method", "hdgWuNBjZHE"), ("il2cpp_custom_attrs_from_field", "TSNtIHFohaO"), ("il2cpp_custom_attrs_get_attr", "jtFcNDldAcC"), ("il2cpp_custom_attrs_has_attr", "iFJnoOxafey"), ("il2cpp_custom_attrs_construct", "KPoSbzPCuBP"), ("il2cpp_custom_attrs_free", "YMpwqonKaMS"), ("il2cpp_class_set_userdata", "IN_yUMndXZh"), ("il2cpp_class_get_userdata_offset", "iDJFVOADMyw"), ("il2cpp_set_default_thread_affinity", "ygXP_vcjIjp"), ("il2cpp_unity_set_android_network_up_state_func", "vq_ROHDlwlL"), ];

pub fn get_value(key: &str) -> Option<&str> {
    EXPORTS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

pub fn get(key: &str) -> &str {
    get_value(key).unwrap_or(key)
}
/// possible library loading errors
#[derive(Debug, Error)]
pub enum LibError {
    /// failed to load library
    #[error("Failed to load library!")]
    FailedToLoadLib,

    /// failed to get lib name
    #[error("Failed to get lib name!")]
    FailedToGetLibName,

    /// failed to get lib path
    #[error("Failed to get lib path!")]
    FailedToGetLibPath,

    /// failed to get function pointer
    #[error("Failed to get function pointer: {0}")]
    FailedToGetFnPtr(String),

    #[error("Failed to create C-String")]
    FailedToCreateCString,
}

/// a representation of a permanently loaded library
#[derive(Debug, Clone)]
pub struct NativeLibrary {
    /// the name of the lib
    pub name: String,
    /// the path to the lib
    pub path: PathBuf,
    /// the pointer to the lib
    pub handle: *mut c_void,
}

impl NativeLibrary {
    /// gets a function pointer
    #[cfg(not(target_os = "windows"))]
    pub fn sym<T>(&self, name_strr: &str) -> Result<NativeMethod<T>, LibError> {
        let name_str = get(name_strr);
        let display_string = name_str.to_string();

        let name = std::ffi::CString::new(name_str).map_err(|_| LibError::FailedToCreateCString)?;
        let ptr = unsafe { libc::dlsym(self.handle, name.as_ptr()) };
        if ptr.is_null() {
            return Err(LibError::FailedToGetFnPtr(display_string));
        }

        Ok(NativeMethod {
            inner: ptr.cast(),
            pd: PhantomData,
        })
    }
    /// gets a function pointer
    #[cfg(target_os = "windows")]
    pub fn sym<T>(&self, name_strr: &str) -> Result<NativeMethod<T>, LibError> {
        let name_str = get(name_strr);
        use std::ffi::CString;

        let display_string = name_str.to_string();

        use winapi::um::libloaderapi::GetProcAddress;

        let name = CString::new(name_str).map_err(|_| LibError::FailedToCreateCString)?;

        let ptr = unsafe { GetProcAddress(self.handle.cast(), name.as_ptr()) };
        if ptr.is_null() {
            return Err(LibError::FailedToGetFnPtr(display_string));
        }

        Ok(NativeMethod {
            inner: ptr.cast(),
            pd: PhantomData,
        })
    }
}

/// loads a library permanently
///
/// # Arguments
///
/// * `path` - the path to the library
///
/// # Errors
///
/// * `LibError::FailedToLoadLib` - if the library failed to load
/// * `LibError::FailedToGetLibName` - if the library name failed to be retrieved
///
/// # Safety
///
/// this function is unsafe because it permanently loads a library
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use unity_rs::libs::load_lib;
///
/// let path = PathBuf::from("path/to/lib");
/// let lib = load_lib(&path);
///
/// assert!(lib.is_ok());
#[cfg(not(target_os = "windows"))]
pub fn load_lib<P: AsRef<Path>>(path: P) -> Result<NativeLibrary, LibError> {
    use std::ffi::CString;

    let path = path.as_ref();

    let path_string = path.to_str().ok_or(LibError::FailedToGetLibPath)?;

    let c_path = CString::new(path_string).map_err(|_| LibError::FailedToCreateCString)?;

    let lib = unsafe { libc::dlopen(c_path.as_ptr(), libc::RTLD_NOW | libc::RTLD_GLOBAL) };

    if lib.is_null() {
        return Err(LibError::FailedToLoadLib);
    }

    let lib_name = path
        .file_name()
        .ok_or(LibError::FailedToGetLibName)?
        .to_str()
        .ok_or(LibError::FailedToGetLibName)?
        .to_string();

    Ok(NativeLibrary {
        name: lib_name,
        path: path.to_path_buf(),
        handle: lib,
    })
}

/// loads a library permanently
///
/// # Arguments
///
/// * `path` - the path to the library
///
/// # Errors
///
/// * `LibError::FailedToLoadLib` - if the library failed to load
/// * `LibError::FailedToGetLibName` - if the library name failed to be retrieved
///
/// # Safety
///
/// this function is unsafe because it permanently loads a library
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use unity_rs::libs::load_lib;
///
/// let path = PathBuf::from("path/to/lib");
/// let lib = load_lib(&path);
///
/// assert!(lib.is_ok());
#[cfg(target_os = "windows")]
pub fn load_lib<P: AsRef<Path>>(path: P) -> Result<NativeLibrary, LibError> {
    use std::ffi::CString;

    let path = path.as_ref();

    use winapi::um::libloaderapi::LoadLibraryA;

    let path_string = path.to_str().ok_or_else(|| LibError::FailedToGetLibPath)?;
    let win_path = CString::new(path_string).map_err(|_| LibError::FailedToCreateCString)?;

    let lib = unsafe { LoadLibraryA(win_path.as_ptr()) };

    if lib.is_null() {
        return Err(LibError::FailedToLoadLib);
    }

    let lib_name = path
        .file_name()
        .ok_or(LibError::FailedToGetLibName)?
        .to_str()
        .ok_or(LibError::FailedToGetLibName)?
        .to_string();

    Ok(NativeLibrary {
        name: lib_name,
        path: path.to_path_buf(),
        handle: lib.cast(),
    })
}

#[derive(Debug)]
pub struct NativeMethod<T> {
    pub inner: *mut c_void,
    pd: PhantomData<T>,
}

unsafe impl<T: Send> Send for NativeMethod<T> {}
unsafe impl<T: Sync> Sync for NativeMethod<T> {}

impl<T> Clone for NativeMethod<T> {
    fn clone(&self) -> NativeMethod<T> {
        NativeMethod { ..*self }
    }
}

impl<T> Deref for NativeMethod<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(&self.inner as *const *mut _ as *const T) }
    }
}
