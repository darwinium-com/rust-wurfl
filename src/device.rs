/// Device provides access to device static and virtual capabilities
pub struct Device {
    // we use this attribute to silence the "variable is never read" warning
    #[allow(dead_code)]
    wurfl: wurfl_handle,
    device: wurfl_device_handle,
}

/// Device provides access to device static and virtual capabilities and other device data
impl Device {
    /// returns the WURFL device unique ID
    pub fn get_device_id(&self) -> &str {
        let d_id = unsafe { wurfl_device_get_id(self.device) };
        return to_str(d_id);
    }

    /// Returns the unique ID of this device ancestor in the WURFL hierarchy
    pub fn get_root_ID(&self) -> &str {
        let d_root_id = unsafe { wurfl_device_get_root_id(self.device) };
        return to_str(d_root_id);
    }

    /// Returns true if this device is a root device in the WURFL hierarchy
    pub fn is_root(&self) -> bool {
        let is_root = unsafe { wurfl_device_is_actual_device_root(self.device) };
        if is_root == 0 {
            return false;
        }
        return true;
    }

    /// Returns the original UserAgent of matched device (the one passed to lookup)
    pub fn get_original_user_agent(&self) -> &str {
        let d_orig_ua = unsafe { wurfl_device_get_original_useragent(self.device) };
        return to_str(d_orig_ua);
    }

    /// Return a normalized version of this device User-Agent
    pub fn get_normalized_user_agent(&self) -> &str {
        let d_norm_ua = unsafe { wurfl_device_get_normalized_useragent(self.device) };
        return to_str(d_norm_ua);
    }

    /// Returns default UserAgent of matched device (might be different from UA passed to lookup)
    pub fn get_user_agent(&self) -> &str {
        let d_ua = unsafe { wurfl_device_get_useragent(self.device) };
        return to_str(d_ua);
    }

    /// Returns the device value of the capability with the given name
    pub fn get_capability(&self, capability_name: &str) -> Option<&str> {
        let c_str_cap_name = match CString::new(capability_name) {
            Ok(cn) => cn,
            Err(_) => return None
        };
        let cap_value = unsafe { wurfl_device_get_capability(self.device, c_str_cap_name.as_ptr()) };
        if cap_value.is_null() {
            return None;
        }
        return Some(to_str(cap_value));
    }

    /// Returns the device value of the virtual capability with the given name
    pub fn get_virtual_capability(&self, virtual_capability_name: &str) -> Option<&str> {
        let c_str_vcap_name = match CString::new(virtual_capability_name) {
            Ok(cn) => cn,
            Err(_) => return None
        };
        let vcap_value = unsafe { wurfl_device_get_virtual_capability(self.device, c_str_vcap_name.as_ptr()) };
        if vcap_value.is_null() {
            return None;
        }
        return Some(to_str(vcap_value));
    }

    /// Returns a map -> key:cap_name,value:cap_value for the calling device
    pub fn get_capabilities(&self, cap_names: &[&str]) -> Result<HashMap<String, String>, WurflError> {
        let mut caps = HashMap::new();
        for cap_name in cap_names {
            let msg = format!("Unable to convert into a CString capability name  {}", cap_name);
            // this way string doesn't take ownership of cap_name
            let c_str_cap_name = match CString::new(cap_name.clone()) {
                Ok(cn) => cn,
                Err(_) => return Err(WurflError { msg: msg }),  // failed to convert to C string
            };

            let cap_ptr = unsafe { wurfl_device_get_capability(self.device, c_str_cap_name.as_ptr()) };
            if !cap_ptr.is_null() {
                let c_str = unsafe { CStr::from_ptr(cap_ptr) };
                // the string coming from get_capability should be UTF-8 compliant
                let cap_value = match c_str.to_str() {
                    Ok(cv) => cv.to_string(),
                    Err(utf_err) => {
                        let emsg = format!("Capability value for {} is an invalid UTF-8 string, it was valid until character {}", &cap_name, utf_err.valid_up_to());
                        return Err(WurflError { msg: emsg });
                    }
                };
                caps.insert(cap_name.to_string(), cap_value);
            }
        };
        return Ok(caps);
    }

    /// Returns a map -> key:vcap_name,value:vcap_value for the calling device
    pub fn get_virtual_capabilities(&self, vcap_names: &[&str]) -> Result<HashMap<String, String>, WurflError> {
        let mut vcaps = HashMap::new();
        for vcap_name in vcap_names {
            let msg = format!("Unable to convert  virtual capability name  {}! into a CString", vcap_name);
            // this way string doesn't take ownership of cap_name
            let c_str_vcap_name = match CString::new(vcap_name.clone()) {
                Ok(vcn) => vcn,
                Err(_) => return Err(WurflError { msg: msg }),  // failed to convert to C string
            };
            let vcap_ptr = unsafe { wurfl_device_get_virtual_capability(self.device, c_str_vcap_name.as_ptr()) };
            if !vcap_ptr.is_null() {
                let c_str = unsafe { CStr::from_ptr(vcap_ptr) };
                // the string coming from get_capability should be UTF-8 compliant
                let vcap_value = match c_str.to_str() {
                    Ok(cv) => cv.to_string(),
                    Err(utf_err) => {
                        let emsg = format!("Virtual capability value for {} is an invalid UTF-8 string, it was valid until characer {}", &vcap_name, utf_err.valid_up_to());
                        return Err(WurflError { msg: emsg });
                    }
                };
                vcaps.insert(vcap_name.to_string(), vcap_value);
            }
        };
        return Ok(vcaps);
    }

    /// Return the type of matching performed to detect this device
    pub fn get_match_type(&self) -> MatchType {
        let mt = unsafe { wurfl_device_get_match_type(self.device) };
        match mt {
            0 => MatchType::WurflMatchTypeExact,
            1 => MatchType::WurflMatchTypeConclusive,
            2 => MatchType::WurflMatchTypeRecovery,
            3 => MatchType::WurflMatchTypeCatchall,
            // Value 4 (performance) is deprecated and shouldn0t be returned anymore by C API.
            5 => MatchType::WurflMatchTypeNone,
            6 => MatchType::WurflMatchTypeCached,
            // Catch all match: any other value is mapped as Match None
            _ => MatchType::WurflMatchTypeNone
        }
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
