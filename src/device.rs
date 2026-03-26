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

/// Device provides access to device static and virtual capabilities
pub struct Device {
    // we use this attribute to silence the "variable is never read" warning
    #[allow(dead_code)]
    wurfl: wurfl_handle,
    device: wurfl_device_handle,
}

///  This Device implementation provides methods to retrieve various capabilities and information about a device,
/// such as static capabilities, virtual capabilities, user agent strings, wurfl IDs,
/// and information about the device's position in the WURFL hierarchy.
/// Device class is designed to be used in conjunction with the Wurfl class, which provides
///  methods for looking up devices and managing the WURFL data.
/// 
/// Note: Device is not designed to be constructed directly. Instead, its instances should be obtained through the Wurfl lookup() methods.
impl Device {
    /// returns the WURFL device unique ID
    pub fn get_device_id(&self) -> &str {
        let c_d_id_ptr = unsafe { wurfl_device_get_id(self.device) };
        let d_id_str = unsafe { CStr::from_ptr(c_d_id_ptr) };
        d_id_str.to_str().unwrap_or("")
    }

    /// returns the WURFL device parent unique ID
    pub fn get_parent_id(&self) -> &str {
        let parent_id_ptr = unsafe { wurfl_device_get_parent_id(self.device) };
        let parent_id_str = unsafe { CStr::from_ptr(parent_id_ptr) };
        parent_id_str.to_str().unwrap_or("")
    }

    #[deprecated(since="1.3.0", note="please use `get_root_id` instead. From 1.7.0 returns empty string")]
    pub fn get_root_ID(&self) -> &str {
        ""
    }

    /// Returns the unique ID of this device ancestor in the WURFL hierarchy
    pub fn get_root_id(&self) -> &str {
        let d_root_id_ptr = unsafe { wurfl_device_get_root_id(self.device) };
        let d_root_id_str = unsafe { CStr::from_ptr(d_root_id_ptr) };
        d_root_id_str.to_str().unwrap_or("")
    }

    /// Returns true if this device is a root device in the WURFL hierarchy
    pub fn is_root(&self) -> bool {
        unsafe { wurfl_device_is_actual_device_root(self.device) != 0 }
    }

    /// Returns the original UserAgent of matched device (the one passed to lookup)
    pub fn get_original_user_agent(&self) -> &str {
        let d_orig_ua_ptr = unsafe { wurfl_device_get_original_useragent(self.device) };
        let d_orig_ua = unsafe { CStr::from_ptr(d_orig_ua_ptr) };
        d_orig_ua.to_str().unwrap_or("")
    }

    /// Return a normalized version of this device User-Agent
    ///
    /// only for internal use / tooling, don't use in applications
    pub fn get_normalized_user_agent(&self) -> &str {
        let d_norm_ua_ptr = unsafe { wurfl_device_get_normalized_useragent(self.device) };
        let d_norm_ua = unsafe { CStr::from_ptr(d_norm_ua_ptr) };
        d_norm_ua.to_str().unwrap_or("")
    }

    /// Returns default UserAgent of matched device (might be different from UA passed to lookup)
    pub fn get_user_agent(&self) -> &str {
        let d_ua_ptr = unsafe { wurfl_device_get_useragent(self.device) };
        let d_ua = unsafe { CStr::from_ptr(d_ua_ptr) };
        d_ua.to_str().unwrap_or("")
        
    }

    /// Returns the value of the static capability with the given name.
    ///
    /// # Arguments
    /// * `capability_name` - The name of the static capability to retrieve.
    ///
    /// # Returns
    /// A `Result` containing an `Option` of a string representing the value of the static capability, or a `WurflError`
    /// if any errors occur during the retrieval process.
    /// Errors are mainly caused by wrong capability names passed in the input.
    pub fn get_static_cap(&self, capability_name: &str) -> Result<Option<&str>, WurflError> {
        let c_str_cap_name = CString::new(capability_name).map_err(|_| {
            WurflError::new(format!(
                "Unable to convert capability_name '{}' into a CString",
                capability_name
            ))
        })?;
        let mut wurfl_error: u32 = 0;
        let cap_value_ptr = unsafe { wurfl_device_get_static_cap(
            self.device, 
            c_str_cap_name.as_ptr(), 
            &mut wurfl_error
        ) };
        
        check_wurfl!(wurfl_error, self.wurfl)?;

        if cap_value_ptr.is_null() {
            return Ok(None);
        }
        let cap_value = unsafe { CStr::from_ptr(cap_value_ptr) };
        Ok(Some(cap_value.to_str().unwrap_or("")))
    }

    /// Returns the device value of the capability with the given name
    #[deprecated(since="1.5.0", note="please use `get_static_cap` instead")]
    pub fn get_capability(&self, capability_name: &str) -> Option<&str> {
        let c_str_cap_name = match CString::new(capability_name) {
            Ok(cn) => cn,
            Err(_) => return None
        };
        let cap_value_ptr = unsafe { wurfl_device_get_capability(self.device, c_str_cap_name.as_ptr()) };
        if cap_value_ptr.is_null() {
            return None;
        }
        let cap_value = unsafe { CStr::from_ptr(cap_value_ptr) };
        Some(cap_value.to_str().unwrap_or(""))
    }

    #[deprecated(since="1.5.0", note="please use `get_virtual_cap` instead")]
    pub fn get_virtual_capability(&self, virtual_capability_name: &str) -> Option<&str> {
        let c_str_vcap_name = match CString::new(virtual_capability_name) {
            Ok(cn) => cn,
            Err(_) => return None
        };
        let vcap_value_ptr = unsafe { wurfl_device_get_virtual_capability(self.device, c_str_vcap_name.as_ptr()) };
        if vcap_value_ptr.is_null() {
            return None;
        }
        let vcap_value = unsafe { CStr::from_ptr(vcap_value_ptr) };
        Some(vcap_value.to_str().unwrap_or(""))
    }

    /// Returns the value of the virtual capability with the given name.
    ///
    /// # Arguments
    /// * `vcap_name` - The name of the virtual capability to retrieve.
    ///
    /// # Returns
    /// A `Result` containing an `Option` of a string representing the value of the virtual capability, or a `WurflError` 
    /// if any errors occur during the computation process.
    /// Errors are mainly caused by wrong virtual capability names passed in the input.
    pub fn get_virtual_cap(&self, vcap_name: &str) -> Result<Option<&str>, WurflError> {
        let c_str_cap_name = CString::new(vcap_name).map_err(|_| {
            WurflError::new(format!(
                "Unable to convert virtual capability name '{}' into a CString",
                vcap_name
            ))
        })?;
        let mut wurfl_error: u32 = 0;
        let vcap_value_ptr = unsafe { wurfl_device_get_virtual_cap(
            self.device, 
            c_str_cap_name.as_ptr(), 
            &mut wurfl_error
        ) };
        
        check_wurfl!(wurfl_error, self.wurfl)?;

        if vcap_value_ptr.is_null() {
            return Ok(None);
        }
        let vcap_value = unsafe { CStr::from_ptr(vcap_value_ptr) };
        Ok(Some(vcap_value.to_str().unwrap_or("")))
    }

    #[deprecated(since="1.5.0", note="please use `get_static_capabilities` instead")]
    pub fn get_capabilities(&self, cap_names: &[&str]) -> Result<HashMap<String, String>, WurflError> {
        let mut caps = HashMap::new();
        for cap_name in cap_names {
            let msg = format!("Unable to convert into a CString capability name  {}", cap_name);
            // this way string doesn't take ownership of cap_name
            let c_str_cap_name = match CString::new(*cap_name) {
                Ok(cn) => cn,
                Err(_) => return Err(WurflError::new(msg)),  // failed to convert to C string
            };

            let cap_ptr = unsafe { wurfl_device_get_capability(self.device, c_str_cap_name.as_ptr()) };
            if !cap_ptr.is_null() {
                let c_str = unsafe { CStr::from_ptr(cap_ptr) };
                // the string coming from get_capability should be UTF-8 compliant
                let cap_value = match c_str.to_str() {
                    Ok(cv) => cv.to_string(),
                    Err(utf_err) => {
                        let emsg = format!("Capability value for {} is an invalid UTF-8 string, it was valid until character {}", &cap_name, utf_err.valid_up_to());
                        return Err(WurflError::new(emsg));
                    }
                };
                caps.insert(cap_name.to_string(), cap_value);
            }
        };
        Ok(caps)
    }

    /// This function retrieves the static capabilities for the device and returns a `HashMap` mapping the capability names to their corresponding values.
    ///
    /// # Arguments
    /// * `cap_names` - A slice of string slices representing the names of the static capabilities to retrieve.
    ///
    /// # Returns
    /// A `Result` containing a `HashMap` mapping capability names to their values, or a `WurflError` if any errors occur during the retrieval process.
    /// Errors are mainly caused by wrong static capability names passed in the input slice
    pub fn get_static_capabilities(&self, cap_names: &[&str]) -> Result<HashMap<String, String>, WurflError> {
        let mut caps = HashMap::new();
        for cap_name in cap_names {
            let c_str_cap_name = CString::new(*cap_name).map_err(|_| {
                WurflError::new(format!(
                    "Unable to convert capability name into a CString {}",
                    cap_name
                ))
            })?; // failed to convert to C string

            let mut wurfl_error: u32 = 0;
            let cap_ptr = unsafe { wurfl_device_get_static_cap(
                self.device, 
                c_str_cap_name.as_ptr(), 
                &mut wurfl_error
            ) };

            check_wurfl!(wurfl_error, self.wurfl)?;

            if !cap_ptr.is_null() {
                let c_str = unsafe { CStr::from_ptr(cap_ptr) };
                // the string coming from get_capability should be UTF-8 compliant
                let cap_value = c_str.to_str().map_err(|utf_err| {
                    let emsg = format!(
                        "Capability value for {} is an invalid UTF-8 string, it was valid until character {}",
                        &cap_name,
                        utf_err.valid_up_to()
                    );
                    WurflError::new(emsg)
                })?.to_string();
                caps.insert(cap_name.to_string(), cap_value);
            }
        };
        Ok(caps)
    }

    
    /// This function retrieves the virtual capabilities for the device and returns a `HashMap` mapping the capability names to their corresponding values.
    ///
    /// # Arguments
    /// * `vcap_names` - A slice of strings representing the names of the virtual capabilities to compute.
    ///
    /// # Returns
    /// A `Result` containing a `HashMap` mapping virtual capability names to their values, or a `WurflError` if any errors occur during the computation process.
    /// Errors are mainly caused by wrong virtual capability names passed in the input slice.
    pub fn get_virtual_capabilities(&self, vcap_names: &[&str]) -> Result<HashMap<String, String>, WurflError> {
        let mut vcaps = HashMap::new();
        for vcap_name in vcap_names {
            let c_str_vcap_name = CString::new(*vcap_name).map_err(|_| {
                WurflError::new(format!(
                    "Unable to convert  virtual capability name  {} into a CString",
                    vcap_name
                ))
            })?;

            let mut wurfl_error: u32 = 0;
            let vcap_ptr = unsafe { wurfl_device_get_virtual_cap(
                self.device, 
                c_str_vcap_name.as_ptr(), 
                &mut wurfl_error
            ) };

            check_wurfl!(wurfl_error, self.wurfl)?;
            
            if !vcap_ptr.is_null() {
                let c_str = unsafe { CStr::from_ptr(vcap_ptr) };
                // the string coming from get_capability should be UTF-8 compliant
                let vcap_value = c_str.to_str().map_err(|utf_err| {
                    let emsg = format!(
                        "Virtual capability value for {} is an invalid UTF-8 string, it was valid until character {}",
                        &vcap_name,
                        utf_err.valid_up_to()
                    );
                    WurflError::new(emsg)
                })?.to_string();
                vcaps.insert(vcap_name.to_string(), vcap_value);
            }
        };
        Ok(vcaps)
    }

    /// Return the type of matching performed to detect this device
    pub fn get_match_type(&self) -> MatchType {
        let mt = unsafe { wurfl_device_get_match_type(self.device) };
        match mt {
            0 => MatchType::WurflMatchTypeExact,
            1 => MatchType::WurflMatchTypeConclusive,
            2 => MatchType::WurflMatchTypeRecovery,
            3 => MatchType::WurflMatchTypeCatchall,
            // Value 4 (performance) is deprecated and shouldn't be returned anymore by C API.
            5 => MatchType::WurflMatchTypeNone,
            6 => MatchType::WurflMatchTypeCached,
            // Catch all match: any other value is mapped as Match None
            _ => MatchType::WurflMatchTypeNone
        }
    }

    /// Creates a StaticCapabilitiesIterator which can be used to loop through the static capabilities available for the current device
    pub fn static_capabilities_iterator(&self) -> StaticCapabilitiesIterator  {
        StaticCapabilitiesIterator::new(&self)
    }

    /// Creates a VirtualCapabilitiesIterator which can be used to loop through the virtual capabilities available for the current device
    pub fn virtual_capabilities_iterator(&self) -> VirtualCapabilitiesIterator  {
        VirtualCapabilitiesIterator::new(&self)
    }
}

// Device memory deallocation
impl Drop for Device {
    fn drop(&mut self) {
        //println!("Destroying... WURFL device");
        unsafe { wurfl_device_destroy(self.device) };
    }
}

// makes device (and it internal objects) transferable between threads
unsafe impl Send for Device {}

// assumes Device struct thread safety, which, in our case is supported by the underlying Infuze C library
unsafe impl Sync for Device {}
