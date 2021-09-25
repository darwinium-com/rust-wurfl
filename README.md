# rust-wurfl
rust bindings to InFuze C API (libwurfl).

### wurfl-sys

This directory contains the bindings to the InFuze libwurfl (which MUST be installed to make everything work).
It requires Rust edition 2018 to work.

### rust-wurfl
rust-wurfl is the root project that contains the `wurfl-sys`. 
It provides an abstraction layer built on top of the bindings of the wurfl-sys crate. 
It exposes two structs Wurfl and Device with `lookup*` methods by User-Agent, Headers map and `device_id`.
Wurfl struct also exposes methods to update WURFL.xml (the so called WURFL-updater) like `updater_runonce`, `updater_start`, `updater_stop`.
Device struct exposes `get_capability`, `get_virtual_capability` and aggregate methods to get groups of them.
Wurfl and Device structs implement `Drop` trait to deallocate their resources.

To compile and run the rust-wurfl example you just need to run `cargo run --example example` from the rust-wurfl directory.
It will print something like this: 
```
Starting WURFL wrapper usage sample!
WURFL API Version created: 1.12.1.0
Last load time:  Thu Jun 24 12:38:03 2021
WURFL info:  Root:/usr/share/wurfl/wurfl.zip:for WURFL API 1.12.1.0 - min, db.scientiamobile.com - 2021-04-27 12:18:27:2021-04-27 12:27:39 -0400
Printing capability name from Iterator
> is_wireless_device <
Printing all capability names --------------------------
< is_wireless_device >
< model_name >
< ux_full_desktop >
< pointing_method >
< physical_screen_width >
< preferred_markup >
< is_tablet >
< resolution_height >
< resolution_width >
< can_assign_phone_number >
< marketing_name >
< brand_name >
< physical_screen_height >
< mobile_browser >
< device_os_version >
< is_smarttv >
< xhtml_support_level >
< mobile_browser_version >
< device_os >
---------------------------------------------------------
Printing all virtual capability names -------------------
< advertised_browser >
< advertised_browser_version >
< advertised_device_os >
< advertised_device_os_version >
< complete_device_name >
< device_name >
< form_factor >
< is_android >
< is_app >
< is_app_webview >
< is_full_desktop >
< is_html_preferred >
< is_ios >
< is_largescreen >
< is_mobile >
< is_phone >
< is_robot >
< is_smartphone >
< is_touchscreen >
< is_windows_phone >
< is_wml_preferred >
< is_xhtmlmp_preferred >
< advertised_app_name >
< generalized_browser_type >
< generalized_os_brand >
< pixel_density >
< is_generic >
// more output follows...
```

Please note that the number and availability of capabilities may change depending on yout WURFL license.
