extern crate kplayer_rust_wrap;
extern crate serde_json;

use kplayer_rust_wrap::kplayer;

struct ShowFilename {
    show_extension: bool,
}
impl ShowFilename {
    fn new() -> Self {
        ShowFilename {
            show_extension: false,
        }
    }
}

impl kplayer::plugin::BasePlugin for ShowFilename {
    fn get_name(&self) -> String {
        String::from("show-filename")
    }
    fn get_args(
        &mut self,
        _custom_args: std::collections::HashMap<String, String>,
    ) -> std::vec::Vec<std::string::String> {
        for item in &_custom_args {
            let log = format!("{}={}", item.0, item.1);
            kplayer::util::os::print_log(kplayer::util::os::PrintLogLevel::DEBUG, &log);
        }

        // set arguments
        if _custom_args.contains_key("show_extension") {
            let value = &_custom_args["show_extension"];
            if value == "true" || value == "1" {
                self.show_extension = true;
            }
        }

        // get history message
        let history_message = kplayer::get_history_message(
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED,
        );

        let value: serde_json::Value;
        let mut file_name = "none";

        if history_message != "history cannot be found" {
            value = serde_json::from_str(history_message.as_str()).unwrap();
            let path = value["resource"]["path"].as_str().unwrap();

            if self.show_extension {
                file_name = std::path::Path::new(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();
            } else {
                file_name = std::path::Path::new(path)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
            }
        }

        // set arg
        let mut args: Vec<std::string::String> = Vec::new();
        args.push(String::from(format!("text={}", file_name)));
        args.push(String::from("fontsize=17"));
        args.push(String::from("fontcolor=white"));
        args.push(String::from("fontfile=resource/font.ttf"));
        args.push(String::from("x=0"));
        args.push(String::from("y=0"));

        args
    }
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        vec!["x", "y", "fontsize", "fontcolor", "fontfile"]
    }
    fn get_author(&self) -> std::string::String {
        String::from("kplayer")
    }
    fn get_filter_name(&self) -> std::string::String {
        String::from("drawtext")
    }
    fn get_media_type(&self) -> kplayer::plugin::MediaType {
        kplayer::plugin::MediaType::MediaTypeVideo
    }
    fn validate_user_args(
        &self,
        _args: std::collections::HashMap<String, String>,
    ) -> std::result::Result<bool, &'static str> {
        // validate arg
        for (key, value) in _args {
            // validate font file exist
            if key == String::from("fontfile") {
                if !kplayer::util::os::file_exist(&value) {
                    self.print_log(
                        kplayer::util::os::PrintLogLevel::ERROR,
                        format!("font file not eixst: {}", &value).as_str(),
                    );
                    return Err("font file not exist");
                }
                continue;
            }
        }

        Ok(true)
    }
    fn register_message_keys(&self) -> Vec<kplayer::proto::keys::EventMessageAction> {
        let empty: Vec<kplayer::proto::keys::EventMessageAction> =
            vec![kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED];
        empty
    }
    fn execute_message(&mut self, action: i32, body: String) {
        let start_value =
            kplayer::proto::keys::EventMessageAction::EVENT_MESSAGE_ACTION_RESOURCE_CHECKED as i32;
        if action == start_value {
            let value: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
            let path = value["resource"]["path"].as_str().unwrap();
            let file_name;

            if self.show_extension {
                file_name = std::path::Path::new(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();
            } else {
                file_name = std::path::Path::new(path)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
            }

            kplayer::util::core::update_args(String::from("text"), file_name.to_string()).unwrap();
        }
    }
}

kplayer_rust_wrap::export!(ShowFilename);
