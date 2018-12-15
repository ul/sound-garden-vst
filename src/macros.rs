// This is essentially sciter::dispatch_script_call! which is altered to pass &root to handlers.
// It is required because of two mysterious bugs:
// 1. root obtained in EventHandler::attached is invalid when used in call_function inside andler.
// 2. when handler is called from on_script_call neither audio nor call_function works properly.
#[macro_export]
macro_rules! dispatch_script_call {

    (
        $(
            fn $name:ident ( $( $argt:ident ),* );
         )*
    ) => {

        fn dispatch_script_call(&mut self, root: sciter::HELEMENT, name: &str, argv: &[sciter::Value]) -> Option<sciter::Value>
        {
            let root = sciter::Element::from(root);
            match name {
                $(
                    stringify!($name) => {

                        // args count
                        let mut _i = 0;
                        $(
                            let _: $argt;
                            _i += 1;
                        )*
                        let argc = _i;

                        if argv.len() != argc {
                            return Some(sciter::Value::error(&format!("{} error: {} of {} arguments provided.", stringify!($name), argv.len(), argc)));
                        }

                        // call function
                        let mut _i = 0;
                        let rv = self.$name(
                            &root,
                            $(
                                {
                                    match sciter::FromValue::from_value(&argv[_i]) {
                                        Some(arg) => { _i += 1; arg },
                                        None => {
                                            // invalid type
                                            return Some(sciter::Value::error(&format!("{} error: invalid type of {} argument ({} expected, {:?} provided).",
                                                stringify!($name), _i, stringify!($argt), argv[_i])));
                                        },
                                    }
                                }
                             ),*
                        );

                        // return result value
                        return Some(sciter::Value::from(rv));
                    },
                 )*

                _ => ()
            };

            // script call not handled
            return None;
        }
    };
}
