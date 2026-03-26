/*
  WURFL API for Rust - Commercial Use License
  This software module is a proprietary component of ScientiaMobile's WURFL API for Rust. Access and use of this code are strictly governed by the terms of a valid commercial license agreement with ScientiaMobile, Inc.
  License Terms:

  Commercial License Required: This module can only be used by entities or individuals who have purchased and maintain an active commercial license from ScientiaMobile, Inc.
  Prohibited Use: Any use, modification, distribution, or reproduction of this module without an explicit commercial license from ScientiaMobile, Inc. is strictly prohibited.
  Compliance: Users must comply with all terms, conditions, and restrictions set forth in their specific commercial license agreement with ScientiaMobile, Inc.
  No Open Source: This module is not open source, and no rights are granted under any open source license terms.
  Enforcement: ScientiaMobile, Inc. reserves the right to enforce compliance with this license, including but not limited to, legal action against unauthorized use.

  For License Information:

  Please contact ScientiaMobile, Inc. at  https://www.scientiamobile.com/general-inquiry/

  IMPORTANT: This notice must remain intact in all copies or substantial portions of the Software.
*/
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use thiserror::Error;
use crate::HeaderQuality::{HeaderQualityBasic, HeaderQualityFull, HeaderQualityNone};
use crate::WurflAttr::WurflAttrExtraHeadersExperimental;

/// Macro to handle WURFL return codes and convert them to Results
#[macro_export]
macro_rules! check_wurfl {
    ($error_code:expr, $handle:expr) => {{
        let code = $error_code;
        if code != $crate::WURFL_OK {
            Err($crate::WurflError::new($crate::Wurfl::get_error_message($handle)))
        } else {
            Ok(())
        }
    }};
}

/// Custom error for WURFL handle operations
#[derive(Error, Debug)]
pub struct WurflError {
    pub msg: String,
}

impl std::fmt::Display for WurflError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl WurflError {
    pub fn new(msg: impl Into<String>) -> Self {
        WurflError { msg: msg.into() }
    }
}


/// This represents a condition where no error has occurred
const WURFL_OK: u32 = wurfl_error_WURFL_OK;

/// Version of the Rust binding for infuze libwurfl
const VERSION: &str = "1.8.0";


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

#[derive(PartialEq, Clone, Copy)]
pub enum WurflAttr {
    #[deprecated]
    WurflAttrExtraHeadersExperimental
}

#[derive(PartialEq, Debug)]
pub enum HeaderQuality {
    HeaderQualityNone,
    HeaderQualityBasic,
    HeaderQualityFull,
}

impl HeaderQuality {
    pub fn as_str(&self) -> &'static str {
        match self {
            HeaderQualityNone => "None",
            HeaderQualityBasic => "Basic",
            HeaderQualityFull => "Full"
        }
    }
}

/// convenience method for get a rust String from a C char array pointer
fn to_owned_string(char_seq: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(char_seq) };
    let str_value = c_str.to_str().unwrap();
    let ret_val = String::from(str_value);
    ret_val
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

pub struct StaticCapabilitiesIterator<'a> {
    device: &'a Device,
    cap_names: Vec<String>,
    index: usize,
}

impl StaticCapabilitiesIterator<'_> {
    pub fn new(device: &Device) -> StaticCapabilitiesIterator {
        let cap_names = _get_all_caps(&device.wurfl);
        return StaticCapabilitiesIterator {
            device,
            cap_names,
            index: 0,
        };
    }
}

impl Iterator for StaticCapabilitiesIterator<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.cap_names.len() {
            let cap_name = &self.cap_names[self.index];
            self.index += 1;

            let cap_value_res = self.device.get_static_cap(cap_name.as_str());
            let str_name = cap_name.clone();

            match cap_value_res {
                Ok(Some(val)) => return Some((str_name, val.to_string())),
                Ok(None) => return Some((str_name, String::new())),
                Err(_) => continue, 
            }
        }
        None
    }
}


pub struct VirtualCapabilitiesIterator<'a> {
    device: &'a Device,
    vcap_names: Vec<String>,
    index: usize,
}

impl VirtualCapabilitiesIterator<'_> {
    pub fn new(device: &Device) -> VirtualCapabilitiesIterator {
        VirtualCapabilitiesIterator {
            device,
            vcap_names: _get_all_vcaps(&device.wurfl),
            index: 0,
        }
    }
}

impl Iterator for VirtualCapabilitiesIterator<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.vcap_names.len() {
            let vcap_name = &self.vcap_names[self.index];
            self.index += 1;

            let vcap_value_res = self.device.get_virtual_cap(vcap_name.as_str());

            let str_name = vcap_name.clone();

            match vcap_value_res {
                Ok(Some(val)) => return Some((str_name, val.to_string())),
                Ok(None) => return Some((str_name, String::new())),
                Err(_) => continue, 
            }
        }
        None
    }
}

/// Wurfl holds data and exposes methods used to perform device detection and access device capabilities.
pub struct Wurfl {
    wurfl: wurfl_handle,
    _important_header_names: Vec<String>,
}

/// The Wurfl implementation represents an instance of the WURFL API engine.
    ///    This struct provides an interface to the WURFL API, allowing you to perform device detection
    ///    using the HTTP headers or a user-agent string, and access various WURFL-related information and capabilities.
    /// The class supports the following features:
    /// - Loading WURFL data from a file
    /// - Applying WURFL patches (a WURFL patch is a set of updates to the WURFL file that can be used to extend
    ///  or modify the WURFL static capabilities or to override virtual capabilities values)
    /// - Caching device detection results
    /// - Updating the WURFL data file automatically
    /// - Retrieving device information
    /// The struct also provides methods for managing the WURFL API engine,
    /// such as setting and retrieving generic attributes, getting version information.
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
            return Err(WurflError::new("Wurfl handle is NULL".to_string()));
        }

        let c_str_path = match CString::new(wurfl_xml) {
            Ok(c) => c,
            Err(_) => return Err(WurflError::new("Error creating CString from wurfl xml file path".to_string()))
        };
        let c_chars_path = c_str_path.as_ptr() as *const c_char;
        // Set the root path that will be used to load the engine
        check_wurfl!(unsafe { wurfl_set_root(wh, c_chars_path) }, wh)?;


        // let's set the cache provider - if specified
        match cache_provider {
            WurflCacheProvider::LRU => {
                let cache_provider_val = to_cache_provider(cache_provider);

                if cache_extra_config.is_some() {
                    let u_cache_extra_config = cache_extra_config.unwrap();
                    let c_str_extra_cache_config = CString::new(u_cache_extra_config).map_err(|_| WurflError::new(
                        "Invalid cache_extra_config passed in create WURFL engine. Possibly a nul character in the input string".to_string()))?;
                    check_wurfl!(unsafe { wurfl_set_cache_provider(wh, cache_provider_val, c_str_extra_cache_config.as_ptr()) }, wh)?;
                };
            }
            _ => {}
        }

        if patches.is_some() {
            let u_patches = patches.unwrap();
            for p in u_patches {
                let s_slice: &str = &p[..];
                let c_str_p = CString::new(s_slice).map_err(|e| {
                    let err_msg = format!("Invalid patch path passed: {}", e.to_string());
                    WurflError::new(err_msg)
                })?;
                check_wurfl!(unsafe { wurfl_add_patch(wh, c_str_p.as_ptr()) }, wh)?;
            }
        }

        // filter capabilities in engine
        if cap_filter.is_some() {
            let u_filter = cap_filter.unwrap();
            for cap_name in u_filter {
                let s_slice: &str = &cap_name[..];

                let c_str_cap_name = CString::new(s_slice).map_err(|e| {
                    let err_msg = format!("Invalid capability filter passed: {}", e.to_string());
                    WurflError::new(err_msg)
                })?;
                check_wurfl!(unsafe { wurfl_add_requested_capability(wh, c_str_cap_name.as_ptr()) }, wh)?;
            }
        }

        check_wurfl!(unsafe { wurfl_load(wh) }, wh)?;

        let mut imp_h_names = vec![];
        let mut imh_h_cstr_names = HashMap::new();

        // prepare important headers slice
        unsafe {
            let ihe = wurfl_get_important_header_enumerator(wh);
            if ihe.is_null() {
                let err_msg = Wurfl::get_error_message(wh);
                return Err(WurflError::new(err_msg ));
            }

            while wurfl_important_header_enumerator_is_valid(ihe) != 0 {
                // get the header name
                let c_header_name = wurfl_important_header_enumerator_get_value(ihe);
                // convert header name to go string
                let str_header_name = to_owned_string(c_header_name);
                // create a C string copy from the Rust String. Note that we need to clone it here because CString::new does not take references
                // and will steal ownership
                let c_str_header_name = CString::new(str_header_name.clone()).map_err(|_| {
                    WurflError::new(format!("Cannot convert important header name {} to C String", str_header_name))
                })?;
                // append to slice and map
                imp_h_names.push(str_header_name.clone());
                imh_h_cstr_names.insert(str_header_name.to_lowercase(), c_str_header_name);
                // advance
                wurfl_important_header_enumerator_move_next(ihe);
            }
            wurfl_important_header_enumerator_destroy(ihe);
        }

        let wurfl_engine = Wurfl {
            wurfl: wh,
            _important_header_names: imp_h_names,
        };

        Ok(wurfl_engine)
    }

    /// Returns the last error message held by the WURFL engine
    fn get_error_message(wh: wurfl_handle) -> String {
        let wu_err_ptr = unsafe { wurfl_get_error_message(wh) };
        let wu_err = unsafe { CStr::from_ptr(wu_err_ptr) };
        let err_msg = wu_err.to_str().unwrap_or("Unknown error");
        err_msg.to_string()
    }

    /// Returns the current underlying WURFL API version
    pub fn get_api_version(&self) -> &str {
        let c_buf: *const c_char = unsafe { wurfl_get_api_version() };
        let api_ver = unsafe { CStr::from_ptr(c_buf) };
        api_ver.to_str().unwrap_or("Unknown version")
    }

    /// Sets a value for a WURFL engine attribute. Possible attributes are listed under the
    /// WurflAttr enum. As of current version, the only available engine attribute
    /// is WurflAttrExtraHeadersExperimental, which is deprecated
    pub fn set_attr(&self, attr: WurflAttr, attr_value: i32) -> Option<WurflError> {

        // the only possible value for the attribute right now is 0 (= extra headers experimental)
        let mut c_wurfl_attr: wurfl_attr = 0;
        if attr == WurflAttrExtraHeadersExperimental {
            c_wurfl_attr = wurfl_attr_WURFL_ATTR_EXTRA_HEADERS_EXPERIMENTAL;
        }

        let c_attr_val = attr_value as c_int;
        let w_err: wurfl_error = unsafe { wurfl_set_attr(self.wurfl, c_wurfl_attr, c_attr_val) };
        if w_err != WURFL_OK {
            let error_message = Wurfl::get_error_message(self.wurfl);
            let wu_err = WurflError::new(error_message);
            return Some(wu_err);
        }
        None
    }

    /// Gets the value for a WURFL engine attribute. Possible attributes are listed under the
    /// WurflAttr enum. As of current version, the only available engine attribute
    /// is WurflAttrExtraHeadersExperimental
    pub fn get_attr(&self, attr: WurflAttr) -> Result<i32, WurflError> {
        let mut val: c_int = -1;
        let attr_value = &mut val as *mut c_int;
        check_wurfl!(unsafe { wurfl_get_attr(self.wurfl, attr as c_uint, attr_value) }, self.wurfl)?;

        let ci_value = unsafe { *attr_value };
        Ok(ci_value)
    }

    /// Returns the last load time of the WURFL file
    #[deprecated(since = "1.8.0", note = "Use last_load_time() instead, which returns an owned String")]
    pub fn get_last_load_time(&self) -> &str {
        let llt_ptr = unsafe { wurfl_get_last_load_time_as_string(self.wurfl) };
        let llt = unsafe { CStr::from_ptr(llt_ptr) };
        llt.to_str().unwrap_or("")
    }

    /// Returns information about the running WURFL engine and loaded file
    #[deprecated(since = "1.8.0", note = "Use info() instead, which returns an owned String")]
    pub fn get_info(&self) -> &str {
        let info_ptr = unsafe { wurfl_get_wurfl_info(self.wurfl) };
        let info = unsafe { CStr::from_ptr(info_ptr) };
        info.to_str().unwrap_or("")
    }

    /// Returns the last load time of the WURFL file.
    ///
    /// This method returns an owned `String`.
    pub fn last_load_time(&self) -> String {
        let llt_ptr = unsafe { wurfl_get_last_load_time_as_string(self.wurfl) };
        let llt = unsafe { CStr::from_ptr(llt_ptr) };
        llt.to_str().unwrap_or("").to_string()
    }

    /// Returns information about the running WURFL engine and loaded file.
    ///
    /// This method returns an owned `String`.
    pub fn info(&self) -> String {
        let info_ptr = unsafe { wurfl_get_wurfl_info(self.wurfl) };
        let info = unsafe { CStr::from_ptr(info_ptr) };
        info.to_str().unwrap_or("").to_string()
    }

    /// Returns all capabilities names
    pub fn get_all_caps(&self) -> Vec<String> {
        _get_all_caps(&self.wurfl)
    }

    /// Returns all this WURFL engine virtual capabilities names
    pub fn get_all_vcaps(&self) -> Vec<String> {
        _get_all_vcaps(&self.wurfl)
    }

    /// Retrieves device data based on the given User-Agent string.
    ///
    /// This method creates a new WURFL device handle by looking up the device data using the provided User-Agent string.
    ///
    /// # Arguments
    /// * `user_agent` - The User-Agent string to use for the device lookup.
    ///
    /// # Returns
    /// A `Result` containing the `Device` object if the lookup was successful, or a `WurflError` if there was a problem.
    pub fn lookup_useragent(&self, user_agent: &str) -> Result<Device, WurflError> {
        let c_str_useragent = CString::new(user_agent).map_err(|_| {
            let msg = format!("Unable to convert the User-Agent into a CString  {}", user_agent);
            WurflError::new(msg)
        })?;  // failed to convert to C string
        let d_handle = unsafe { wurfl_lookup_useragent(self.wurfl, c_str_useragent.as_ptr()) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError::new(err_msg));
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        Ok(device)
    }


    /// Retrieves device data based on HTTP request headers that can be passed in any data structures that implement the
    /// `IntoIterator` trait (for example: HashMap or Hyper framework HeaderMap).
    ///
    /// This method creates a new WURFL important header handle, sets the provided headers on it, and then uses that handle
    /// to look up the device data. The important header handle is destroyed before returning the `Device` object.
    ///
    /// # Arguments
    /// * `headers` - An `IntoIterator` that provides the HTTP request headers to use for the device lookup.
    ///
    /// # Returns
    /// A `Result` containing the `Device` object if the lookup was successful, or a `WurflError` if there was a problem.
    pub fn lookup_with_headers<U, V, T: IntoIterator<Item=(U, V)>>(&self, headers: T) -> Result<Device, WurflError> where
        U: ToString,
        V: AsRef<[u8]> {
        let cih = unsafe { wurfl_important_header_create(self.wurfl) };
        if cih.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError::new(err_msg ));
        }

        let ok_opt = self.internal_set_important_headers(headers, cih);
        if ok_opt.is_some(){
            // destroy important header handle before returning error
            unsafe {wurfl_important_header_destroy(cih)}
            return Err(ok_opt.unwrap());
        }

        let d_handle = unsafe { wurfl_lookup_with_important_header(self.wurfl, cih) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError::new(err_msg ));
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        unsafe { wurfl_important_header_destroy(cih) };
        Ok(device)
    }

   
    /// Retrieves device data based on the device's WURFL ID and HTTP request headers that can be passed in any data structures that implement the
    /// `IntoIterator` trait (for example: HashMap or Hyper framework HeaderMap).
    ///
    /// This method creates a new WURFL important header handle, sets the provided headers on it, and then uses that handle
    /// to look up the device data with the given device's WURFL ID. The important header handle is destroyed before returning the `Device` object.
    ///
    /// # Arguments
    /// * `device_id` - The device's WURFL ID to look up.
    /// * `headers` - An `IntoIterator` that provides the HTTP request headers to use for the device lookup.
    ///
    /// # Returns
    /// A `Result` containing the `Device` object if the lookup was successful, or a `WurflError` if there was a problem.
    pub fn lookup_device_id_with_headers<U, V, T: IntoIterator<Item=(U, V)>>(&self, wurfl_id: &str, headers: T) -> Result<Device, WurflError> where
        U: ToString,
        V: AsRef<[u8]> {

        // Get the device's WURFL ID
        let c_dev_id = CString::new(wurfl_id).map_err(|_| {
            let msg = format!("Unable to convert wurfl id  {}! into a CString", wurfl_id);
            WurflError::new(msg)
        })?;  // failed to convert to C string

        let cih = unsafe { wurfl_important_header_create(self.wurfl) };
        if cih.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError::new(err_msg ));
        }

        let ok_opt = self.internal_set_important_headers(headers, cih);
        if ok_opt.is_some(){
            // destroy important header handle before returning error
            unsafe {wurfl_important_header_destroy(cih)}
            return Err(ok_opt.unwrap());
        }

        let d_handle = unsafe { wurfl_get_device_with_important_header(self.wurfl, c_dev_id.as_ptr(), cih) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError::new(err_msg));
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        unsafe { wurfl_important_header_destroy(cih) };
        Ok(device)
    }

    /// Retrieves a WURFL device's data based on the device's  WURFL ID
    ///
    /// # Arguments
    /// * `device_id` - The device's WURFL ID to look up.
    ///
    /// # Returns
    /// A `Result` containing the `Device` object if the lookup was successful, or a `WurflError` if there was a problem.
    pub fn lookup_device_id(&self, device_id: &str) -> Result<Device, WurflError> {
        let c_dev_id = match CString::new(device_id) {
            Ok(did) => did,
            Err(_) => {
                let msg = format!("Unable to convert wurfl id  {}! into a CString", device_id);
                return Err(WurflError::new(msg));
            }  // failed to convert to C string
        };
        let d_handle = unsafe { wurfl_get_device(self.wurfl, c_dev_id.as_ptr()) };
        if d_handle.is_null() {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Err(WurflError::new(err_msg));
        }
        let device = Device {
            wurfl: self.wurfl,
            device: d_handle,
        };
        Ok(device)
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
            // convert wurfl id to go string

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
        dev_ids
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
        true
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
        true
    }

    // Client-Hints and User-Agent frozen-ness functions
    /// Returns true if user-agent string has been frozen (it has been made immutable in a certain
    ///         version of a browser/app and will never change anymore), false otherwise.
    pub fn is_ua_frozen(&self, user_agent: &str) -> bool {
        let c_user_agent = match CString::new(user_agent) {
            Ok(c_ua) => c_ua,
            Err(_) => return false,  // failed to convert to C string
        };

        let is_ua_frozen = unsafe { wurfl_is_ua_frozen(self.wurfl, c_user_agent.as_ptr()) };
        is_ua_frozen == 1i32
    }

    /// Evaluates the quality of the provided headers based on the WURFL library's understanding of the
    /// Client-Hints and User-Agent headers.
    ///
    /// This function creates a WURFL important header handle, sets the provided headers on it, and then
    /// tries to determine the quality of the headers. The quality is returned as a
    /// `HeaderQuality` enum value. Possible values are: None, Basic and  Full.
    ///
    /// # Arguments
    /// * `headers` - An iterator of key-value pairs representing the headers to evaluate.
    ///
    /// # Returns
    /// A `HeaderQuality` enum value indicating the quality of the provided headers.
    pub fn header_quality<U, V, T: IntoIterator<Item=(U, V)>>(&self, headers: T) -> HeaderQuality where
        U: ToString,
        V: AsRef<[u8]> {
        let cih = unsafe { wurfl_important_header_create(self.wurfl) };
        if cih.is_null() {
            return HeaderQualityNone;
        }

        let ok_opt = self.internal_set_important_headers(headers, cih);
        if ok_opt.is_some(){
            return HeaderQualityNone;
        }

        let quality = unsafe { wurfl_important_header_uach_quality(cih) };
        let header_quality = match quality {
            wurfl_enum_uach_type_WURFL_ENUM_UACH_BASIC => HeaderQualityBasic,
            wurfl_enum_uach_type_WURFL_ENUM_UACH_FULL => HeaderQualityFull,
            _ => HeaderQualityNone
        };
        header_quality
    }

    fn internal_set_important_headers<U, V, T: IntoIterator<Item=(U, V)>>(&self, headers: T, cih: wurfl_important_header_handle) -> Option<WurflError> where
        U: ToString,
        V: AsRef<[u8]> {

        for (key, value) in headers {
            let header_name = key.to_string();
            let c_header_name;

            // create a C string from header name
            c_header_name = match CString::new(header_name) {
                Ok(c_h) => c_h,
                Err(_) => return Some(WurflError::new("Error creating CString from header name".to_string()))
            };

            // get a &str from the generic header output
            let utf_str_res = std::str::from_utf8(value.as_ref());
            if utf_str_res.is_ok() {
                let utf_str = utf_str_res.unwrap();
                // create a C string from header value
                match CString::new(utf_str) {
                    Ok(c_h_value) => {
                        // set important header to handle
                        unsafe {
                            wurfl_important_header_set(cih, c_header_name.as_ptr(), c_h_value.as_ptr());
                        }
                    },
                    Err(_) => return Some(WurflError::new("Error creating CString from header value".to_string()))
                };
            }
        }
        return None
    }

    // START UPDATER METHODS ------------------------------------------------------------------------
    /// Set the URL of the WURFL file to download in the update process
    pub fn set_updater_data_url(&self, data_url: &str) -> Option<WurflError> {

        let api_version = self.get_api_version();
        let version = Version::parse(api_version).unwrap();
        let refVersion = Version::parse("1.13.0.0").unwrap();


        // only set the custom user-agent for versions >= 1.13.0.0 (with older version we'll end up overwriting the base user-agent)
        if version >= refVersion {
            let base_api = "infuze_rust/";
            let ua = format!("{}{}", base_api, VERSION);
            let c_ua = CString::new(ua).unwrap_or_else(|_| CString::new("infuze_rust/unknown").unwrap());
            unsafe { wurfl_updater_set_useragent(self.wurfl, c_ua.as_ptr()) };
        }

        let c_url = match CString::new(data_url) {
            Ok(cu) => cu,
            Err(_) => return Some(WurflError::new("Unable to create C string for updater data URL".to_string())),
        };

        let url_set = unsafe { wurfl_updater_set_data_url(self.wurfl, c_url.as_ptr()) };
        if url_set != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError::new(err_msg));
        }
        None
    }

    
    /// This function sets the frequency at which the updater process will check for updates to the WURFL data.
    ///
    /// # Arguments
    /// * `freq` - The frequency to set, either `WurflUpdaterFrequency::WurflUpdaterFrequencyDaily` or `WurflUpdaterFrequency::WurflUpdaterFrequencyWeekly`.
    ///
    /// # Returns
    /// * `Option<WurflError>` - `None` if the frequency was set successfully, `Some(WurflError)` if there was an error.
    pub fn set_updater_data_frequency(&self, freq: WurflUpdaterFrequency) -> Option<WurflError> {
        let c_freq = match freq {
            WurflUpdaterFrequency::WurflUpdaterFrequencyDaily => wurfl_updater_frequency_WURFL_UPDATER_FREQ_DAILY,
            WurflUpdaterFrequency::WurflUpdaterFrequencyWeekly => wurfl_updater_frequency_WURFL_UPDATER_FREQ_WEEKLY,
        };

        unsafe {
            let freq_set = wurfl_updater_set_data_frequency(self.wurfl, c_freq);
            if freq_set != WURFL_OK {
                let err_msg = Wurfl::get_error_message(self.wurfl);
                return Some(WurflError::new(err_msg));
            }
        }
        None
    }

    /// Sets connection and data transfer timeouts (in millisecs) for updater
    /// http call. 0 for no timeout, -1 for defaults
    pub fn set_updater_data_url_timeout(&self, conn_timeout: i32, data_transfer_timeout: i32) -> Option<WurflError> {
        unsafe {
            // wurfl_error wurfl_updater_set_data_url_timeouts(wurfl_handle hwurfl, int connection_timeout, int data_transfer_timeout);
            if wurfl_updater_set_data_url_timeouts(self.wurfl, conn_timeout, data_transfer_timeout) != WURFL_OK {
                let err_msg = Wurfl::get_error_message(self.wurfl);
                return Some(WurflError::new(err_msg));
            }
        }
        None
    }

    /// Sets the updater log file path
    pub fn set_updater_log_path(&self, log_file: &str) -> Option<WurflError> {
        let c_log = match CString::new(log_file) {
            Ok(cl) => cl,
            Err(_) => return Some(WurflError::new("Unable to create C string for log path".to_string())),
        };
        unsafe {
            if wurfl_updater_set_log_path(self.wurfl, c_log.as_ptr()) != WURFL_OK {
                let err_msg = Wurfl::get_error_message(self.wurfl);
                return Some(WurflError::new(err_msg));
            }
        }
        None
    }

    /// Starts the updater process once and waits for it to complete.
    ///
    /// # Returns
    /// * `Option<WurflError>` - `None` if the updater process started successfully, `Some(WurflError)` if there was an error.
    pub fn updater_runonce(&self) -> Option<WurflError> {
    
        if unsafe { wurfl_updater_runonce(self.wurfl) } != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError::new(err_msg));
        }
        None
    }

    /// Starts periodic updater execution
    ///
    /// # Returns
    /// * `Option<WurflError>` - `None` if the updater process started successfully, `Some(WurflError)` if there was an error.
    pub fn updater_start(&self) -> Option<WurflError> {
    
        if unsafe { wurfl_updater_start(self.wurfl) } != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError ::new(err_msg));
        }
        None
    }

    /// Stops the updater execution.
    ///
    /// # Returns
    /// * `Option<WurflError>` - `None` if the updater process stopped successfully, `Some(WurflError)` if there was an error.
    pub fn updater_stop(&self) -> Option<WurflError> {
        if unsafe { wurfl_updater_stop(self.wurfl) } != WURFL_OK {
            let err_msg = Wurfl::get_error_message(self.wurfl);
            return Some(WurflError::new(err_msg));
        }
        None
    }
}

/// Gets a list of all static capabilities supported by the WURFL engine.
///
/// # Arguments
/// * `wh` - A reference to the WURFL handle.
///
/// # Returns
/// A `Vec<String>` containing the names of all static capabilities.
fn _get_all_caps(wh: &wurfl_handle) -> Vec<String> {
    let mut cap_names = Vec::new();
    let enum_type = to_wurfl_enum_type(WurflEnumType::WurflEnumStaticCapabilities);
    unsafe {
        let caps_enum = wurfl_enum_create(*wh, enum_type);

        if !caps_enum.is_null() {
            while wurfl_enum_is_valid(caps_enum) == 1 {
                let cap_name_ptr = wurfl_enum_get_name(caps_enum);
                let cap_name = CStr::from_ptr(cap_name_ptr); 
                cap_names.push(cap_name.to_str().unwrap_or("").to_string());
                wurfl_enum_move_next(caps_enum);
            }
            // once listed the capability names, destroy the enumeration created by the C lib
            wurfl_enum_destroy(caps_enum);
        }
    }
    cap_names
}

/// Gets a list of all virtual capabilities supported by the WURFL engine.
///
/// # Arguments
/// * `wh` - A reference to the WURFL handle.
///
/// # Returns
/// A `Vec<String>` containing the names of all virtual capabilities.
fn _get_all_vcaps(wh: &wurfl_handle) -> Vec<String> {
    let mut vcap_names = Vec::new();
    let enum_type = to_wurfl_enum_type(WurflEnumType::WurflEnumVirtualCapabilities);
    unsafe {
        let vcaps_enum = wurfl_enum_create(*wh, enum_type);
        if vcaps_enum.is_null() {
            return vcap_names;
        }
        while wurfl_enum_is_valid(vcaps_enum) == 1 {
            let vcap_name_ptr = wurfl_enum_get_name(vcaps_enum);
            let vcap_name =  CStr::from_ptr(vcap_name_ptr);
            vcap_names.push(vcap_name.to_str().unwrap_or("").to_string());
            wurfl_enum_move_next(vcaps_enum);
        }
        // once listed the capability names, destroy the enumeration created by the C lib
        wurfl_enum_destroy(vcaps_enum);
    }
    vcap_names
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

/// Downloads the WURFL data file from the specified URL and saves it to the specified destination.
///
/// # Arguments
/// * `url` - The URL of the WURFL data file to download.
/// * `destination` - The path where the downloaded WURFL data file should be saved. Destination must be writable.
///
/// # Returns
/// * `Ok(())` if the download was successful.
/// * `Err(WurflError)` if there was an error downloading or saving the file.
pub fn wurfl_download(url: &str, destination: &str) -> Result<(), WurflError> {
    let c_url = CString::new(url).map_err(|_| WurflError::new("Invalid URL".to_string()))?;
    let c_destination = CString::new(destination).map_err(|_| WurflError::new("Invalid destination path".to_string()))?;

    let result = unsafe {
        wurfl_sys::wurfl_download(c_url.as_ptr(), c_destination.as_ptr())
    };

    if result == wurfl_sys::wurfl_error_WURFL_OK {
        Ok(())
    } else {
        let error_message = unsafe {
            let error_ptr = wurfl_sys::wurfl_get_error_string(result);
            CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
        };
        Err(WurflError::new(error_message))
    }
}

// -------- COMPARE VERSIONS METHOD --------------------------------------------------------------
/// A version number, modeled as a struct with four fields: major, minor, patch, and build.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    build: u32,
}

impl Version {
    pub fn parse(version: &str) -> Option<Version> {
        let parts: Vec<_> = version.split('.').collect();
        if parts.len() != 4 {
            return None;
        }

        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        let build = parts[3].parse().ok()?;

        Some(Version { major, minor, patch, build })
    }
}

    // -------- END COMPARE VERSIONS METHOD ----------------------------------------------------------
