use log::info;
use proxy_wasm as wasm;

#[no_mangle]
pub fn _start() {
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    info!("XXXXXXXXXXXXXXXXXXXXXXXX");
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(HelloWorld { context_id })
        },
    )
}

struct HelloWorld {
   context_id: u32, 
}

impl wasm::traits::Context for HelloWorld {}
impl wasm::traits::HttpContext for HelloWorld {
    fn on_http_request_headers(&mut self, num_headers: usize) -> wasm::types::Action {
        info!("Got {} request HTTP headers in #{}.", num_headers, self.context_id );
        let headers = self.get_http_request_headers();
        let mut authority = "";

        for (name, value) in &headers {
            if name == ":authority" {
                authority = value;
            }
        }

        self.set_http_request_header("x-hello", Some(&format!("Hello world from {}", authority)));

        wasm::types::Action::Continue
    }

    fn on_http_response_headers(&mut self, num_headers: usize) -> wasm::types::Action {
        info!("Got {} HTTP response headers in #{}.", num_headers, self.context_id );
        let headers = self.get_http_response_headers();
        let mut authority = "";

        for (name, value) in &headers {
            if name == ":authority" {
                authority = value;
            }
        }

        self.set_http_response_header("x-goodbye", Some(&format!("Goodbye world from {}", authority)));

        wasm::types::Action::Continue
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

