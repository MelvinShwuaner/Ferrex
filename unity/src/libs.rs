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
static EXPORTS: &[(&str, &str)] = &[("il2cpp_init", "iEpdBYjUgCF"), ("il2cpp_init_utf16", "MmzEGUhaxQL"), ("il2cpp_shutdown", "gJdbsQTYtyL"), ("il2cpp_set_config_dir", "VtWtPRxCQWC"), ("il2cpp_set_data_dir", "qRtzUOuDPmq"), ("il2cpp_set_temp_dir", "dRYGcUtSbvu"), ("il2cpp_set_commandline_arguments", "rBOtokqByfg"), ("il2cpp_set_commandline_arguments_utf16", "EmyifNeOiYb"), ("il2cpp_set_config_utf16", "vevNhTRMrVU"), ("il2cpp_set_config", "aYOwnEkFYtN"), ("il2cpp_set_memory_callbacks", "aITQSuiRMdg"), ("il2cpp_memory_pool_set_region_size", "UvWKcnITQZU"), ("il2cpp_memory_pool_get_region_size", "GFMbnYXASAA"), ("il2cpp_get_corlib", "nbnMbXwAeSR"), ("il2cpp_add_internal_call", "DrNtnErTIJU"), ("il2cpp_resolve_icall", "OpTANDFfYRH"), ("il2cpp_alloc", "ZfdWJeklDe"), ("il2cpp_free", "VtcJVfxXeuv"), ("il2cpp_array_class_get", "DNiWGVnHeto"), ("il2cpp_array_length", "NCgdwIYtvXw"), ("il2cpp_array_get_byte_length", "vcFrpozvZVZ"), ("il2cpp_array_new", "gjSAHdk_pUP"), ("il2cpp_array_new_specific", "YgYWOzDkMxD"), ("il2cpp_array_new_full", "rAwQnyRn_Oh"), ("il2cpp_bounded_array_class_get", "xfIoJFUyPLu"), ("il2cpp_array_element_size", "dzQZNHyTBnV"), ("il2cpp_assembly_get_image", "FDCXgNbTIEI"), ("il2cpp_class_for_each", "bLmgsuqyGKv"), ("il2cpp_class_enum_basetype", "HAspQgJNXbQ"), ("il2cpp_class_is_inited", "XbKcoaniGPx"), ("il2cpp_class_is_generic", "AppbiebOXIj"), ("il2cpp_class_is_inflated", "mGXeadQzsLD"), ("il2cpp_class_is_assignable_from", "dBnALlfVKgZ"), ("il2cpp_class_is_subclass_of", "BTWIT_AWsSJ"), ("il2cpp_class_has_parent", "IEIIiTeWaz"), ("il2cpp_class_from_il2cpp_type", "jbeStAzaNuQ"), ("il2cpp_class_from_name", "Me_nzDMRWaX"), ("il2cpp_class_from_system_type", "KYuxxVTPWGz"), ("il2cpp_class_get_element_class", "YoWJCHCOE_r"), ("il2cpp_class_get_events", "qrrPqcgIYjo"), ("il2cpp_class_get_fields", "bEziFdPVluA"), ("il2cpp_class_get_nested_types", "csoEKYpRdKH"), ("il2cpp_class_get_interfaces", "vppOaVbHSuA"), ("il2cpp_class_get_properties", "GSFhjNNXzZm"), ("il2cpp_class_get_property_from_name", "TjLXfKMnxEm"), ("il2cpp_class_get_field_from_name", "CRMEoBjQuJM"), ("il2cpp_class_get_methods", "UobILWcgHXv"), ("il2cpp_class_get_method_from_name", "HjGghVZCbSz"), ("il2cpp_class_get_name", "fLthbBlsYJx"), ("il2cpp_type_get_name_chunked", "YBC_FQdoWpP"), ("il2cpp_class_get_namespace", "eamkwrancEH"), ("il2cpp_class_get_parent", "RVnaNXZxsEY"), ("il2cpp_class_get_declaring_type", "PCvmPxVRtpi"), ("il2cpp_class_instance_size", "DEFtWICfdoM"), ("il2cpp_class_num_fields", "PHgAUSSSEBW"), ("il2cpp_class_is_valuetype", "QozUAlpQMZR"), ("il2cpp_class_value_size", "ZKqyJAbOdTs"), ("il2cpp_class_is_blittable", "fqsKgGTTkID"), ("il2cpp_class_get_flags", "BuqeXumTJNZ"), ("il2cpp_class_is_abstract", "ZFRkJqiBOoV"), ("il2cpp_class_is_interface", "CyQioMHWhkf"), ("il2cpp_class_array_element_size", "HNLHPeWwpPW"), ("il2cpp_class_from_type", "fREIWpcotRn"), ("il2cpp_class_get_type", "bNlGSJN_imy"), ("il2cpp_class_get_type_token", "uPXdKJWkItz"), ("il2cpp_class_has_attribute", "BeNHslUtgca"), ("il2cpp_class_has_references", "dwzHkkbGqCw"), ("il2cpp_class_is_enum", "bFpwCg_hCTY"), ("il2cpp_class_get_image", "HXhSJKiJzYh"), ("il2cpp_class_get_assemblyname", "WZIbEPTkyYo"), ("il2cpp_class_get_rank", "boreiGeOMQ"), ("il2cpp_class_get_data_size", "gBhIbHARAkE"), ("il2cpp_class_get_static_field_data", "TLtS_cicAIJ"), ("il2cpp_stats_dump_to_file", "pbqZvhjhQaU"), ("il2cpp_stats_get_value", "VdTiATMiLg"), ("il2cpp_domain_get", "tbNCHnlpUrG"), ("il2cpp_domain_assembly_open", "AEWRMFIDBXa"), ("il2cpp_domain_get_assemblies", "lcoXDKExMmu"), ("il2cpp_raise_exception", "ZfFSNcHdzUW"), ("il2cpp_exception_from_name_msg", "MTxqYkvVFhK"), ("il2cpp_get_exception_argument_null", "hpGYHcvaWYk"), ("il2cpp_format_exception", "QnyVSkMoaNq"), ("il2cpp_format_stack_trace", "kTOqvnmYonU"), ("il2cpp_unhandled_exception", "GogOXrhnFgv"), ("il2cpp_native_stack_trace", "KSDT_KFWQzJ"), ("il2cpp_field_get_flags", "SOOlOEhkovu"), ("il2cpp_field_get_from_reflection", "qMgkUxsZKBL"), ("il2cpp_field_get_name", "BBvNOEOBa_j"), ("il2cpp_field_get_parent", "PNziqTSmRQk"), ("il2cpp_field_get_object", "otiasDKcjuN"), ("il2cpp_field_get_offset", "REPsBNHOtDc"), ("il2cpp_field_get_type", "AtffLxqvTvh"), ("il2cpp_field_get_value", "SjlrkMNOJ_Z"), ("il2cpp_field_get_value_object", "ukiJuGapvXc"), ("il2cpp_field_has_attribute", "ArPuTJouqUE"), ("il2cpp_field_set_value", "rWIUtJSaJga"), ("il2cpp_field_static_get_value", "ilGiSDSUHyO"), ("il2cpp_field_static_set_value", "RgQUrZYAUzn"), ("il2cpp_field_set_value_object", "IcNPWOFCiZn"), ("il2cpp_field_is_literal", "SCuYSVIVYjM"), ("il2cpp_gc_collect", "xsgHytiBkqb"), ("il2cpp_gc_collect_a_little", "tbeSslzCKRL"), ("il2cpp_gc_start_incremental_collection", "mKdvHiKPt_t"), ("il2cpp_gc_disable", "AhoYgGhOjwb"), ("il2cpp_gc_enable", "WzsP_CnZ_qk"), ("il2cpp_gc_is_disabled", "QwTsiwXggAS"), ("il2cpp_gc_set_mode", "HwZljaIQpAY"), ("il2cpp_gc_get_max_time_slice_ns", "ilWTiwTLsfZ"), ("il2cpp_gc_set_max_time_slice_ns", "qtXuNQVtUdg"), ("il2cpp_gc_is_incremental", "fSmY_HvSnDq"), ("il2cpp_gc_get_used_size", "ywHRJjYgEvY"), ("il2cpp_gc_get_heap_size", "PWazHyJIntZ"), ("il2cpp_gc_wbarrier_set_field", "UerugSEPbAU"), ("il2cpp_gc_has_strict_wbarriers", "PimVN_e_IkP"), ("il2cpp_gc_set_external_allocation_tracker", "Hh_kpw_dYDQ"), ("il2cpp_gc_set_external_wbarrier_tracker", "gWaDwIHLsAX"), ("il2cpp_gc_foreach_heap", "g_jrjdtjjvZ"), ("il2cpp_stop_gc_world", "fZZujmbNVvO"), ("il2cpp_start_gc_world", "ySBRDiFdIrd"), ("il2cpp_gc_alloc_fixed", "eDaPghQaWMH"), ("il2cpp_gc_free_fixed", "QBsInsBFdFf"), ("il2cpp_gchandle_new", "mbUTJsLLV_n"), ("il2cpp_gchandle_new_weakref", "iCoyGpPgskn"), ("il2cpp_gchandle_get_target", "QWtpeLNMFbq"), ("il2cpp_gchandle_free", "oCVCuedUxBx"), ("il2cpp_gchandle_foreach_get_target", "OlsSkFUZRqA"), ("il2cpp_object_header_size", "ckqZInkfepC"), ("il2cpp_array_object_header_size", "nmdAGoTTwAv"), ("il2cpp_offset_of_array_length_in_array_object_header", "C_GIsDoTTam"), ("il2cpp_offset_of_array_bounds_in_array_object_header", "VFHBHAluMqG"), ("il2cpp_allocation_granularity", "z_Leptxeueo"), ("il2cpp_unity_liveness_allocate_struct", "IqIhOTbuXdx"), ("il2cpp_unity_liveness_calculation_from_root", "DtnVvZcqhJM"), ("il2cpp_unity_liveness_calculation_from_statics", "istVCgREULl"), ("il2cpp_unity_liveness_finalize", "OlHAgzXQ_FL"), ("il2cpp_unity_liveness_free_struct", "xgpOUVQnljT"), ("il2cpp_method_get_return_type", "kOYxfeGxW_D"), ("il2cpp_method_get_declaring_type", "VPKqUidtuwU"), ("il2cpp_method_get_name", "VcXifIAGCqW"), ("il2cpp_method_get_from_reflection", "MnZ_JatJjKk"), ("il2cpp_method_get_object", "eXcSVjKcGGh"), ("il2cpp_method_is_generic", "QlvHQSAvCF_"), ("il2cpp_method_is_inflated", "HYJvzzNUayL"), ("il2cpp_method_is_instance", "NTMzXAhQcOg"), ("il2cpp_method_get_param_count", "BJUagMHWYFJ"), ("il2cpp_method_get_param", "CstsAJI_VDd"), ("il2cpp_method_get_class", "LocTHSsxX_m"), ("il2cpp_method_has_attribute", "QmeKzUhJxft"), ("il2cpp_method_get_flags", "prAaJIBUnOX"), ("il2cpp_method_get_token", "xGSbLNO__uT"), ("il2cpp_method_get_param_name", "OaPoZcQJcqW"), ("il2cpp_property_get_flags", "FPbukwUniCR"), ("il2cpp_property_get_get_method", "gkhhTMiNCJk"), ("il2cpp_property_get_set_method", "pCVmBYQROWq"), ("il2cpp_property_get_name", "bBbgeaPXAka"), ("il2cpp_property_get_parent", "NqEaaezpqAx"), ("il2cpp_object_get_class", "RZfWRfdextS"), ("il2cpp_object_get_size", "oOzjLD_Phoh"), ("il2cpp_object_get_virtual_method", "YnmDtXkyEAk"), ("il2cpp_object_new", "HYkenaN_ffY"), ("il2cpp_object_unbox", "dXQOarfokjJ"), ("il2cpp_value_box", "gIjILHAcdXH"), ("il2cpp_monitor_enter", "GxytmFtakk"), ("il2cpp_monitor_try_enter", "ywqoAdoNIcV"), ("il2cpp_monitor_exit", "PxIozNZXLnU"), ("il2cpp_monitor_pulse", "hOrKkxieZZX"), ("il2cpp_monitor_pulse_all", "U_atDFtQKpC"), ("il2cpp_monitor_wait", "UVks_NMUtl"), ("il2cpp_monitor_try_wait", "vEXlh_HTqQf"), ("il2cpp_runtime_invoke", "cvHaEvuKJRd"), ("il2cpp_runtime_invoke_convert_args", "xBsvWwlPKpN"), ("il2cpp_runtime_class_init", "cNGeqKT_eIi"), ("il2cpp_runtime_object_init", "dybOHBJXHog"), ("il2cpp_runtime_object_init_exception", "pIHLaPpqeLx"), ("il2cpp_runtime_unhandled_exception_policy_set", "beOPJTjeKdS"), ("il2cpp_string_length", "otIzjvGplgs"), ("il2cpp_string_chars", "RvViYMAjufK"), ("il2cpp_string_new", "nUvGbl_aHjz"), ("il2cpp_string_new_len", "zwfbJmKhSTe"), ("il2cpp_string_new_utf16", "HBvUfK_Nldw"), ("il2cpp_string_new_wrapper", "AzBsLjVeMoY"), ("il2cpp_string_intern", "vHJyOlgJwJi"), ("il2cpp_string_is_interned", "jQePKkBwzkj"), ("il2cpp_thread_current", "wyScEJcqUBF"), ("il2cpp_thread_attach", "fpBkYwTeXpE"), ("il2cpp_thread_detach", "kaPsFOvx_pl"), ("il2cpp_is_vm_thread", "PiRBJXwhgxs"), ("il2cpp_current_thread_walk_frame_stack", "mafkCTrbQHw"), ("il2cpp_thread_walk_frame_stack", "aZPzNPeYJsL"), ("il2cpp_current_thread_get_top_frame", "LaAxPJntkin"), ("il2cpp_thread_get_top_frame", "mljzTpEMelo"), ("il2cpp_current_thread_get_frame_at", "zZpWQVCNgnh"), ("il2cpp_thread_get_frame_at", "jcKFOtakxXJ"), ("il2cpp_current_thread_get_stack_depth", "KWdQeKGOHxi"), ("il2cpp_thread_get_stack_depth", "YKMNlhLfYDn"), ("il2cpp_override_stack_backtrace", "MdlqLc_Czwg"), ("il2cpp_type_get_object", "ZPQIQDbjibW"), ("il2cpp_type_get_type", "YyqhDLUeTwR"), ("il2cpp_type_get_class_or_element_class", "HTwgDksLgmq"), ("il2cpp_type_get_name", "CmeKVYFLQVJ"), ("il2cpp_type_is_byref", "nyISzrGSbmr"), ("il2cpp_type_get_attrs", "rBxcSUOcPYS"), ("il2cpp_type_equals", "hfNtLgtgLhJ"), ("il2cpp_type_get_assembly_qualified_name", "KDomJwvZHvi"), ("il2cpp_type_get_reflection_name", "vuiKgAZxFxq"), ("il2cpp_type_is_static", "dAGGbOfRaDB"), ("il2cpp_type_is_pointer_type", "vpYlbcZmZAe"), ("il2cpp_image_get_assembly", "zZeVwfOckyM"), ("il2cpp_image_get_name", "qDHPwTDfNti"), ("il2cpp_image_get_filename", "CAGtHzngrng"), ("il2cpp_image_get_entry_point", "F_xunYYaYdu"), ("il2cpp_image_get_class_count", "smPUPEnPQFa"), ("il2cpp_image_get_class", "wqIVNcReGLX"), ("il2cpp_capture_memory_snapshot", "GZtvTfH_JgR"), ("il2cpp_free_captured_memory_snapshot", "elQvchmdqcM"), ("il2cpp_set_find_plugin_callback", "TBjWKBQPrrj"), ("il2cpp_register_log_callback", "HKGcrA_ZBDS"), ("il2cpp_debugger_set_agent_options", "LIj_wyfeYx_"), ("il2cpp_is_debugger_attached", "VVrZ_CyWkxS"), ("il2cpp_register_debugger_agent_transport", "GINJmLsqqxW"), ("il2cpp_debug_foreach_method", "FpTrJBKIOkf"), ("il2cpp_debug_get_method_info", "vaJfRrkXiSz"), ("il2cpp_unity_install_unitytls_interface", "MbdxAaVNdan"), ("il2cpp_custom_attrs_from_class", "amOUsPCOvKx"), ("il2cpp_custom_attrs_from_method", "TTmWWMcwgPF"), ("il2cpp_custom_attrs_from_field", "BBMHThrNzuq"), ("il2cpp_custom_attrs_get_attr", "MtbuvotAOrN"), ("il2cpp_custom_attrs_has_attr", "QNyZMnIMgnB"), ("il2cpp_custom_attrs_construct", "tLWLmRgJuVn"), ("il2cpp_custom_attrs_free", "adRIuuSnuiS"), ("il2cpp_class_set_userdata", "iewfWsRDrnZ"), ("il2cpp_class_get_userdata_offset", "HyKHlWQjqkS"), ("il2cpp_set_default_thread_affinity", "tlgMoPPfckt"), ("il2cpp_unity_set_android_network_up_state_func", "iwrCmV_LQyH"), ];

pub fn get_value(key: &str) -> Option<&str> {
    EXPORTS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

fn get(key: &str) -> &str {
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
