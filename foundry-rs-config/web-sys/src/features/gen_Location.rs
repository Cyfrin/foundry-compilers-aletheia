#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Location , typescript_type = "Location")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Location` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub type Location;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = href)]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn href(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = href)]
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_href(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = origin)]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn origin(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = protocol)]
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn protocol(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = protocol)]
    #[doc = "Setter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_protocol(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = host)]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn host(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = host)]
    #[doc = "Setter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_host(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = hostname)]
    #[doc = "Getter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn hostname(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = hostname)]
    #[doc = "Setter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_hostname(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = port)]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn port(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = port)]
    #[doc = "Setter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_port(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = pathname)]
    #[doc = "Getter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn pathname(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = pathname)]
    #[doc = "Setter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_pathname(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = search)]
    #[doc = "Getter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn search(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = search)]
    #[doc = "Setter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_search(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (structural , catch , method , getter , js_class = "Location" , js_name = hash)]
    #[doc = "Getter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn hash(this: &Location) -> Result<::alloc::string::String, JsValue>;
    # [wasm_bindgen (structural , catch , method , setter , js_class = "Location" , js_name = hash)]
    #[doc = "Setter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn set_hash(this: &Location, value: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Location" , js_name = assign)]
    #[doc = "The `assign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/assign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn assign(this: &Location, url: &str) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Location" , js_name = reload)]
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn reload(this: &Location) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Location" , js_name = reload)]
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn reload_with_forceget(this: &Location, forceget: bool) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "Location" , js_name = replace)]
    #[doc = "The `replace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/replace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
    pub fn replace(this: &Location, url: &str) -> Result<(), JsValue>;
}
