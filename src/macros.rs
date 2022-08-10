#[macro_export]
macro_rules! generate_bindings {
    ($(binding $static_name:ident fn $binding_name:ident($($parameter_name:ident:$parameter_type:ty),*) $(-> $return_type:ty)?;)*) => {
        $(
            static $static_name: once_cell::sync::Lazy<libloading::Symbol<'static, extern "system" fn($($parameter_name: $parameter_type),*) $(-> $return_type)?>> = once_cell::sync::Lazy::new(|| {
                if let Ok(function) = unsafe { BASS_LIBRARY.get(stringify!($binding_name).as_bytes()) } {
                    return function;
                } else {
                    panic!("Failed to load the function.");
                }
            });

            #[allow(non_snake_case)]
            pub fn $binding_name($($parameter_name: $parameter_type),*) $(-> $return_type)? {
                $static_name($($parameter_name),*)
            }
        )*
    };
}
