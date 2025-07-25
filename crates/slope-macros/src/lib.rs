#[macro_export]
macro_rules! declare_alkane {
    (impl AlkaneResponder for $struct_name:ident {
        type Message = $message_type:ident;
    }) => {
        #[no_mangle]
        pub extern "C" fn __execute() -> i32 {
            use alkanes_runtime::runtime::AlkaneResponder;
            use alkanes_runtime::runtime::{handle_error, handle_success, prepare_response};
            use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};

            let mut responder = $struct_name::default();
            let mut context = responder.context().unwrap();
            responder.set_context(context.clone());
            let mut inputs = context.inputs.clone();

            if inputs.is_empty() {
                let extended = handle_error("No opcode provided");
                return alkanes_runtime::runtime::response_to_i32(extended);
            }

            let opcode = inputs[0];
            inputs.remove(0);

            let result = match $message_type::from_opcode(opcode, inputs.clone()) {
                Ok(message) => message.dispatch(&mut responder),
                Err(err) => {
                    // Call the fallback method on the AlkaneResponder
                    // This will use the default implementation if not overridden by the contract
                    responder.fallback()
                }
            };

            let extended = match result {
                Ok(res) => handle_success(res),
                Err(err) => {
                    let error_msg = format!("Error: {}", err);
                    let extended = handle_error(&error_msg);
                    return alkanes_runtime::runtime::response_to_i32(extended);
                }
            };

            alkanes_runtime::runtime::response_to_i32(extended)
        }

        #[no_mangle]
        pub extern "C" fn __meta() -> i32 {
            let abi = $message_type::export_abi();
            export_bytes(&abi)
        }

        fn export_bytes(data: &[u8]) -> i32 {
            let response_bytes = to_arraybuffer_layout(data);
            Box::leak(Box::new(response_bytes)).as_mut_ptr() as usize as i32 + 4
        }
    };
}