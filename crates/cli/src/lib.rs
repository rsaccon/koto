//! Exposes all the markdown docs as included raw strings

macro_rules! include_doc {
    ($doc:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/", $doc))
    };
}

pub mod docs {
    pub fn language_guide() -> &'static str {
        include_doc!("language_guide.md")
    }

    pub mod core_lib {
        pub fn io() -> &'static str {
            include_doc!("core_lib/io.md")
        }

        pub fn iterator() -> &'static str {
            include_doc!("core_lib/iterator.md")
        }

        pub fn koto() -> &'static str {
            include_doc!("core_lib/koto.md")
        }

        pub fn list() -> &'static str {
            include_doc!("core_lib/list.md")
        }

        pub fn map() -> &'static str {
            include_doc!("core_lib/map.md")
        }

        pub fn number() -> &'static str {
            include_doc!("core_lib/number.md")
        }

        pub fn os() -> &'static str {
            include_doc!("core_lib/os.md")
        }

        pub fn range() -> &'static str {
            include_doc!("core_lib/range.md")
        }

        pub fn string() -> &'static str {
            include_doc!("core_lib/string.md")
        }

        pub fn test() -> &'static str {
            include_doc!("core_lib/test.md")
        }

        pub fn tuple() -> &'static str {
            include_doc!("core_lib/tuple.md")
        }
    }

    pub mod extra_lib {
        pub fn color() -> &'static str {
            include_doc!("libs/color.md")
        }

        pub fn geometry() -> &'static str {
            include_doc!("libs/geometry.md")
        }

        pub fn json() -> &'static str {
            include_doc!("libs/json.md")
        }

        pub fn random() -> &'static str {
            include_doc!("libs/random.md")
        }

        pub fn regex() -> &'static str {
            include_doc!("libs/regex.md")
        }

        pub fn tempfile() -> &'static str {
            include_doc!("libs/tempfile.md")
        }

        pub fn toml() -> &'static str {
            include_doc!("libs/toml.md")
        }

        pub fn yaml() -> &'static str {
            include_doc!("libs/yaml.md")
        }
    }
}
