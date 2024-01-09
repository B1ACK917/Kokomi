use std::env;
use once_cell::sync::Lazy;

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("KKM_DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});


// For external package macro
#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        if *kokomi_core::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
            eprint!(" ");
            eprintln!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_fn {
    ($($expression:expr), *) => {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *kokomi_core::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
            eprint!(" Calling {}(),", name.strip_suffix("::f").unwrap());
            $(
                {
                    eprint!(" {:?} = {:?}", stringify!($expression), &$expression);
                }
            )*
            eprintln!();
        }
    };
}

#[macro_export]
macro_rules! debug_var {
    ($($expression:expr), *) => (
        if *kokomi_core::macros::DEBUG {
            $(
                {
                    eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
                    eprint!(" ");
                    eprint!("{:?} = {:#?}",stringify!($expression),&$expression);
                    eprintln!();
                }
            )*
        }
    )
}

// For local use
#[macro_export]
macro_rules! debug_fn_inline {
    ($($expression:expr), *) => {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *crate::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
            eprint!(" Calling {}(),", name.strip_suffix("::f").unwrap());
            $(
                {
                    eprint!(" {:?} = {:?}", stringify!($expression), &$expression);
                }
            )*
            eprintln!();
        }
    };
}