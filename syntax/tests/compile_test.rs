macro_rules! parse_file {
    ($( $filename:ident),* $(,)*) => {
        $(
            #[test]
            fn $filename() {
                use std::path::Path;
                let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join("data")
                    .join(concat!(stringify!($filename), ".st"));

                assert!(path.exists(), "\"{}\" doesn't exist", path.display());

                let body = std::fs::read_to_string(&path).unwrap();

                let file: iec_syntax::File = body.parse().unwrap();

                let jason = serde_json::to_string_pretty(&file).unwrap();
                println!("{}", jason);
            }
        )*
    };
    ($filename:ident) => {
    };
}

parse_file! {
    hello_world,
    id_function,
    // struct_decl,
    // function_block,
}
