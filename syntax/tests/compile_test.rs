macro_rules! parse_file {
    ($( $filename:ident,)*) => {
        $(
            crate::parse_file!($filename);
        )*
    };
    ($filename:ident) => {
        #[test]
        fn $filename() {
            use std::path::Path;
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("data")
                .join(concat!(stringify!($filename), ".st"));

            assert!(path.exists(), "\"{}\" doesn't exist", path.display());

            let body = std::fs::read_to_string(&path).unwrap();

            let program = iec_syntax::parse(&body).unwrap();

            let jason = serde_json::to_string_pretty(&program).unwrap();
            println!("{}", jason);
        }
    };
}

parse_file!(hello_world);
