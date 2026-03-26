## rust-wurfl [1.8.0] - InFuze 1.13.6.0 - 20269-02-23

- removal of deprecated bindings

## rust-wurfl [1.7.0] - InFuze 1.13.5.0 - 2025-04-11

- Fixed a VirtualCapabilitiesIterator behaviour inconsistency
- Removed to_str helper function which had an unbound lifetime and could cause errors
- Added a constructor to WurflError

## rust-wurfl [1.6.0] - InFuze 1.13.4.0 - 2025-06-10

- refactored capability iterator to internally use get_static_cap

## rust-wurfl [1.5.0] - InFuze 1.13.3.0 - 2025-02-17

- No modification

## rust-wurfl [1.5.0] - InFuze 1.13.2.0 - 2024-10-01

- added wurfl_download function to download a WURFL file from the Snapshot Generator to a destination folder
- deprecated methods:
   get_capability (replaced by get_static_cap)
   get_virtual_capability (replaced by get_virtual_cap)
   get_capabilities (replaced by get_static_capabilities)
- fixed issue in VirtualCapabilitiesIterator

## rust-wurfl [1.4.0] - InFuze 1.13.1.0 - 2024-05-28

- No modifications

## rust-wurfl [1.3.0] - InFuze 1.13.0.0 - 2024-02-28

- No modifications

## rust-wurfl [1.3.0] - InFuze 1.12.11.0 - 2023-10-23

- No modifications

## rust-wurfl [1.3.0] - InFuze 1.12.10.0 - 2023-06-27

- added device method get_parent_id
- method get_root_id is deprecated and will be removed in a next
release. Use get_root_id instead

## rust-wurfl [1.2.0] - InFuze 1.12.8.0 - 2022-11-30

- No modifications

## rust-wurfl [1.2.0] - InFuze 1.12.7.0 - 2022-09-21

- From this version, rust-wurfl requires edition 2021

## rust-wurfl [1.2.0] - InFuze 1.12.6.0 - 2022-05-31

- Added as_str() function to HeaderQuality enum that allow to have string representation of its values ("None", "Basic" or "Full")

## rust-wurfl [1.1.0] - InFuze 1.12.5.0 - 2022-02-28

- Added functions is_ua_frozen and header_quality to handle Client Hints

## rust-wurfl [1.0.0] - InFuze 1.12.4.0 - 2021-11-23

- No modifications

## rust-wurfl [1.0.0] - InFuze 1.12.3.1 - 2021-10-21

- Added set_attr and get_attr methods

## rust-wurfl [0.9.0] - InFuze 1.12.3.0 - 2021-09-29

 - Added iterators for static and virtual capabilities

## rust-wurfl [0.8.0] - InFuze 1.12.2.0 - 2021-07-20

Initial release
- Wurfl provides detection methods:
  - lookup_useragent
  - lookup_with_headers
  - lookup_device_id_with_headers
  - lookup_device_id
    
- Device methods:
  - get_capability
  - get_virtual_capability
  - get_capabilities
  - get_virtual_capabilities
  - has_capability
  - has_virtual_capability

 - Wurfl Updater methods
   - updater_runonce
   - updater_start
   - updater_stop
   - other updater configuration methods
