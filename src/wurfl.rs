use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use thiserror::Error;

// Custom error for WURFL handle operations
#[derive(Error, Debug)]
pub struct WurflError {
    pub msg: String,
}

impl std::fmt::Display for WurflError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// This represents a condition where no error has occurred
const WURFL_OK: u32 = 0;

/// Enumeration of the cache providers supported by the WURFL Engine
pub enum WurflCacheProvider {
    /// No cache is used
    NoCache,
    /// An LRU cache provider is used
    LRU,
}

/// Enumeration of the device matching types of the WURFL API
#[derive(PartialEq, Debug)]
pub enum MatchType {
    WurflMatchTypeExact,
    WurflMatchTypeConclusive,
    WurflMatchTypeRecovery,
    WurflMatchTypeCatchall,
    WurflMatchTypeNone,
    WurflMatchTypeCached,
}

/// Enumeration of the available update frequencies for the WURFL updater process
#[derive(PartialEq, Debug)]
pub enum WurflUpdaterFrequency {
    /// Daily updates
    WurflUpdaterFrequencyDaily,
    /// Weekly updates
    WurflUpdaterFrequencyWeekly,
}

#[derive(PartialEq, Debug)]
pub enum WurflEnumType {
    WurflEnumStaticCapabilities,
    WurflEnumVirtualCapabilities,
    WurflEnumWurflID,
}

/// convenience method for get a rust str from a C char array pointer
fn to_str<'a>(char_seq: *const c_char) -> &'a str {
    let c_str = unsafe { CStr::from_ptr(char_seq) };
    return c_str.to_str().unwrap();
}

/// convenience method for get a rust String from a C char array pointer
fn to_owned_string(char_seq: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(char_seq) };
    let str_value = c_str.to_str().unwrap();
    let ret_val = String::from(str_value);
    return ret_val;
}

fn to_cache_provider(cp: WurflCacheProvider) -> wurfl_cache_provider {
    match cp {
        WurflCacheProvider::NoCache => wurfl_cache_provider_WURFL_CACHE_PROVIDER_NONE,
        WurflCacheProvider::LRU => wurfl_cache_provider_WURFL_CACHE_PROVIDER_LRU
    }
}

fn to_wurfl_enum_type(et: WurflEnumType) -> wurfl_enum_type {
    match et {
        WurflEnumType::WurflEnumStaticCapabilities => wurfl_enum_type_WURFL_ENUM_STATIC_CAPABILITIES,
        WurflEnumType::WurflEnumVirtualCapabilities => wurfl_enum_type_WURFL_ENUM_VIRTUAL_CAPABILITIES,
        WurflEnumType::WurflEnumWurflID => wurfl_enum_type_WURFL_ENUM_WURFLID
    }
}

/// WURFL holds data and exposes methods used to perform device detection and access device capabilities.
pub struct Wurfl {
    wurfl: wurfl_handle,
    _important_header_names: Vec<String>,
    // used for header ignore-case comparison and to avoid converting well-know header names every time
    // key: a lowercase header name, value: its CString conversion
    important_header_cstring_names: HashMap<String, CString>,
}

/// Implementation of the WURFL API engine. Loads the WURFL file, exposes methods to perform device detection and query
/// the detected device capabilities.
impl Wurfl {
    /// Creates the wurfl engine.
    /// Parameters :
    /// wurfl_xml : path to the wurfl.xml/zip file
    /// patches : vector of paths of patches files to load
    /// cap_filter : list of capabilities used; allow to init engine without loading all 500+ caps
    /// cache_provider : WurflCacheProviderLru or NoCache
    /// cache_extra_config : size of lru cache in the form "100000"
    pub fn new(wurfl_xml: &str, patches: Option<&[&str]>, cap_filter: Option<&[&str]>,
               cache_provider: WurflCacheProvider, cache_extra_config: Option<&str>) -> Result<Wurfl, WurflError> {
        let wh = unsafe { wurfl_create() };
        if wh.is_null() {
            return Err(WurflError { msg: "Wurfl handle is NULL".to_string() });
        }

        let c_str_path = match CString::new(wurfl_xml) {
            Ok(c) => c,
            Err(_) => return Err(WurflError { msg: "Error creating CString from wurfl xml file path".to_string() })
        };
        let c_chars_path = c_str_path.as_ptr() as *const c_char;
        // Set the root path that will be used to load the engine
        let we = unsafe { wurfl_set_root(wh, c_chars_path) };
        if we != WURFL_OK {
            let error_message = Wurfl::get_error_message(wh);
            let werr = WurflError { msg: error_message };
            return Err(werr);
        }


        // let's set the cache provider - if specified
        match cache_provider {
            WurflCacheProvider::LRU => {
                let cache_provider_val = to_cache_provider(cache_provider);

                if cache_extra_config.is_some() {
                    let u_cache_extra_config = cache_extra_config.unwrap();
                    let c_str_extra_cache_config = match CString::new(u_cache_extra_config) {
                        Ok(ecc) => ecc,
                        Err(_) => return Err(WurflError { msg: "Invalid cache_extra_config passed in create WURFL engine. Possibly a nul character in the input string".to_string() })
                    };
                    unsafe {
                        let we = wurfl_set_cache_provider(wh, cache_provider_val, c_str_extra_cache_config.as_ptr());
                        if we != WURFL_OK {
                            let err_msg = Wurfl::get_error_message(wh);
                            return Err(WurflError { msg: err_msg });
                        }
                    };
                };
            }
            _ => {}
        }

        // setting patches
        if patches.is_some() {
            let u_patches = patches.unwrap();
            for p in u_patches {
                let s_slice: &str = &p[..];
                let c_str_p = match CString::new(s_slice) {
                    Ok(cs) => cs,
                    Err(e) => {
                        let err_msg = format!("Invalid patch path passed: {}", e.to_string());
                        return Err(WurflError { msg: err_msg });
                    }
                };
                let we = unsafe { wurfl_add_patch(wh, c_str_p.as_ptr()) };
                if we != WURFL_OK {
                    let error_message = Wurfl::get_error_message(wh);
                    let werr = WurflError { msg: error_message };
                    return Err(werr);
                }
                //println!("Adding patch file : {}", p);
            }
        }


        // filter capabilities in engine
        if cap_filter.is_some() {
            let u_filter = cap_filter.unwrap();
            for cap_name in u_filter {
                let s_slice: &str = &cap_name[..];

                let c_str_cap_name = match CString::new(s_slice) {
                    Ok(cs) => cs,
                    Err(e) => {
                        let err_msg = format!("Invalid capability filter passed: {}", e.to_string());
                        return Err(WurflError { msg: err_msg });
                    }
                };
                let we = unsafe { wurfl_add_requested_capability(wh, c_str_cap_name.as_ptr()) };
                if we != WURFL_OK {
                    let error_message = Wurfl::get_error_message(wh);
                    return Err(WurflError { msg: error_message });
                }
                //println!("Adding capability : {}", cap_name);
            }
        }

        let wel = unsafe { wurfl_load(wh) };
        if wel != WURFL_OK {
            let load_error_message = Wurfl::get_error_message(wh);
            let w_load_err = WurflError { msg: load_error_message };
            return Err(w_load_err);
        }

        let mut imp_h_names = vec![];
        let mut imh_h_cstr_names = HashMap::new();

        // prepare important headers slice
        unsafe {
            let ihe = wurfl_get_important_header_enumerator(wh);
            if ihe.is_null() {
                let err_msg = Wurfl::get_error_message(wh);
                return Err(WurflError { msg: err_msg });
            }

            while wurfl_important_header_enumerator_is_valid(ihe) != 0 {
                // get the header name
                let c_header_name = wurfl_important_header_enumerator_get_value(ihe);
                // convert header name to go string
                let str_headerName = to_owned_string(c_header_name);
                // create a C string copy from the Rust String. Note that we need to clone it here because CString::new does not take references
                // and will steal ownership
                let c_str_header_name = match CString::new(str_headerName.clone()) {
                    Ok(chn) => chn,
                    Err(_) => {
                        return Err(WurflError { msg: format!("Cannot convert important header name {} to C String", str_headerName) });
                    }
                };
                // append to slice and map
                imp_h_names.push(str_headerName.clone());
                imh_h_cstr_names.insert(str_headerName.to_lowercase(), c_str_header_name);
                // advance
                wurfl_important_header_enumerator_move_next(ihe);
            }
            wurfl_important_header_enumerator_destroy(ihe);
        }

        let wurfl_engine = Wurfl {
            wurfl: wh,
            _important_header_names: imp_h_names,
            important_header_cstring_names: imh_h_cstr_names,
        };

        return Ok(wurfl_engine);
    }

    /// Returns the last error message held by the WURFL engine
    fn get_error_message(wh: wurfl_handle) -> String {
        let wu_err = unsafe { wurfl_get_error_message(wh) };
        let err_msg = to_str(wu_err);
        return err_msg.to_string();
    }

    /// Returns the current underlying WURFL API version
    pub fn get_api_version(&self) -> &str {
        let c_buf: *const c_char = unsafe { wurfl_get_api_version() };
        let api_ver = to_str(c_buf);
        return api_ver;
    }

    /// Returns the last load time of the WURFL file
    pub fn get_last_load_time(&self) -> &str {
        let llt = unsafe { wurfl_get_last_load_time_as_string(self.wurfl) };
        return to_str(llt);
    }

    /// Returns information about the running WURFL engine and loaded file
    pub fn get_info(&self) -> &str {
        let info = unsafe { wurfl_get_wurfl_info(self.wurfl) };
        return to_str(info);
    }

    /// Returns all capabilities names
    pub fn get_all_caps(&self) -> Vec<String> {
        let mut cap_names = Vec::new();
        let enum_type = to_wurfl_enum_type(WurflEnumType::WurflEnumStaticCapabilities);
        unsafe {
            let caps_enum = wurfl_enum_create(self.wurfl, enum_type);
            if caps_enum.is_null() {
                return cap_names;
            }
            while wurfl_enum_is_valid(caps_enum) == 1 {
                let cap_name_ptr = wurfl_enum_get_name(caps_enum);
                let cap_name = to_str(cap_name_ptr);
                cap_names.push(cap_name.to_string());
                wurfl_enum_move_next(caps_enum);
            }
            // once listed the capability names, destroy the enumeration created by the C lib
            wurfl_enum_destroy(caps_enum);
        }
        return cap_names;
    }

    /// Returns all this WURFL engine virtual capabilities names
    pub fn get_all_vcaps(&self) -> Vec<String> {
        let mut vcap_names = Vec::new();
        let enum_type = to_wurfl_enum_type(WurflEnumType::WurflEnumVirtualCapabilities);
        unsafe {
            let vcaps_enum = wurfl_enum_create(self.wurfl, enum_type);
            if vcaps_enum.is_null() {
                return vcap_names;
            }
            while wurfl_enum_is_valid(vcaps_enum) == 1 {
                let vcap_name_ptr = wurfl_enum_get_name(vcaps_enum);
                let vcap_name = to_str(vcap_name_ptr);
                vcap_names.push(vcap_name.to_string());
                wurfl_enum_move_next(vcaps_enum);
            }
            // once listed the capability names, destroy the enumeration created by the C lib
            wurfl_enum_destroy(vcaps_enum);
        }
        return vcap_names;
    }

    /// Retrieves device data based on the given User-Agent string
    pub fn lookup_useragent(&self, user_agent: &str) -> Result<Device, WurflError> {
        let c_str_useragent = match CString::new(user_agent) {
            Ok(ua) => ua,
            Err(_) => {
                let msg = format!("Unable to convert into a CString the User-Agent  {}", user_agent);
                return Err(WurflError { msg });
            }  // failed to convert to C string
        };
        let d_handle = unsafe { wurfl_lookup_useragent(self.wurfl, c_str_useragent.as_ptr()) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError { msg: err_msg });
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        return Result::Ok(device);
    }

    /// Retrieves device data based on HTTP request headers that can be passed in any data structures that implement the
    /// `IntoIterator` trait (for example: HashMap or Hyper framework HeaderMap.
    pub fn lookup_with_headers<U, V, T: IntoIterator<Item=(U, V)>>(&self, headers: T) -> Result<Device, WurflError> where
        U: ToString,
        V: AsRef<[u8]> {
        let cih = unsafe { wurfl_important_header_create(self.wurfl) };
        if cih.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError { msg: err_msg });
        }

        let ih_names_ref = &self.important_header_cstring_names;
        for (key, value) in headers {
            let low_key = &key.to_string().to_lowercase();
            if ih_names_ref.contains_key(low_key)
            {
                let c_header_name = self.important_header_cstring_names.get(low_key).unwrap();

                // get a &str from the generic header output
                let utf_str_res = std::str::from_utf8(value.as_ref());
                if utf_str_res.is_ok() {
                    let utf_str = utf_str_res.unwrap();
                    // create a C string from header value
                    let c_header = match CString::new(utf_str) {
                        Ok(c_h) => c_h,
                        Err(_) => {
                            // before returning error, free memory for the C important headers
                            unsafe { wurfl_important_header_destroy(cih) };
                            return Err(WurflError { msg: "Unable to convert header value to C string".to_string() });
                        }
                    };

                    // add this header to cih
                    unsafe {
                        wurfl_important_header_set(cih, c_header_name.as_ptr(), c_header.as_ptr());
                    }
                }
            }
        }

        let d_handle = unsafe { wurfl_lookup_with_important_header(self.wurfl, cih) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError { msg: err_msg });
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        unsafe { wurfl_important_header_destroy(cih) };
        return Ok(device);
    }

    /// Retrieves device data based on a WURFL device ID and HTTP request headers that can be passed in any data structures that implement the
    /// `IntoIterator` trait (for example: HashMap or Hyper framework HeaderMap.
    pub fn lookup_device_id_with_headers<U, V, T: IntoIterator<Item=(U, V)>>(&self, device_id: &str, headers: T) -> Result<Device, WurflError> where
        U: ToString,
        V: AsRef<[u8]> {

        // Get device ID
        let c_dev_id = match CString::new(device_id) {
            Ok(did) => did,
            Err(_) => {
                let msg = format!("Unable to convert device id  {}! into a CString", device_id);
                return Err(WurflError { msg });
            }  // failed to convert to C string
        };

        let cih = unsafe { wurfl_important_header_create(self.wurfl) };
        if cih.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError { msg: err_msg });
        }

        let ih_names_ref = &self.important_header_cstring_names;
        for (key, value) in headers {
            let low_key = &key.to_string().to_lowercase();
            if ih_names_ref.contains_key(low_key) {
                let c_header_name = self.important_header_cstring_names.get(low_key).unwrap();

                // get a &str from the generic header output
                let utf_str_res = std::str::from_utf8(value.as_ref());
                if utf_str_res.is_ok() {
                    let utf_str = utf_str_res.unwrap();
                    // create a C string from header value
                    let c_header = match CString::new(utf_str) {
                        Ok(c_h) => c_h,
                        Err(_) => {
                            // before returning error, free memory for the C important headers
                            unsafe { wurfl_important_header_destroy(cih) };
                            return Err(WurflError { msg: "Unable to convert header value to C string".to_string() });
                        }
                    };

                    // add this header to cih
                    unsafe {
                        wurfl_important_header_set(cih, c_header_name.as_ptr(), c_header.as_ptr());
                    }
                }
            }
        }

        let d_handle = unsafe { wurfl_get_device_with_important_header(self.wurfl, c_dev_id.as_ptr(), cih) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError { msg: err_msg });
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        unsafe { wurfl_important_header_destroy(cih) };
        return Ok(device);
    }

    /// Retrieves device data based on the WURFL device ID
    pub fn lookup_device_id(&self, device_id: &str) -> Result<Device, WurflError> {
        let c_dev_id = match CString::new(device_id) {
            Ok(did) => did,
            Err(_) => {
                let msg = format!("Unable to convert device id  {}! into a CString", device_id);
                return Err(WurflError { msg });
            }  // failed to convert to C string
        };
        let d_handle = unsafe { wurfl_get_device(self.wurfl, c_dev_id.as_ptr()) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError { msg: err_msg });
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        return Result::Ok(device);
    }

    /// Returns a vector that contains the IDs of all devices loaded from the WURFL file
    pub fn get_all_device_ids(&self) -> Vec<String> {
        let mut dev_ids = vec![];
        let enum_type = to_wurfl_enum_type(WurflEnumType::WurflEnumWurflID);
        let d_id_enum = unsafe { wurfl_enum_create(self.wurfl, enum_type) };
        if d_id_enum.is_null() {
            return dev_ids;
        }

        while unsafe { wurfl_enum_is_valid(d_id_enum) != 0 } {
            // convert device id to go string

            let id_ptr = unsafe { wurfl_enum_get_name(d_id_enum) };
            let c_str = unsafe { CStr::from_ptr(id_ptr) };
            // the string coming from get_capability should be UTF-8 compliant
            let id = match c_str.to_str() {
                Ok(conv_id) => conv_id,
                Err(_utf_err) => "" // empty, string, will be discarded later
            };

            if id.len() != 0 {
                // add this id to the slice
                dev_ids.push(id.to_string());
            }
            unsafe { wurfl_enum_move_next(d_id_enum) }
        }

        unsafe { wurfl_enum_destroy(d_id_enum) }
        return dev_ids;
    }

    /// Returns true if the WURFL contains the capability with the given name, false otherwise
    pub fn has_capability(&self, cap_name: &str) -> bool {
        let c_cap_name = match CString::new(cap_name) {
            Ok(cn) => cn,
            Err(_) => return false,  // failed to convert to C string
        };

        let has_cap = unsafe { wurfl_has_capability(self.wurfl, c_cap_name.as_ptr()) };
        if has_cap == 0i32 {
            return false;
        }
        return true;
    }

    /// Returns true if the WURFL contains the virtual capability with the given name, false otherwise
    pub fn has_virtual_capability(&self, vcap_name: &str) -> bool {
        let c_vcap_name = match CString::new(vcap_name) {
            Ok(cn) => cn,
            Err(_) => return false,  // failed to convert to C string
        };

        let has_vcap = unsafe { wurfl_has_virtual_capability(self.wurfl, c_vcap_name.as_ptr()) };
        if has_vcap == 0i32 {
            return false;
        }
        return true;
    }

    // START UPDATER METHODS ------------------------------------------------------------------------
    /// Set the URL of the WURFL file to download in the update process
    pub fn set_updater_data_url(&self, data_url: &str) -> Option<WurflError> {
        let c_url = match CString::new(data_url) {
            Ok(cu) => cu,
            Err(_) => return Some(WurflError { msg: "Unable to create C string for updater data URL".to_string() }),
        };

        let url_set = unsafe { wurfl_updater_set_data_url(self.wurfl, c_url.as_ptr()) };
        if url_set != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError { msg: err_msg });
        }
        return None;
    }

    /// Sets the interval of update checks
    pub fn set_updater_data_frequency(&self, freq: WurflUpdaterFrequency) -> Option<WurflError> {
        let c_freq = match freq {
            WurflUpdaterFrequency::WurflUpdaterFrequencyDaily => 0u32,
            WurflUpdaterFrequency::WurflUpdaterFrequencyWeekly => 1u32,
        };

        unsafe {
            let freq_set = wurfl_updater_set_data_frequency(self.wurfl, c_freq);
            if freq_set != WURFL_OK {
                let err_msg = Wurfl::get_error_message(self.wurfl);
                return Some(WurflError { msg: err_msg });
            }
        }
        return None;
    }

    /// Sets connection and data transfer timeouts (in millisecs) for updater
    /// http call. 0 for no timeout, -1 for defaults
    pub fn set_updater_data_url_timeout(&self, conn_timeout: i32, data_transfer_timeout: i32) -> Option<WurflError> {
        unsafe {
            // wurfl_error wurfl_updater_set_data_url_timeouts(wurfl_handle hwurfl, int connection_timeout, int data_transfer_timeout);
            if wurfl_updater_set_data_url_timeouts(self.wurfl, conn_timeout, data_transfer_timeout) != WURFL_OK {
                let err_msg = Wurfl::get_error_message(self.wurfl);
                return Some(WurflError { msg: err_msg });
            }
        }
        return None;
    }

    /// Sets the updater log file path
    pub fn set_updater_log_path(&self, log_file: &str) -> Option<WurflError> {
        let c_log = match CString::new(log_file) {
            Ok(cl) => cl,
            Err(_) => return Some(WurflError { msg: "Unable to create C string for log path".to_string() }),
        };
        unsafe {
            if wurfl_updater_set_log_path(self.wurfl, c_log.as_ptr()) != WURFL_OK {
                let err_msg = Wurfl::get_error_message(self.wurfl);
                return Some(WurflError { msg: err_msg });
            }
        }
        return None;
    }

    /// Start updater process once and wait for termination
    pub fn updater_runonce(&self) -> Option<WurflError> {
        if unsafe { wurfl_updater_runonce(self.wurfl) } != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError { msg: err_msg });
        }
        return None;
    }

    /// Starts periodic updater execution
    pub fn updater_start(&self) -> Option<WurflError> {
        if unsafe { wurfl_updater_start(self.wurfl) } != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError { msg: err_msg });
        }
        return None;
    }

    /// Stops updater execution
    pub fn updater_stop(&self) -> Option<WurflError> {
        if unsafe { wurfl_updater_stop(self.wurfl) } != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError { msg: err_msg });
        }
        return None;
    }

    pub fn get_important_headers(&self) -> &HashMap<String, CString> {
        return &self.important_header_cstring_names;
    }

}
// END UPDATER METHODS ------------------------------------------------------------------------

impl Drop for Wurfl {
    fn drop(&mut self) {
        //println!("Destroying... WURFL engine");
        unsafe { wurfl_destroy(self.wurfl) };
    }
}

// makes wurfl (and it internal objects) transferable between threads
unsafe impl Send for Wurfl {}

// assumes Wurfl struct thread safety, which, in our case is supported by the underlying Infuze C library
unsafe impl Sync for Wurfl {}
