#[macro_export]
macro_rules! debug_info {
    () => {
        eprint!("[{}][{}]", "DEBUG".bright_green(), format!("{}:{}", file!(), line!()).cyan());
    };
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        if *DEBUG{
            debug_info!();
            eprint!(" ");
            eprintln!("{}",format!($($arg)*).bright_green());
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
        if *DEBUG {
            debug_info!();
            eprint!("{}", format!(" Calling {}()",name.strip_suffix("::f").unwrap()).bright_purple());
            $(
                {
                    eprint!("{}",format!(" {:?} = {:?}", stringify!($expression), &$expression).bright_yellow());
                }
            )*
            eprintln!();
        }
    };
}

#[macro_export]
macro_rules! debug_var {
    ($($expression:expr), *) => (
        if *DEBUG {
            $(
                {
                    debug_info!();
                    eprint!(" ");
                    eprint!("{}",format!(" {:#?} = {:#?}", stringify!($expression), &$expression).bright_yellow());
                    eprintln!();
                }
            )*
        }
    )
}