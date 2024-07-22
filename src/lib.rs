pub use grammar::parse;

#[rust_sitter::grammar("sleigh")]
#[allow(
    clippy::declare_interior_mutable_const,
    clippy::borrow_interior_mutable_const,
    clippy::large_enum_variant
)]
pub mod grammar {
    use malachite::{num::conversion::traits::FromStringBase, Integer};
    use regex::Regex;
    use std::cell::OnceCell;
    use typed_builder::TypedBuilder;

    trait XmlUnescape {
        /// Unescape XML:
        /// - `&amp;` -> `&`
        /// - `&lt;` -> `<`
        /// - `&gt;` -> `>`
        /// - `&quot;` -> `"`
        /// - `&apos;` -> `'`
        fn xml_unescape(&self) -> String;
    }

    impl<S> XmlUnescape for S
    where
        S: AsRef<str>,
    {
        fn xml_unescape(&self) -> String {
            self.as_ref()
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&apos;", "'")
        }
    }

    impl Sleigh {
        const VERSION_REGEX: OnceCell<Regex> = OnceCell::new();
        const BIGENDIAN_REGEX: OnceCell<Regex> = OnceCell::new();
        const ALIGN_REGEX: OnceCell<Regex> = OnceCell::new();
        const UNIQBASE_REGEX: OnceCell<Regex> = OnceCell::new();
        const MAXDELAY_REGEX: OnceCell<Regex> = OnceCell::new();
        const UNIQMASK_REGEX: OnceCell<Regex> = OnceCell::new();
        const NUMSECTIONS_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[rust_sitter::language]
    #[derive(TypedBuilder, Debug, PartialEq)]
    /// Sleigh Base
    ///
    pub struct Sleigh {
        #[rust_sitter::leaf(pattern = r#"<sleigh"#)]
        #[builder(default, setter(skip))]
        _open: (),
        #[rust_sitter::leaf(
            pattern = r#"version\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Sleigh::VERSION_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"version\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        /// Technically, version is optional
        version: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"bigendian\s*=\s*"([a-z]+)""#,
            transform = |v| {
                Sleigh::BIGENDIAN_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"bigendian\s*=\s*"([a-z]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid boolean")
            }
        )]
        bigendian: bool,
        #[rust_sitter::leaf(
            pattern = r#"align\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Sleigh::ALIGN_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"align\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        align: Integer,
        #[rust_sitter::leaf(
            pattern = r#"uniqbase\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Sleigh::UNIQBASE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"uniqbase\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        uniqbase: Integer,
        #[rust_sitter::leaf(
            pattern = r#"maxdelay\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Sleigh::MAXDELAY_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"maxdelay\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()).expect("Invalid integer")
            }
        )]
        #[builder(default, setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        /// `maxdelay` is used, but is only usually set to 0x1 (1 delay slot)
        maxdelay: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"uniqmask\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Sleigh::UNIQMASK_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"uniqmask\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()).expect("Invalid integer")
            }
        )]
        #[builder(default, setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        /// `maxdelay` is used, but is only usually set to 0x1 (1 delay slot)
        uniqmask: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"numsections\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Sleigh::NUMSECTIONS_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"numsections\s*=\s*"([0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(default, setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        numsections: Option<Integer>,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        sourcefiles: SourceFiles,
        spaces: Spaces,
        symbol_table: SymbolTable,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*sleigh\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct SourceFiles {
        #[rust_sitter::leaf(pattern = r#"<\s*sourcefiles\s*>"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[builder(default)]
        source_files: Vec<SourceFile>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*sourcefiles\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl SourceFile {
        const NAME_REGEX: OnceCell<Regex> = OnceCell::new();
        const INDEX_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct SourceFile {
        #[rust_sitter::leaf(pattern = r#"<\s*sourcefile"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"name\s*=\s*"([^"]+)""#,
            transform = |v| {
                SourceFile::NAME_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"name\s*=\s*"([^"]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .to_string()
                    .xml_unescape()
            }
        )]
        name: String,
        #[rust_sitter::leaf(
            pattern = r#"index\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                SourceFile::INDEX_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"index\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(default, setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        index: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl Spaces {
        const DEFAULTSPACE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Spaces {
        #[rust_sitter::leaf(pattern = r#"<\s*spaces"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(pattern = r#"defaultspace\s*=\s*"([^"]+)""#, transform = |v| {
            Spaces::DEFAULTSPACE_REGEX
                .get_or_init(|| {
                    Regex::new(r#"defaultspace\s*=\s*"([^"]+)""#).expect("Invalid regular expression")
                })
                .captures(v)
                .expect("No captures or no capture group")
                .get(1)
                .expect("No capture group")
                .as_str()
                .to_string()
                .xml_unescape()
        })]
        defaultspace: String,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        #[builder(default)]
        spaces: Vec<AddrSpaceType>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*spaces\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl AddrSpace {
        pub const NAME_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const INDEX_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const BIGENDIAN_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const DELAY_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const DEADCODEDELAY_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const SIZE_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const WORDSIZE_REGEX: OnceCell<Regex> = OnceCell::new();
        pub const PHYSICAL_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct AddrSpace {
        #[rust_sitter::leaf(
            pattern = r#"name\s*=\s*"([^"]+)""#,
            transform = |v| {
                AddrSpace::NAME_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"name\s*=\s*"([^"]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .to_string()
                    .xml_unescape()
            }
        )]
        name: String,
        #[rust_sitter::leaf(
            pattern = r#"index\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                AddrSpace::INDEX_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"index\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        index: Integer,
        #[rust_sitter::leaf(
            pattern = r#"bigendian\s*=\s*"([a-z]+)""#,
            transform = |v| {
                AddrSpace::BIGENDIAN_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"bigendian\s*=\s*"([a-z]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid boolean")
            }
        )]
        bigendian: bool,
        #[rust_sitter::leaf(
            pattern = r#"delay\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                AddrSpace::DELAY_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"delay\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        delay: Integer,
        #[rust_sitter::leaf(
            pattern = r#"deadcodedelay\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                AddrSpace::DEADCODEDELAY_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"deadcodedelay\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(default, setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        deadcodedelay: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"size\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                AddrSpace::SIZE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"size\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        size: Integer,
        #[rust_sitter::leaf(
            pattern = r#"wordsize\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                AddrSpace::WORDSIZE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"wordsize\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(default, setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        wordsize: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"physical\s*=\s*"([a-z]+)""#,
            transform = |v| {
                AddrSpace::PHYSICAL_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"physical\s*=\s*"([a-z]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid boolean")
            }
        )]
        physical: bool,
    }

    #[derive(Debug, PartialEq)]
    pub enum AddrSpaceType {
        Base {
            #[rust_sitter::leaf(pattern = r#"<\s*space_base"#)]
            _start: (),
            space: AddrSpace,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Unique {
            #[rust_sitter::leaf(pattern = r#"<\s*space_unique"#)]
            _start: (),
            space: AddrSpace,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Other {
            #[rust_sitter::leaf(pattern = r#"<\s*space_other"#)]
            _start: (),
            space: AddrSpace,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Overlay {
            #[rust_sitter::leaf(pattern = r#"<\s*space_overlay"#)]
            _start: (),
            space: AddrSpace,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Space {
            #[rust_sitter::leaf(pattern = r#"<\s*space"#)]
            _start: (),
            space: AddrSpace,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
    }

    impl SymbolTable {
        const SCOPESIZE_REGEX: OnceCell<Regex> = OnceCell::new();
        const SYMBOLSIZE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct SymbolTable {
        #[rust_sitter::leaf(pattern = r#"<\s*symbol_table"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"scopesize\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                SymbolTable::SCOPESIZE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"scopesize\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        scopesize: Integer,
        #[rust_sitter::leaf(
            pattern = r#"symbolsize\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                SymbolTable::SYMBOLSIZE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"symbolsize\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .parse()
                    .expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        symbolsize: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        #[builder(default)]
        scopes: Vec<Scope>,
        #[builder(default)]
        symbol_headers: Vec<SymbolHeaderType>,
        #[builder(default)]
        symbols: Vec<SleighSymbolType>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*symbol_table\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl Scope {
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
        const PARENT_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Scope {
        #[rust_sitter::leaf(pattern = r#"<\s*scope"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Scope::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(
            pattern = r#"parent\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Scope::PARENT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"parent\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        parent: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl SymbolHeader {
        const NAME_REGEX: OnceCell<Regex> = OnceCell::new();
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
        const SCOPE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct SymbolHeader {
        #[rust_sitter::leaf(
            pattern = r#"name\s*=\s*"([^"]+)""#,
            transform = |v| {
                SymbolHeader::NAME_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"name\s*=\s*"([^"]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .to_string()
                    .xml_unescape()
            }
        )]
        name: String,
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, SymbolHeader::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(
            pattern = r#"scope\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, SymbolHeader::SCOPE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"scope\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        scope: Integer,
    }

    #[derive(Debug, PartialEq)]
    pub enum SymbolHeaderType {
        UserOp {
            #[rust_sitter::leaf(pattern = r#"<\s*userop_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Epsilon {
            #[rust_sitter::leaf(pattern = r#"<\s*epsilon_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Value {
            #[rust_sitter::leaf(pattern = r#"<\s*value_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        ValueMap {
            #[rust_sitter::leaf(pattern = r#"<\s*valuemap_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Name {
            #[rust_sitter::leaf(pattern = r#"<\s*name_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        VarNode {
            #[rust_sitter::leaf(pattern = r#"<\s*varnode_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Context {
            #[rust_sitter::leaf(pattern = r#"<\s*context_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        VarNodeList {
            #[rust_sitter::leaf(pattern = r#"<\s*varlist_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Operand {
            #[rust_sitter::leaf(pattern = r#"<\s*operand_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Start {
            #[rust_sitter::leaf(pattern = r#"<\s*start_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        End {
            #[rust_sitter::leaf(pattern = r#"<\s*end_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Next2 {
            #[rust_sitter::leaf(pattern = r#"<\s*next2_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        FlowDest {
            #[rust_sitter::leaf(pattern = r#"<\s*flowdest_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        FlowRef {
            #[rust_sitter::leaf(pattern = r#"<\s*flowref_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        SubTable {
            #[rust_sitter::leaf(pattern = r#"<\s*subtable_sym_head"#)]
            _start: (),
            header: SymbolHeader,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _close: (),
        },
    }

    #[derive(Debug, PartialEq)]
    pub enum PatternExpressionType {
        PatternValue(PatternValueType),
        BinaryExpression(Box<BinaryExpressionType>),
        UnaryExpression(Box<UnaryExpressionType>),
    }

    impl TokenField {
        const BIGENDIAN_REGEX: OnceCell<Regex> = OnceCell::new();
        const SIGNBIT_REGEX: OnceCell<Regex> = OnceCell::new();
        const BITSTART_REGEX: OnceCell<Regex> = OnceCell::new();
        const BITEND_REGEX: OnceCell<Regex> = OnceCell::new();
        const BYTESTART_REGEX: OnceCell<Regex> = OnceCell::new();
        const BYTEEND_REGEX: OnceCell<Regex> = OnceCell::new();
        const SHIFT_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct TokenField {
        #[rust_sitter::leaf(pattern = r#"<\s*tokenfield"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"bigendian\s*=\s*"([a-z]+)""#,
            transform = |v| {
                TokenField::BIGENDIAN_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"bigendian\s*=\s*"([a-z]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        bigendian: bool,
        #[rust_sitter::leaf(
            pattern = r#"signbit\s*=\s*"([a-z]+)""#,
            transform = |v| {
                TokenField::SIGNBIT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"signbit\s*=\s*"([a-z]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        signbit: bool,
        #[rust_sitter::leaf(
            pattern = r#"bitstart\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                TokenField::BITSTART_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"bitstart\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        bitstart: Integer,
        #[rust_sitter::leaf(
            pattern = r#"bitend\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                TokenField::BITEND_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"bitend\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        bitend: Integer,
        #[rust_sitter::leaf(
            pattern = r#"bytestart\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                TokenField::BYTESTART_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"bytestart\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        bytestart: Integer,
        #[rust_sitter::leaf(
            pattern = r#"byteend\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                TokenField::BYTEEND_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"byteend\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        byteend: Integer,
        #[rust_sitter::leaf(
            pattern = r#"shift\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                TokenField::SHIFT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"shift\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        shift: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl ContextField {
        const SIGNBIT_REGEX: OnceCell<Regex> = OnceCell::new();
        const STARTBIT_REGEX: OnceCell<Regex> = OnceCell::new();
        const ENDBIT_REGEX: OnceCell<Regex> = OnceCell::new();
        const STARTBYTE_REGEX: OnceCell<Regex> = OnceCell::new();
        const ENDBYTE_REGEX: OnceCell<Regex> = OnceCell::new();
        const SHIFT_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ContextField {
        #[rust_sitter::leaf(pattern = r#"<\s*contextfield"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"signbit\s*=\s*"([a-z]+)""#,
            transform = |v| {
                ContextField::SIGNBIT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"signbit\s*=\s*"([a-z]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        signbit: bool,
        #[rust_sitter::leaf(
            pattern = r#"startbit\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextField::STARTBIT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"startbit\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        startbit: Integer,
        #[rust_sitter::leaf(
            pattern = r#"endbit\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextField::ENDBIT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"endbit\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        endbit: Integer,
        #[rust_sitter::leaf(
            pattern = r#"startbyte\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextField::STARTBYTE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"startbyte\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        startbyte: Integer,
        #[rust_sitter::leaf(
            pattern = r#"endbyte\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextField::ENDBYTE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"endbyte\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        endbyte: Integer,
        #[rust_sitter::leaf(
            pattern = r#"shift\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextField::SHIFT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"shift\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        shift: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl ConstantValue {
        const VAL_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ConstantValue {
        #[rust_sitter::leaf(pattern = r#"<\s*intb"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"val\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ConstantValue::VAL_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"val\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        val: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl OperandValue {
        // DEC:
        const INDEX_REGEX: OnceCell<Regex> = OnceCell::new();
        // HEX:
        const TABLE_REGEX: OnceCell<Regex> = OnceCell::new();
        /// HEX: Constructor ID
        const CONSTRUCTOR_ID_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct OperandValue {
        #[rust_sitter::leaf(pattern = r#"<\s*operand_exp"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"index\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                OperandValue::INDEX_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"index\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        index: Integer,
        #[rust_sitter::leaf(
            pattern = r#"table\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, OperandValue::TABLE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"table\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        table: Integer,
        #[rust_sitter::leaf(
            pattern = r#"ct\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, OperandValue::CONSTRUCTOR_ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"ct\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        constructor_id: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    /// Class Inheritance goes:
    /// - PatternExpression:
    ///   - PatternValue : PatternExpression
    ///     - TokenField : PatternValue
    ///     - ContextField : PatternValue
    ///     - ConstantValue : PatternValue
    ///     - StartInstructionValue : PatternValue
    ///     - EndInstructionValue : PatternValue
    ///     - Next2InstructionValue : PatternValue
    ///     - OperandValue : PatternValue
    ///   - UnaryExpression : PatternExpression
    ///   - BinaryExpression : PatternExpression
    pub enum PatternValueType {
        TokenField(TokenField),
        ContextField(ContextField),
        ConstantValue(ConstantValue),
        OperandValue(OperandValue),
        StartInstructionValue {
            #[rust_sitter::leaf(pattern = r#"<\s*start_exp\s*/\s*>"#)]
            _tag: (),
        },
        EndInstructionValue {
            #[rust_sitter::leaf(pattern = r#"<\s*end_exp\s*/\s*>"#)]
            _tag: (),
        },
        Next2InstructionValue {
            #[rust_sitter::leaf(pattern = r#"<\s*next2_exp\s*/\s*>"#)]
            _tag: (),
        },
    }

    #[derive(Debug, PartialEq)]
    /// Class Inheritance goes:
    /// - PatternExpression:
    ///   - BinaryExpression : PatternExpression
    ///     - PlusExpression : BinaryExpression
    ///     - SubExpression : BinaryExpression
    ///     - MultExpression : BinaryExpression
    ///     - LeftShiftExpression : BinaryExpression
    ///     - RightShiftExpression : BinaryExpression
    ///     - AndExpression : BinaryExpression
    ///     - OrExpression : BinaryExpression
    ///     - XorExpression : BinaryExpression
    ///     - DivExpression : BinaryExpression
    ///   - PatternValue : PatternExpression
    ///   - UnaryExpression : PatternExpression
    pub enum BinaryExpressionType {
        Plus {
            #[rust_sitter::leaf(pattern = r#"<\s*plus_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*plus_exp\s*>"#)]
            _end: (),
        },
        Sub {
            #[rust_sitter::leaf(pattern = r#"<\s*sub_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*sub_exp\s*>"#)]
            _end: (),
        },
        Mult {
            #[rust_sitter::leaf(pattern = r#"<\s*mult_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*mult_exp\s*>"#)]
            _end: (),
        },
        LeftShift {
            #[rust_sitter::leaf(pattern = r#"<\s*lshift_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*lshift_exp\s*>"#)]
            _end: (),
        },
        RightShift {
            #[rust_sitter::leaf(pattern = r#"<\s*rshift_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*rshift_exp\s*>"#)]
            _end: (),
        },
        And {
            #[rust_sitter::leaf(pattern = r#"<\s*and_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*and_exp\s*>"#)]
            _end: (),
        },
        Or {
            #[rust_sitter::leaf(pattern = r#"<\s*or_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*or_exp\s*>"#)]
            _end: (),
        },
        Xor {
            #[rust_sitter::leaf(pattern = r#"<\s*xor_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*xor_exp\s*>"#)]
            _end: (),
        },
        Div {
            #[rust_sitter::leaf(pattern = r#"<\s*div_exp\s*>"#)]
            _start: (),
            left: PatternExpressionType,
            right: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*div_exp\s*>"#)]
            _end: (),
        },
    }

    #[derive(Debug, PartialEq)]
    /// Class Inheritance goes:
    /// - PatternExpression:
    ///   - PatternValue : PatternExpression
    ///   - BinaryExpression : PatternExpression
    ///   - UnaryExpression : PatternExpression
    ///     - MinusExpression : UnaryExpression
    ///     - NotExpression : UnaryExpression
    pub enum UnaryExpressionType {
        Minus {
            #[rust_sitter::leaf(pattern = r#"<\s*minus_exp\s*>"#)]
            _start: (),
            inner: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*minus_exp\s*>"#)]
            _end: (),
        },
        Not {
            #[rust_sitter::leaf(pattern = r#"<\s*not_exp\s*>"#)]
            _start: (),
            inner: PatternExpressionType,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*not_exp\s*>"#)]
            _end: (),
        },
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Value {
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patval: PatternValueType,
    }

    impl UserOpSymbol {
        const INDEX_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct UserOpSymbol {
        header: SymbolHeader,
        #[rust_sitter::leaf(
            pattern = r#"index\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                UserOpSymbol::INDEX_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"index\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        index: Integer,
    }

    #[derive(Debug, PartialEq)]
    /// Class Inheritance goes:
    /// - SleighSymbol
    ///   - SpaceSymbol : SleighSymbol
    ///   - TokenSymbol : SleighSymbol
    ///   - SectionSymbol : SleighSymbol
    ///   - UserOpSymbol : SleighSymbol
    ///   - TripleSymbol : SleighSymbol
    ///     - FamilySymbol : TripleSymbol
    ///       - ValueSymbol : FamilySymbol
    ///         - ValueMapSymbol : ValueSymbol
    ///         - NameSymbol : ValueSymbol
    ///         - ContextSymbol : ValueSymbol
    ///         - VarNodeListSymbol : ValueSymbol
    ///     - SpecificSymbol : TripleSymbol
    ///       - PatternlessSymbol : SpecificSymbol
    ///         - EpsilonSymbol : PatternlessSymbol
    ///         - VarNodeSymbol : PatternlessSymbol
    ///       - OperandSymbol : SpecificSymbol
    ///       - StartSymbol : SpecificSymbol
    ///       - EndSymbol : SpecificSymbol
    ///       - Next2Symbol : SpecificSymbol
    ///       - FlowDestSymbol : SpecificSymbol
    ///       - FlowRefSymbol : SpecificSymbol
    ///     - SubtableSymbol : SpecificSymbol
    ///   - BitRangeSymbol : SleighSymbol
    ///   - MacroSymbol : SleighSymbol
    ///   - LabelSymbol : SleighSymbol
    pub enum SleighSymbolType {
        // NOTE: Not saved/restored
        // SpaceSymbol(SpaceSymbol),
        // TokenSymbol(TokenSymbol),
        // SectionSymbol(SectionSymbol),
        UserOpSymbol {
            #[rust_sitter::leaf(pattern = r#"<\s*userop"#)]
            _start: (),
            user_op: UserOpSymbol,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        TripleSymbol(TripleSymbol),
        // NOTE: Not saved/restored
        // BitRangeSymbol(BitRangeSymbol),
    }

    impl ConstructorOperand {
        // HEX:
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
    }
    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ConstructorOperand {
        #[rust_sitter::leaf(pattern = r#"<\s*oper"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, ConstructorOperand::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl OperandPrint {
        // DEC
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct OperandPrint {
        #[rust_sitter::leaf(pattern = r#"<\s*opprint"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                OperandPrint::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl Print {
        // ESCAPED STRING
        const PIECE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Print {
        #[rust_sitter::leaf(pattern = r#"<\s*print"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"piece\s*=\s*"([^"]*)""#,
            transform = |v| {
                Print::PIECE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"piece\s*=\s*"([^"]*)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .to_string()
                    .xml_unescape()
            }
        )]
        piece: String,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum PrintPieceType {
        Operand(OperandPrint),
        Print(Print),
    }

    impl Operation {
        // DEC
        const I_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const SHIFT_REGEX: OnceCell<Regex> = OnceCell::new();
        // HEX
        const MASK_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Operation {
        #[rust_sitter::leaf(pattern = r#"<\s*context_op"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"i\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Operation::I_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"i\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        i: Integer,
        #[rust_sitter::leaf(
            pattern = r#"shift\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Operation::SHIFT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"shift\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        shift: Integer,
        #[rust_sitter::leaf(
            pattern = r#"mask\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Operation::MASK_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"mask\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        mask: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patexp: PatternExpressionType,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*context_op\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl Commit {
        // HEX
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const NUM_REGEX: OnceCell<Regex> = OnceCell::new();
        // HEX
        const MASK_REGEX: OnceCell<Regex> = OnceCell::new();
        // BOOLEAN
        const FLOW_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Commit {
        #[rust_sitter::leaf(pattern = r#"<\s*commit"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Commit::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(
            pattern = r#"num\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Commit::NUM_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"num\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        num: Integer,
        #[rust_sitter::leaf(
            pattern = r#"mask\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Commit::MASK_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"mask\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        mask: Integer,
        #[rust_sitter::leaf(
            pattern = r#"flow\s*=\s*"([a-z]+)""#,
            transform = |v| {
                Commit::FLOW_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"flow\s*=\s*"([a-z]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        flow: bool,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum ContextChangeType {
        Operation(Operation),
        Commit(Commit),
    }

    #[derive(Debug, PartialEq)]
    pub enum ConstantTemplateSelector {
        Space {
            #[rust_sitter::leaf(pattern = r#"s\s*=\s*"space""#)]
            _space: (),
        },
        Offset {
            #[rust_sitter::leaf(pattern = r#"s\s*=\s*"offset""#)]
            _offset: (),
        },
        Size {
            #[rust_sitter::leaf(pattern = r#"s\s*=\s*"size""#)]
            _size: (),
        },
        OffsetPlus {
            #[rust_sitter::leaf(pattern = r#"s\s*=\s*"offset_plus""#)]
            _offset_plus: (),
        },
    }

    impl ConstantTemplateType {
        // DEC or HEX depending on type
        const DEC_VAL_REGEX: OnceCell<Regex> = OnceCell::new();
        const HEX_VAL_REGEX: OnceCell<Regex> = OnceCell::new();
        // HEX
        const PLUS_REGEX: OnceCell<Regex> = OnceCell::new();
        // STRING
        const NAME_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(Debug, PartialEq)]
    pub enum ConstantTemplateType {
        Real {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"real""#)]
            _start: (),
            // This one is hex
            #[rust_sitter::leaf(
                pattern = r#"val\s*=\s*"0x([0-9a-fA-F]+)""#,
                transform = |v| {
                    Integer::from_string_base(16, ConstantTemplateType::HEX_VAL_REGEX
                        .get_or_init(|| {
                            Regex::new(r#"val\s*=\s*"0x([0-9a-fA-F]+)""#)
                                .expect("Invalid regular expression")
                        })
                        .captures(v)
                        .expect("No captures or no capture group")
                        .get(1)
                        .expect("No capture group").as_str()).expect("Invalid integer")
                }
            )]
            val: Integer,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Handle {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"handle""#)]
            _start: (),
            #[rust_sitter::leaf(
                pattern = r#"val\s*=\s*"(-?[0-9]+)""#,
                transform = |v| {
                    ConstantTemplateType::DEC_VAL_REGEX
                        .get_or_init(|| {
                            Regex::new(r#"val\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                        })
                        .captures(v)
                        .expect("No captures or no capture group")
                        .get(1)
                        .expect("No capture group").as_str().parse().expect("Invalid integer")
                }
            )]
            val: Integer,
            selector: ConstantTemplateSelector,
            #[rust_sitter::leaf(
                pattern = r#"plus\s*=\s*"0x([0-9a-fA-F]+)""#,
                transform = |v| {
                    Integer::from_string_base(16, ConstantTemplateType::PLUS_REGEX
                        .get_or_init(|| {
                            Regex::new(r#"plus\s*=\s*"0x([0-9a-fA-F]+)""#)
                                .expect("Invalid regular expression")
                        })
                        .captures(v)
                        .expect("No captures or no capture group")
                        .get(1)
                        .expect("No capture group").as_str()).expect("Invalid integer")
                }
            )]
            plus: Option<Integer>,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        Start {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"start"\s*/\s*>"#)]
            _start: (),
        },
        End {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"end"\s*/\s*>"#)]
            _end: (),
        },
        Next {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"next"\s*/\s*>"#)]
            _next: (),
        },
        Next2 {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"next2"\s*/\s*>"#)]
            _next2: (),
        },
        CurSpace {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"curspace"\s*/\s*>"#)]
            _cur_space: (),
        },
        CurSpaceSize {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"curspace_size"\s*/\s*>"#)]
            _cur_space_size: (),
        },
        SpaceId {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"spaceid""#)]
            _start: (),
            #[rust_sitter::leaf(
                pattern = r#"name\s*=\s*"([^"]*)""#,
                transform = |v| {
                    ConstantTemplateType::NAME_REGEX
                        .get_or_init(|| {
                            Regex::new(r#"name\s*=\s*"([^"]*)""#).expect("Invalid regular expression")
                        })
                        .captures(v)
                        .expect("No captures or no capture group")
                        .get(1)
                        .expect("No capture group")
                        .as_str()
                        .to_string()
                        .xml_unescape()
                }
            )]
            name: String,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        JumpRelative {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"relative""#)]
            _start: (),
            #[rust_sitter::leaf(
                pattern = r#"val\s*=\s*"0x([0-9a-fA-F]+)""#,
                transform = |v| {
                    Integer::from_string_base(16, ConstantTemplateType::HEX_VAL_REGEX
                        .get_or_init(|| {
                            Regex::new(r#"val\s*=\s*"0x([0-9a-fA-F]+)""#)
                                .expect("Invalid regular expression")
                        })
                        .captures(v)
                        .expect("No captures or no capture group")
                        .get(1)
                        .expect("No capture group").as_str()).expect("Invalid integer")
                }
            )]
            val: Integer,
            #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
            _end: (),
        },
        FlowRef {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"flowref"\s*/\s*>"#)]
            _flow_ref: (),
        },
        FlowDest {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"flowdest"\s*/\s*>"#)]
            _flow_dest: (),
        },
        FlowDestSize {
            #[rust_sitter::leaf(pattern = r#"<\s*const_tpl\s*type\s*=\s*"flowdest_size"\s*/\s*>"#)]
            _flow_dest_size: (),
        },
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct HandleTemplate {
        #[rust_sitter::leaf(pattern = r#"<\s*handle_tpl\s*>"#)]
        #[builder(default, setter(skip))]
        _start: (),
        space: ConstantTemplateType,
        size: ConstantTemplateType,
        ptrspace: ConstantTemplateType,
        ptroffset: ConstantTemplateType,
        ptrsize: ConstantTemplateType,
        temp_space: ConstantTemplateType,
        temp_offset: ConstantTemplateType,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*handle_tpl\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct VarNodeTemplate {
        #[rust_sitter::leaf(pattern = r#"<\s*varnode_tpl\s*>"#)]
        #[builder(default, setter(skip))]
        _start: (),
        space: ConstantTemplateType,
        offset: ConstantTemplateType,
        size: ConstantTemplateType,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*varnode_tpl\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum OperationCode {
        Blank {
            #[rust_sitter::leaf(pattern = r#"BLANK"#)]
            _blank: (),
        },
        Copy {
            #[rust_sitter::leaf(pattern = r#"COPY"#)]
            _copy: (),
        },
        Load {
            #[rust_sitter::leaf(pattern = r#"LOAD"#)]
            _load: (),
        },
        Store {
            #[rust_sitter::leaf(pattern = r#"STORE"#)]
            _store: (),
        },
        Branch {
            #[rust_sitter::leaf(pattern = r#"BRANCH"#)]
            _branch: (),
        },
        ConditionalBranch {
            #[rust_sitter::leaf(pattern = r#"CBRANCH"#)]
            _cbranch: (),
        },
        BranchIndirect {
            #[rust_sitter::leaf(pattern = r#"BRANCHIND"#)]
            _branchind: (),
        },
        Call {
            #[rust_sitter::leaf(pattern = r#"CALL"#)]
            _call: (),
        },
        CallIndirect {
            #[rust_sitter::leaf(pattern = r#"CALLIND"#)]
            _callind: (),
        },
        CallOther {
            #[rust_sitter::leaf(pattern = r#"CALLOTHER"#)]
            _callother: (),
        },
        Return {
            #[rust_sitter::leaf(pattern = r#"RETURN"#)]
            _return: (),
        },
        IntegerEqual {
            #[rust_sitter::leaf(pattern = r#"INT_EQUAL"#)]
            _int_equal: (),
        },
        IntegerNotEqual {
            #[rust_sitter::leaf(pattern = r#"INT_NOTEQUAL"#)]
            _int_notequal: (),
        },
        IntegerSignedLessThan {
            #[rust_sitter::leaf(pattern = r#"INT_SLESS"#)]
            _int_sless: (),
        },
        IntegerSignedLessThanOrEqual {
            #[rust_sitter::leaf(pattern = r#"INT_SLESSEQUAL"#)]
            _int_slessequal: (),
        },
        IntegerUnsignedLessThan {
            #[rust_sitter::leaf(pattern = r#"INT_LESS"#)]
            _int_less: (),
        },
        IntegerUnsignedLessThanOrEqual {
            #[rust_sitter::leaf(pattern = r#"INT_LESSEQUAL"#)]
            _int_lessequal: (),
        },
        IntegerZeroExtend {
            #[rust_sitter::leaf(pattern = r#"INT_ZEXT"#)]
            _int_zext: (),
        },
        IntegerSignExtend {
            #[rust_sitter::leaf(pattern = r#"INT_SEXT"#)]
            _int_sext: (),
        },
        IntegerAdd {
            #[rust_sitter::leaf(pattern = r#"INT_ADD"#)]
            _int_add: (),
        },
        IntegerSubtract {
            #[rust_sitter::leaf(pattern = r#"INT_SUB"#)]
            _int_sub: (),
        },
        IntegerCarry {
            #[rust_sitter::leaf(pattern = r#"INT_CARRY"#)]
            _int_carry: (),
        },
        IntegerSignedCarry {
            #[rust_sitter::leaf(pattern = r#"INT_SCARRY"#)]
            _int_scarry: (),
        },
        IntegerSignedBorrow {
            #[rust_sitter::leaf(pattern = r#"INT_SBORROW"#)]
            _int_sborrow: (),
        },
        IntegerTwosCompliment {
            #[rust_sitter::leaf(pattern = r#"INT_2COMP"#)]
            _int_2comp: (),
        },
        IntegerNegate {
            #[rust_sitter::leaf(pattern = r#"INT_NEGATE"#)]
            _int_negate: (),
        },
        IntegerXor {
            #[rust_sitter::leaf(pattern = r#"INT_XOR"#)]
            _int_xor: (),
        },
        IntegerAnd {
            #[rust_sitter::leaf(pattern = r#"INT_AND"#)]
            _int_and: (),
        },
        IntegerOr {
            #[rust_sitter::leaf(pattern = r#"INT_OR"#)]
            _int_or: (),
        },
        IntegerLeftShift {
            #[rust_sitter::leaf(pattern = r#"INT_LEFT"#)]
            _int_left: (),
        },
        IntegerRightShift {
            #[rust_sitter::leaf(pattern = r#"INT_RIGHT"#)]
            _int_right: (),
        },
        IntegerSignedRightShift {
            #[rust_sitter::leaf(pattern = r#"INT_SRIGHT"#)]
            _int_sright: (),
        },
        IntegerMultiply {
            #[rust_sitter::leaf(pattern = r#"INT_MULT"#)]
            _int_mult: (),
        },
        IntegerDivide {
            #[rust_sitter::leaf(pattern = r#"INT_DIV"#)]
            _int_div: (),
        },
        IntegerSignedDivide {
            #[rust_sitter::leaf(pattern = r#"INT_SDIV"#)]
            _int_sdiv: (),
        },
        IntegerRemainder {
            #[rust_sitter::leaf(pattern = r#"INT_REM"#)]
            _int_rem: (),
        },
        IntegerSignedRemainder {
            #[rust_sitter::leaf(pattern = r#"INT_SREM"#)]
            _int_srem: (),
        },
        BooleanNegate {
            #[rust_sitter::leaf(pattern = r#"BOOL_NEGATE"#)]
            _bool_negate: (),
        },
        BooleanXor {
            #[rust_sitter::leaf(pattern = r#"BOOL_XOR"#)]
            _bool_xor: (),
        },
        BooleanAnd {
            #[rust_sitter::leaf(pattern = r#"BOOL_AND"#)]
            _bool_and: (),
        },
        BooleanOr {
            #[rust_sitter::leaf(pattern = r#"BOOL_OR"#)]
            _bool_or: (),
        },
        FloatEqual {
            #[rust_sitter::leaf(pattern = r#"FLOAT_EQUAL"#)]
            _float_equal: (),
        },
        FloatNotEqual {
            #[rust_sitter::leaf(pattern = r#"FLOAT_NOTEQUAL"#)]
            _float_notequal: (),
        },
        FloatLessThan {
            #[rust_sitter::leaf(pattern = r#"FLOAT_LESS"#)]
            _float_less: (),
        },
        FloatLessThanOrEqual {
            #[rust_sitter::leaf(pattern = r#"FLOAT_LESSEQUAL"#)]
            _float_lessequal: (),
        },
        Unused1 {
            #[rust_sitter::leaf(pattern = r#"UNUSED1"#)]
            _unused1: (),
        },
        FloatNotANumber {
            #[rust_sitter::leaf(pattern = r#"FLOAT_NAN"#)]
            _float_nan: (),
        },
        FloatAdd {
            #[rust_sitter::leaf(pattern = r#"FLOAT_ADD"#)]
            _float_add: (),
        },
        FloatDivide {
            #[rust_sitter::leaf(pattern = r#"FLOAT_DIV"#)]
            _float_div: (),
        },
        FloatMultiply {
            #[rust_sitter::leaf(pattern = r#"FLOAT_MULT"#)]
            _float_mult: (),
        },
        FloatSubtract {
            #[rust_sitter::leaf(pattern = r#"FLOAT_SUB"#)]
            _float_sub: (),
        },
        FloatNegate {
            #[rust_sitter::leaf(pattern = r#"FLOAT_NEG"#)]
            _float_neg: (),
        },
        FloatAbsoluteValue {
            #[rust_sitter::leaf(pattern = r#"FLOAT_ABS"#)]
            _float_abs: (),
        },
        FloatSquareRoot {
            #[rust_sitter::leaf(pattern = r#"FLOAT_SQRT"#)]
            _float_sqrt: (),
        },
        IntegerToFloat {
            #[rust_sitter::leaf(pattern = r#"INT2FLOAT"#)]
            _int2float: (),
        },
        FloatToFloat {
            #[rust_sitter::leaf(pattern = r#"FLOAT2FLOAT"#)]
            _float2float: (),
        },
        Truncate {
            #[rust_sitter::leaf(pattern = r#"TRUNC"#)]
            _trunc: (),
        },
        Ceiling {
            #[rust_sitter::leaf(pattern = r#"CEIL"#)]
            _ceil: (),
        },
        Floor {
            #[rust_sitter::leaf(pattern = r#"FLOOR"#)]
            _floor: (),
        },
        Round {
            #[rust_sitter::leaf(pattern = r#"ROUND"#)]
            _round: (),
        },
        Build {
            #[rust_sitter::leaf(pattern = r#"BUILD"#)]
            _build: (),
        },
        DelaySlot {
            #[rust_sitter::leaf(pattern = r#"DELAY_SLOT"#)]
            _delay_slot: (),
        },
        Piece {
            #[rust_sitter::leaf(pattern = r#"PIECE"#)]
            _piece: (),
        },
        Subpiece {
            #[rust_sitter::leaf(pattern = r#"SUBPIECE"#)]
            _subpiece: (),
        },
        Cast {
            #[rust_sitter::leaf(pattern = r#"CAST"#)]
            _cast: (),
        },
        Label {
            #[rust_sitter::leaf(pattern = r#"LABEL"#)]
            _label: (),
        },
        CrossBuild {
            #[rust_sitter::leaf(pattern = r#"CROSSBUILD"#)]
            _crossbuild: (),
        },
        SegmentOp {
            #[rust_sitter::leaf(pattern = r#"SEGMENTOP"#)]
            _segmentop: (),
        },
        CpoolRef {
            #[rust_sitter::leaf(pattern = r#"CPOOLREF"#)]
            _cpoolref: (),
        },
        New {
            #[rust_sitter::leaf(pattern = r#"NEW"#)]
            _new: (),
        },
        Insert {
            #[rust_sitter::leaf(pattern = r#"INSERT"#)]
            _insert: (),
        },
        Extract {
            #[rust_sitter::leaf(pattern = r#"EXTRACT"#)]
            _extract: (),
        },
        PopCount {
            #[rust_sitter::leaf(pattern = r#"POPCOUNT"#)]
            _popcount: (),
        },
        LzCount {
            #[rust_sitter::leaf(pattern = r#"LZCOUNT"#)]
            _lzcnt: (),
        },
    }

    #[derive(Debug, PartialEq)]
    pub enum OperationTemplateOutput {
        Null {
            #[rust_sitter::leaf(pattern = r#"<\s*null\s*/\s*>"#)]
            _null: (),
        },
        Output(VarNodeTemplate),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct OperationTemplate {
        #[rust_sitter::leaf(pattern = r#"<\s*op_tpl"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(pattern = r#"code\s*=\s*""#)]
        #[builder(default, setter(skip))]
        _code_pre: (),
        code: OperationCode,
        #[rust_sitter::leaf(pattern = r#""\s*>"#)]
        #[builder(default, setter(skip))]
        _close: (),
        output: OperationTemplateOutput,
        input: Vec<VarNodeTemplate>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*op_tpl\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl ConstructorTemplate {
        // DEC
        const SECTION_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const DELAY_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const NUMLABELS_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(Debug, PartialEq)]
    pub enum ConstructorTemplateResult {
        Null {
            #[rust_sitter::leaf(pattern = r#"<\s*null\s*/\s*>"#)]
            _null: (),
        },
        Result(HandleTemplate),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ConstructorTemplate {
        #[rust_sitter::leaf(pattern = r#"<\s*construct_tpl"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"section\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ConstructorTemplate::SECTION_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"section\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        section: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"delay\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ConstructorTemplate::DELAY_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"delay\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        delay: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"labels\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ConstructorTemplate::NUMLABELS_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"labels\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        numlabels: Option<Integer>,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        result: ConstructorTemplateResult,
        vec: Vec<OperationTemplate>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*construct_tpl\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl Constructor {
        // HEX
        const PARENT_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const FIRST_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const LENGTH_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC:DEC
        const LINE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    #[rust_sitter::prec_left(1)]
    pub struct Constructor {
        #[rust_sitter::leaf(pattern = r#"<\s*constructor"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"parent\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, Constructor::PARENT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"parent\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        parent: Integer,
        #[rust_sitter::leaf(
            pattern = r#"first\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Constructor::FIRST_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"first\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        first: Integer,
        #[rust_sitter::leaf(
            pattern = r#"length\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                Constructor::LENGTH_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"length\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        length: Integer,
        #[rust_sitter::leaf(
            pattern = r#"line\s*=\s*"(-?[0-9]+):(-?[0-9]+)""#,
            transform = |v| {
                let captures = Constructor::LINE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"line\s*=\s*"(-?[0-9]+):(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group");
                let line = captures.get(1).expect("No capture group").as_str().parse().expect("Invalid integer");
                let col = captures.get(2).expect("No capture group").as_str().parse().expect("Invalid integer");
                (line, col)
            }
        )]
        #[builder(setter(transform = |v: impl Into<(Integer, Integer)>| {
            let (line, col) = v.into();
            (line, col)
        }))]
        line: (Integer, Integer),
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        operands: Vec<ConstructorOperand>,
        printpiece: Vec<PrintPieceType>,
        contexts: Vec<ContextChangeType>,
        templ: Option<ConstructorTemplate>,
        namedtempl: Vec<ConstructorTemplate>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*constructor\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl PatternBlockWord {
        // HEX
        const MASK_REGEX: OnceCell<Regex> = OnceCell::new();
        const VAL_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct PatternBlockWord {
        #[rust_sitter::leaf(pattern = r#"<\s*mask_word"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"mask\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, PatternBlockWord::MASK_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"mask\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        mask: Integer,
        #[rust_sitter::leaf(
            pattern = r#"val\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, PatternBlockWord::VAL_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"val\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        val: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl PatternBlock {
        // DEC
        const OFFSET_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const NONZERO_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct PatternBlock {
        #[rust_sitter::leaf(pattern = r#"<\s*pat_block"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"offset\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                PatternBlock::OFFSET_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"offset\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        offset: Integer,
        #[rust_sitter::leaf(
            pattern = r#"nonzero\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                PatternBlock::NONZERO_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"nonzero\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        nonzero: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        mask_vals: Vec<PatternBlockWord>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*pat_block\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct InstructionPattern {
        #[rust_sitter::leaf(pattern = r#"<\s*instruct_pat\s*>"#)]
        #[builder(default, setter(skip))]
        _start: (),
        mask_value: PatternBlock,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*instruct_pat\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ContextPattern {
        #[rust_sitter::leaf(pattern = r#"<\s*context_pat\s*>"#)]
        #[builder(default, setter(skip))]
        _start: (),
        mask_value: PatternBlock,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*context_pat\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct CombinePattern {
        #[rust_sitter::leaf(pattern = r#"<\s*combine_pat\s*>"#)]
        #[builder(default, setter(skip))]
        _start: (),
        context: ContextPattern,
        instr: InstructionPattern,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*combine_pat\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum DisjointPatternType {
        Instruction(InstructionPattern),
        Context(ContextPattern),
        Combine(CombinePattern),
    }

    impl DecisionNodePair {
        // DEC
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct DecisionNodePair {
        #[rust_sitter::leaf(pattern = r#"<\s*pair"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                DecisionNodePair::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        pattern: DisjointPatternType,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*pair\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl DecisionNode {
        // DEC
        const NUMBER_REGEX: OnceCell<Regex> = OnceCell::new();
        // BOOLEAN
        const CONTEXT_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const START_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const SIZE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct DecisionNode {
        #[rust_sitter::leaf(pattern = r#"<\s*decision"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"number\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                DecisionNode::NUMBER_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"number\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        number: Integer,
        #[rust_sitter::leaf(
            pattern = r#"context\s*=\s*"([a-z]+)""#,
            transform = |v| {
                DecisionNode::CONTEXT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"context\s*=\s*"([a-z]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        context: bool,
        #[rust_sitter::leaf(
            pattern = r#"start\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                DecisionNode::START_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"start\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        start: Integer,
        #[rust_sitter::leaf(
            pattern = r#"size\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                DecisionNode::SIZE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"size\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        bitsize: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        pairs: Vec<DecisionNodePair>,
        children: Vec<DecisionNode>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*decision\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl SubtableSymbol {
        const NUMCT_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct SubtableSymbol {
        header: SymbolHeader,
        #[rust_sitter::leaf(
            pattern = r#"numct\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                SubtableSymbol::NUMCT_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"numct\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        numct: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        constructors: Vec<Constructor>,
        decisiontree: DecisionNode,
    }

    impl ValueTableValue {
        // DEC
        const VAL_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ValueTableValue {
        #[rust_sitter::leaf(pattern = r#"<\s*valuetab"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"val\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ValueTableValue::VAL_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"val\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        val: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ValueMapSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*valuemap_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patval: PatternValueType,
        valuetable: Vec<ValueTableValue>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*valuemap_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl NameTableValue {
        const NAME_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct NameTableValue {
        #[rust_sitter::leaf(pattern = r#"<\s*nametab"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"name\s*=\s*"([^"]*)""#,
            transform = |v| {
                NameTableValue::NAME_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"name\s*=\s*"([^"]*)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .to_string()
                    .xml_unescape()
            }
        )]
        #[builder(setter(transform = |v: impl Into<String>| {
            Some(v.into())
        }))]
        name: Option<String>,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct NameSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*name_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patval: PatternValueType,
        nametable: Vec<NameTableValue>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*name_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl ContextSymbol {
        // HEX
        const VARNODE_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const LOW_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const HIGH_REGEX: OnceCell<Regex> = OnceCell::new();
        // BOOLEAN
        const FLOW_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ContextSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*context_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(
            pattern = r#"varnode\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, ContextSymbol::VARNODE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"varnode\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        varnode: Integer,
        #[rust_sitter::leaf(
            pattern = r#"low\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextSymbol::LOW_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"low\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        low: Integer,
        #[rust_sitter::leaf(
            pattern = r#"high\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                ContextSymbol::HIGH_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"high\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        high: Integer,
        #[rust_sitter::leaf(
            pattern = r#"flow\s*=\s*"([a-z]+)""#,
            transform = |v| {
                ContextSymbol::FLOW_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"flow\s*=\s*"([a-z]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        flow: bool,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patval: PatternValueType,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*context_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl VarNodeTableValue {
        // HEX
        const ID_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct VarNodeTableValue {
        #[rust_sitter::leaf(pattern = r#"<\s*var"#)]
        #[builder(default, setter(skip))]
        _start: (),
        #[rust_sitter::leaf(
            pattern = r#"id\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, VarNodeTableValue::ID_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"id\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        id: Integer,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum VarNodeTableValueType {
        Null {
            #[rust_sitter::leaf(pattern = r#"<\s*null\s*/\s*>"#)]
            _null: (),
        },
        Value(VarNodeTableValue),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct VarNodeListSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*varlist_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patval: PatternValueType,
        varnode_table: Vec<VarNodeTableValueType>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*varlist_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct ValueSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*value_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        patval: PatternValueType,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*value_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum ValueSymbolType {
        ValueMapSymbol(ValueMapSymbol),
        NameSymbol(NameSymbol),
        ContextSymbol(ContextSymbol),
        VarNodeListSymbol(VarNodeListSymbol),
        ValueSymbol(ValueSymbol),
    }

    #[derive(Debug, PartialEq)]
    pub enum FamilySymbol {
        ValueSymbol(ValueSymbolType),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct EpsilonSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*epsilon_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    impl VarNodeSymbol {
        // STRING
        const SPACE_REGEX: OnceCell<Regex> = OnceCell::new();
        // HEX
        const OFFSET_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const SIZE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct VarNodeSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*varnode_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(
            pattern = r#"space\s*=\s*"([^"]*)""#,
            transform = |v| {
                VarNodeSymbol::SPACE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"space\s*=\s*"([^"]*)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group")
                    .as_str()
                    .to_string()
                    .xml_unescape()
            }
        )]
        space: String,
        #[rust_sitter::leaf(
            pattern = r#"offset\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, VarNodeSymbol::OFFSET_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"offset\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        offset: Integer,
        #[rust_sitter::leaf(
            pattern = r#"size\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                VarNodeSymbol::SIZE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"size\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        size: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*varnode_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum PatternlessSymbol {
        EpsilonSymbol(EpsilonSymbol),
        VarNodeSymbol(VarNodeSymbol),
    }

    impl OperandSymbol {
        // HEX
        const SUBSYM_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const OFF_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const BASE_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const MINLEN_REGEX: OnceCell<Regex> = OnceCell::new();
        // DEC
        const INDEX_REGEX: OnceCell<Regex> = OnceCell::new();
        // BOOLEAN
        const CODE_REGEX: OnceCell<Regex> = OnceCell::new();
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct OperandSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*operand_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(
            pattern = r#"subsym\s*=\s*"0x([0-9a-fA-F]+)""#,
            transform = |v| {
                Integer::from_string_base(16, OperandSymbol::SUBSYM_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"subsym\s*=\s*"0x([0-9a-fA-F]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str()).expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            Some(v.into())
        }))]
        subsym: Option<Integer>,
        #[rust_sitter::leaf(
            pattern = r#"off\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                OperandSymbol::OFF_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"off\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group")
                    .get(1)
                    .expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        off: Integer,
        #[rust_sitter::leaf(
            pattern = r#"base\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                OperandSymbol::BASE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"base\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        base: Integer,
        #[rust_sitter::leaf(
            pattern = r#"minlen\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                OperandSymbol::MINLEN_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"minlen\s*=\s*"(-?[0-9]+)""#)
                            .expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        minlen: Integer,
        #[rust_sitter::leaf(
            pattern = r#"code\s*=\s*"([a-z]+)""#,
            transform = |v| {
                OperandSymbol::CODE_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"code\s*=\s*"([a-z]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid boolean")
            }
        )]
        code: Option<bool>,
        #[rust_sitter::leaf(
            pattern = r#"index\s*=\s*"(-?[0-9]+)""#,
            transform = |v| {
                OperandSymbol::INDEX_REGEX
                    .get_or_init(|| {
                        Regex::new(r#"index\s*=\s*"(-?[0-9]+)""#).expect("Invalid regular expression")
                    })
                    .captures(v)
                    .expect("No captures or no capture group").get(1).expect("No capture group").as_str().parse().expect("Invalid integer")
            }
        )]
        #[builder(setter(transform = |v: impl Into<Integer>| {
            v.into()
        }))]
        index: Integer,
        #[rust_sitter::leaf(pattern = r#">"#)]
        #[builder(default, setter(skip))]
        _close: (),
        localexp: OperandValue,
        defexp: Option<PatternExpressionType>,
        #[rust_sitter::leaf(pattern = r#"<\s*/\s*operand_sym\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct StartSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*start_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct EndSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*end_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct Next2Symbol {
        #[rust_sitter::leaf(pattern = r#"<\s*next2_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct FlowDestSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*flowdest_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(TypedBuilder, Debug, PartialEq)]
    pub struct FlowRefSymbol {
        #[rust_sitter::leaf(pattern = r#"<\s*flowref_sym"#)]
        #[builder(default, setter(skip))]
        _start: (),
        header: SymbolHeader,
        #[rust_sitter::leaf(pattern = r#"/\s*>"#)]
        #[builder(default, setter(skip))]
        _end: (),
    }

    #[derive(Debug, PartialEq)]
    pub enum SpecificSymbol {
        PatternlessSymbol(PatternlessSymbol),
        OperandSymbol(OperandSymbol),
        StartSymbol(StartSymbol),
        EndSymbol(EndSymbol),
        Next2Symbol(Next2Symbol),
        FlowDestSymbol(FlowDestSymbol),
        FlowRefSymbol(FlowRefSymbol),
    }

    #[derive(Debug, PartialEq)]
    pub enum TripleSymbol {
        FamilySymbol(FamilySymbol),
        SpecificSymbol(SpecificSymbol),
        SubtableSymbol {
            #[rust_sitter::leaf(pattern = r#"<\s*subtable_sym"#)]
            _start: (),
            subtable: SubtableSymbol,
            #[rust_sitter::leaf(pattern = r#"<\s*/\s*subtable_sym\s*>"#)]
            _end: (),
        },
    }

    #[rust_sitter::extra]
    #[derive(Debug)]
    pub struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }
}

#[allow(non_upper_case_globals)]
#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn test_6502() {
        const SLA_6502: &str = include_str!("../Processors/6502/data/languages/6502.sla");
        let slgh = parse(SLA_6502).expect("Failed to parse 6502 sla");
        println!("{slgh:#?}");
    }
    #[test]
    fn test_65c02() {
        const SLA_65c02: &str = include_str!("../Processors/6502/data/languages/65c02.sla");
        parse(SLA_65c02).expect("Failed to parse 65c02 sla");
    }
    #[test]
    fn test_68020() {
        const SLA_68020: &str = include_str!("../Processors/68000/data/languages/68020.sla");
        parse(SLA_68020).expect("Failed to parse 68020 sla");
    }
    #[test]
    fn test_68030() {
        const SLA_68030: &str = include_str!("../Processors/68000/data/languages/68030.sla");
        parse(SLA_68030).expect("Failed to parse 68030 sla");
    }
    #[test]
    fn test_68040() {
        const SLA_68040: &str = include_str!("../Processors/68000/data/languages/68040.sla");
        parse(SLA_68040).expect("Failed to parse 68040 sla");
    }
    #[test]
    fn test_coldfire() {
        const coldfire: &str = include_str!("../Processors/68000/data/languages/coldfire.sla");
        parse(coldfire).expect("Failed to parse coldfire sla");
    }
    #[test]
    fn test_8048() {
        const SLA_8048: &str = include_str!("../Processors/8048/data/languages/8048.sla");
        parse(SLA_8048).expect("Failed to parse 8048 sla");
    }
    #[test]
    fn test_80251() {
        const SLA_80251: &str = include_str!("../Processors/8051/data/languages/80251.sla");
        parse(SLA_80251).expect("Failed to parse 80251 sla");
    }
    #[test]
    fn test_80390() {
        const SLA_80390: &str = include_str!("../Processors/8051/data/languages/80390.sla");
        parse(SLA_80390).expect("Failed to parse 80390 sla");
    }
    #[test]
    fn test_8051() {
        const SLA_8051: &str = include_str!("../Processors/8051/data/languages/8051.sla");
        parse(SLA_8051).expect("Failed to parse 8051 sla");
    }
    #[test]
    fn test_mx51() {
        const mx51: &str = include_str!("../Processors/8051/data/languages/mx51.sla");
        parse(mx51).expect("Failed to parse mx51 sla");
    }
    #[test]
    fn test_8085() {
        const SLA_8085: &str = include_str!("../Processors/8085/data/languages/8085.sla");
        parse(SLA_8085).expect("Failed to parse 8085 sla");
    }
    #[test]
    fn test_AARCH64() {
        const AARCH64: &str = include_str!("../Processors/AARCH64/data/languages/AARCH64.sla");
        parse(AARCH64).expect("Failed to parse AARCH64 sla");
    }
    #[test]
    fn test_AARCH64BE() {
        const AARCH64BE: &str = include_str!("../Processors/AARCH64/data/languages/AARCH64BE.sla");
        parse(AARCH64BE).expect("Failed to parse AARCH64BE sla");
    }
    #[test]
    fn test_AARCH64_AppleSilicon() {
        const AARCH64_AppleSilicon: &str =
            include_str!("../Processors/AARCH64/data/languages/AARCH64_AppleSilicon.sla");
        parse(AARCH64_AppleSilicon).expect("Failed to parse AARCH64_AppleSilicon sla");
    }
    #[test]
    fn test_ARM4_be() {
        const ARM4_be: &str = include_str!("../Processors/ARM/data/languages/ARM4_be.sla");
        parse(ARM4_be).expect("Failed to parse ARM4_be sla");
    }
    #[test]
    fn test_ARM4_le() {
        const ARM4_le: &str = include_str!("../Processors/ARM/data/languages/ARM4_le.sla");
        parse(ARM4_le).expect("Failed to parse ARM4_le sla");
    }
    #[test]
    fn test_ARM4t_be() {
        const ARM4t_be: &str = include_str!("../Processors/ARM/data/languages/ARM4t_be.sla");
        parse(ARM4t_be).expect("Failed to parse ARM4t_be sla");
    }
    #[test]
    fn test_ARM4t_le() {
        const ARM4t_le: &str = include_str!("../Processors/ARM/data/languages/ARM4t_le.sla");
        parse(ARM4t_le).expect("Failed to parse ARM4t_le sla");
    }
    #[test]
    fn test_ARM5_be() {
        const ARM5_be: &str = include_str!("../Processors/ARM/data/languages/ARM5_be.sla");
        parse(ARM5_be).expect("Failed to parse ARM5_be sla");
    }
    #[test]
    fn test_ARM5_le() {
        const ARM5_le: &str = include_str!("../Processors/ARM/data/languages/ARM5_le.sla");
        parse(ARM5_le).expect("Failed to parse ARM5_le sla");
    }
    #[test]
    fn test_ARM5t_be() {
        const ARM5t_be: &str = include_str!("../Processors/ARM/data/languages/ARM5t_be.sla");
        parse(ARM5t_be).expect("Failed to parse ARM5t_be sla");
    }
    #[test]
    fn test_ARM5t_le() {
        const ARM5t_le: &str = include_str!("../Processors/ARM/data/languages/ARM5t_le.sla");
        parse(ARM5t_le).expect("Failed to parse ARM5t_le sla");
    }
    #[test]
    fn test_ARM6_be() {
        const ARM6_be: &str = include_str!("../Processors/ARM/data/languages/ARM6_be.sla");
        parse(ARM6_be).expect("Failed to parse ARM6_be sla");
    }
    #[test]
    fn test_ARM6_le() {
        const ARM6_le: &str = include_str!("../Processors/ARM/data/languages/ARM6_le.sla");
        parse(ARM6_le).expect("Failed to parse ARM6_le sla");
    }
    #[test]
    fn test_ARM7_be() {
        const ARM7_be: &str = include_str!("../Processors/ARM/data/languages/ARM7_be.sla");
        parse(ARM7_be).expect("Failed to parse ARM7_be sla");
    }
    #[test]
    fn test_ARM7_le() {
        const ARM7_le: &str = include_str!("../Processors/ARM/data/languages/ARM7_le.sla");
        parse(ARM7_le).expect("Failed to parse ARM7_le sla");
    }
    #[test]
    fn test_ARM8_be() {
        const ARM8_be: &str = include_str!("../Processors/ARM/data/languages/ARM8_be.sla");
        parse(ARM8_be).expect("Failed to parse ARM8_be sla");
    }
    #[test]
    fn test_ARM8_le() {
        const ARM8_le: &str = include_str!("../Processors/ARM/data/languages/ARM8_le.sla");
        parse(ARM8_le).expect("Failed to parse ARM8_le sla");
    }
    #[test]
    fn test_avr32a() {
        const avr32a: &str = include_str!("../Processors/Atmel/data/languages/avr32a.sla");
        parse(avr32a).expect("Failed to parse avr32a sla");
    }
    #[test]
    fn test_avr8() {
        const avr8: &str = include_str!("../Processors/Atmel/data/languages/avr8.sla");
        parse(avr8).expect("Failed to parse avr8 sla");
    }
    #[test]
    fn test_avr8e() {
        const avr8e: &str = include_str!("../Processors/Atmel/data/languages/avr8e.sla");
        parse(avr8e).expect("Failed to parse avr8e sla");
    }
    #[test]
    fn test_avr8eind() {
        const avr8eind: &str = include_str!("../Processors/Atmel/data/languages/avr8eind.sla");
        parse(avr8eind).expect("Failed to parse avr8eind sla");
    }
    #[test]
    fn test_avr8xmega() {
        const avr8xmega: &str = include_str!("../Processors/Atmel/data/languages/avr8xmega.sla");
        parse(avr8xmega).expect("Failed to parse avr8xmega sla");
    }
    #[test]
    fn test_BPF_le() {
        const BPF_le: &str = include_str!("../Processors/BPF/data/languages/BPF_le.sla");
        parse(BPF_le).expect("Failed to parse BPF_le sla");
    }
    #[test]
    fn test_CP1600() {
        const CP1600: &str = include_str!("../Processors/CP1600/data/languages/CP1600.sla");
        parse(CP1600).expect("Failed to parse CP1600 sla");
    }
    #[test]
    fn test_CR16B() {
        const CR16B: &str = include_str!("../Processors/CR16/data/languages/CR16B.sla");
        parse(CR16B).expect("Failed to parse CR16B sla");
    }
    #[test]
    fn test_CR16C() {
        const CR16C: &str = include_str!("../Processors/CR16/data/languages/CR16C.sla");
        parse(CR16C).expect("Failed to parse CR16C sla");
    }
    #[test]
    fn test_data_be_64() {
        const data_be_64: &str = include_str!("../Processors/DATA/data/languages/data-be-64.sla");
        parse(data_be_64).expect("Failed to parse data-be-64 sla");
    }
    #[test]
    fn test_data_le_64() {
        const data_le_64: &str = include_str!("../Processors/DATA/data/languages/data-le-64.sla");
        parse(data_le_64).expect("Failed to parse data-le-64 sla");
    }
    #[test]
    fn test_Dalvik_Base() {
        const Dalvik_Base: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_Base.sla");
        parse(Dalvik_Base).expect("Failed to parse Dalvik_Base sla");
    }
    #[test]
    fn test_Dalvik_DEX_Android10() {
        const Dalvik_DEX_Android10: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Android10.sla");
        parse(Dalvik_DEX_Android10).expect("Failed to parse Dalvik_DEX_Android10 sla");
    }
    #[test]
    fn test_Dalvik_DEX_Android11() {
        const Dalvik_DEX_Android11: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Android11.sla");
        parse(Dalvik_DEX_Android11).expect("Failed to parse Dalvik_DEX_Android11 sla");
    }
    #[test]
    fn test_Dalvik_DEX_Android12() {
        const Dalvik_DEX_Android12: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Android12.sla");
        parse(Dalvik_DEX_Android12).expect("Failed to parse Dalvik_DEX_Android12 sla");
    }
    #[test]
    fn test_Dalvik_DEX_KitKat() {
        const Dalvik_DEX_KitKat: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_KitKat.sla");
        parse(Dalvik_DEX_KitKat).expect("Failed to parse Dalvik_DEX_KitKat sla");
    }
    #[test]
    fn test_Dalvik_DEX_Lollipop() {
        const Dalvik_DEX_Lollipop: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Lollipop.sla");
        parse(Dalvik_DEX_Lollipop).expect("Failed to parse Dalvik_DEX_Lollipop sla");
    }
    #[test]
    fn test_Dalvik_DEX_Marshmallow() {
        const Dalvik_DEX_Marshmallow: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Marshmallow.sla");
        parse(Dalvik_DEX_Marshmallow).expect("Failed to parse Dalvik_DEX_Marshmallow sla");
    }
    #[test]
    fn test_Dalvik_DEX_Nougat() {
        const Dalvik_DEX_Nougat: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Nougat.sla");
        parse(Dalvik_DEX_Nougat).expect("Failed to parse Dalvik_DEX_Nougat sla");
    }
    #[test]
    fn test_Dalvik_DEX_Oreo() {
        const Dalvik_DEX_Oreo: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Oreo.sla");
        parse(Dalvik_DEX_Oreo).expect("Failed to parse Dalvik_DEX_Oreo sla");
    }
    #[test]
    fn test_Dalvik_DEX_Pie() {
        const Dalvik_DEX_Pie: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_DEX_Pie.sla");
        parse(Dalvik_DEX_Pie).expect("Failed to parse Dalvik_DEX_Pie sla");
    }
    #[test]
    fn test_Dalvik_ODEX_KitKat() {
        const Dalvik_ODEX_KitKat: &str =
            include_str!("../Processors/Dalvik/data/languages/Dalvik_ODEX_KitKat.sla");
        parse(Dalvik_ODEX_KitKat).expect("Failed to parse Dalvik_ODEX_KitKat sla");
    }
    #[test]
    fn test_HC05() {
        const HC05: &str = include_str!("../Processors/HCS08/data/languages/HC05.sla");
        parse(HC05).expect("Failed to parse HC05 sla");
    }
    #[test]
    fn test_HC08() {
        const HC08: &str = include_str!("../Processors/HCS08/data/languages/HC08.sla");
        parse(HC08).expect("Failed to parse HC08 sla");
    }
    #[test]
    fn test_HCS08() {
        const HCS08: &str = include_str!("../Processors/HCS08/data/languages/HCS08.sla");
        parse(HCS08).expect("Failed to parse HCS08 sla");
    }
    #[test]
    fn test_HC12() {
        const HC12: &str = include_str!("../Processors/HCS12/data/languages/HC12.sla");
        parse(HC12).expect("Failed to parse HC12 sla");
    }
    #[test]
    fn test_HCS12() {
        const HCS12: &str = include_str!("../Processors/HCS12/data/languages/HCS12.sla");
        parse(HCS12).expect("Failed to parse HCS12 sla");
    }
    #[test]
    fn test_HCS12X() {
        const HCS12X: &str = include_str!("../Processors/HCS12/data/languages/HCS12X.sla");
        parse(HCS12X).expect("Failed to parse HCS12X sla");
    }
    #[test]
    fn test_JVM() {
        const JVM: &str = include_str!("../Processors/JVM/data/languages/JVM.sla");
        parse(JVM).expect("Failed to parse JVM sla");
    }
    #[test]
    fn test_m8c() {
        const m8c: &str = include_str!("../Processors/M8C/data/languages/m8c.sla");
        parse(m8c).expect("Failed to parse m8c sla");
    }
    #[test]
    fn test_6805() {
        const SLA_6805: &str = include_str!("../Processors/MC6800/data/languages/6805.sla");
        parse(SLA_6805).expect("Failed to parse 6805 sla");
    }
    #[test]
    fn test_6809() {
        const SLA_6809: &str = include_str!("../Processors/MC6800/data/languages/6809.sla");
        parse(SLA_6809).expect("Failed to parse 6809 sla");
    }
    #[test]
    fn test_H6309() {
        const H6309: &str = include_str!("../Processors/MC6800/data/languages/H6309.sla");
        parse(H6309).expect("Failed to parse H6309 sla");
    }
    #[test]
    fn test_MCS96() {
        const MCS96: &str = include_str!("../Processors/MCS96/data/languages/MCS96.sla");
        parse(MCS96).expect("Failed to parse MCS96 sla");
    }
    #[test]
    fn test_mips32R6be() {
        const mips32R6be: &str = include_str!("../Processors/MIPS/data/languages/mips32R6be.sla");
        parse(mips32R6be).expect("Failed to parse mips32R6be sla");
    }
    #[test]
    fn test_mips32R6le() {
        const mips32R6le: &str = include_str!("../Processors/MIPS/data/languages/mips32R6le.sla");
        parse(mips32R6le).expect("Failed to parse mips32R6le sla");
    }
    #[test]
    fn test_mips32be() {
        const mips32be: &str = include_str!("../Processors/MIPS/data/languages/mips32be.sla");
        parse(mips32be).expect("Failed to parse mips32be sla");
    }
    #[test]
    fn test_mips32le() {
        const mips32le: &str = include_str!("../Processors/MIPS/data/languages/mips32le.sla");
        parse(mips32le).expect("Failed to parse mips32le sla");
    }
    #[test]
    fn test_mips64be() {
        const mips64be: &str = include_str!("../Processors/MIPS/data/languages/mips64be.sla");
        parse(mips64be).expect("Failed to parse mips64be sla");
    }
    #[test]
    fn test_mips64le() {
        const mips64le: &str = include_str!("../Processors/MIPS/data/languages/mips64le.sla");
        parse(mips64le).expect("Failed to parse mips64le sla");
    }
    #[test]
    fn test_pa_risc32be() {
        const pa_risc32be: &str =
            include_str!("../Processors/PA-RISC/data/languages/pa-risc32be.sla");
        parse(pa_risc32be).expect("Failed to parse pa-risc32be sla");
    }
    #[test]
    fn test_PIC24E() {
        const PIC24E: &str = include_str!("../Processors/PIC/data/languages/PIC24E.sla");
        parse(PIC24E).expect("Failed to parse PIC24E sla");
    }
    #[test]
    fn test_PIC24F() {
        const PIC24F: &str = include_str!("../Processors/PIC/data/languages/PIC24F.sla");
        parse(PIC24F).expect("Failed to parse PIC24F sla");
    }
    #[test]
    fn test_PIC24H() {
        const PIC24H: &str = include_str!("../Processors/PIC/data/languages/PIC24H.sla");
        parse(PIC24H).expect("Failed to parse PIC24H sla");
    }
    #[test]
    fn test_dsPIC30F() {
        const dsPIC30F: &str = include_str!("../Processors/PIC/data/languages/dsPIC30F.sla");
        parse(dsPIC30F).expect("Failed to parse dsPIC30F sla");
    }
    #[test]
    fn test_dsPIC33C() {
        const dsPIC33C: &str = include_str!("../Processors/PIC/data/languages/dsPIC33C.sla");
        parse(dsPIC33C).expect("Failed to parse dsPIC33C sla");
    }
    #[test]
    fn test_dsPIC33E() {
        const dsPIC33E: &str = include_str!("../Processors/PIC/data/languages/dsPIC33E.sla");
        parse(dsPIC33E).expect("Failed to parse dsPIC33E sla");
    }
    #[test]
    fn test_dsPIC33F() {
        const dsPIC33F: &str = include_str!("../Processors/PIC/data/languages/dsPIC33F.sla");
        parse(dsPIC33F).expect("Failed to parse dsPIC33F sla");
    }
    #[test]
    fn test_pic12c5xx() {
        const pic12c5xx: &str = include_str!("../Processors/PIC/data/languages/pic12c5xx.sla");
        parse(pic12c5xx).expect("Failed to parse pic12c5xx sla");
    }
    #[test]
    fn test_pic16() {
        const pic16: &str = include_str!("../Processors/PIC/data/languages/pic16.sla");
        parse(pic16).expect("Failed to parse pic16 sla");
    }
    #[test]
    fn test_pic16c5x() {
        const pic16c5x: &str = include_str!("../Processors/PIC/data/languages/pic16c5x.sla");
        parse(pic16c5x).expect("Failed to parse pic16c5x sla");
    }
    #[test]
    fn test_pic16f() {
        const pic16f: &str = include_str!("../Processors/PIC/data/languages/pic16f.sla");
        parse(pic16f).expect("Failed to parse pic16f sla");
    }
    #[test]
    fn test_pic17c7xx() {
        const pic17c7xx: &str = include_str!("../Processors/PIC/data/languages/pic17c7xx.sla");
        parse(pic17c7xx).expect("Failed to parse pic17c7xx sla");
    }
    #[test]
    fn test_pic18() {
        const pic18: &str = include_str!("../Processors/PIC/data/languages/pic18.sla");
        parse(pic18).expect("Failed to parse pic18 sla");
    }
    #[test]
    fn test_ppc_32_4xx_be() {
        const ppc_32_4xx_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_32_4xx_be.sla");
        parse(ppc_32_4xx_be).expect("Failed to parse ppc_32_4xx_be sla");
    }
    #[test]
    fn test_ppc_32_4xx_le() {
        const ppc_32_4xx_le: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_32_4xx_le.sla");
        parse(ppc_32_4xx_le).expect("Failed to parse ppc_32_4xx_le sla");
    }
    #[test]
    fn test_ppc_32_be() {
        const ppc_32_be: &str = include_str!("../Processors/PowerPC/data/languages/ppc_32_be.sla");
        parse(ppc_32_be).expect("Failed to parse ppc_32_be sla");
    }
    #[test]
    fn test_ppc_32_e500_be() {
        const ppc_32_e500_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_32_e500_be.sla");
        parse(ppc_32_e500_be).expect("Failed to parse ppc_32_e500_be sla");
    }
    #[test]
    fn test_ppc_32_e500_le() {
        const ppc_32_e500_le: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_32_e500_le.sla");
        parse(ppc_32_e500_le).expect("Failed to parse ppc_32_e500_le sla");
    }
    #[test]
    fn test_ppc_32_le() {
        const ppc_32_le: &str = include_str!("../Processors/PowerPC/data/languages/ppc_32_le.sla");
        parse(ppc_32_le).expect("Failed to parse ppc_32_le sla");
    }
    #[test]
    fn test_ppc_32_quicciii_be() {
        const ppc_32_quicciii_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_32_quicciii_be.sla");
        parse(ppc_32_quicciii_be).expect("Failed to parse ppc_32_quicciii_be sla");
    }
    #[test]
    fn test_ppc_32_quicciii_le() {
        const ppc_32_quicciii_le: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_32_quicciii_le.sla");
        parse(ppc_32_quicciii_le).expect("Failed to parse ppc_32_quicciii_le sla");
    }
    #[test]
    fn test_ppc_64_be() {
        const ppc_64_be: &str = include_str!("../Processors/PowerPC/data/languages/ppc_64_be.sla");
        parse(ppc_64_be).expect("Failed to parse ppc_64_be sla");
    }
    #[test]
    fn test_ppc_64_isa_altivec_be() {
        const ppc_64_isa_altivec_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_64_isa_altivec_be.sla");
        parse(ppc_64_isa_altivec_be).expect("Failed to parse ppc_64_isa_altivec_be sla");
    }
    #[test]
    fn test_ppc_64_isa_altivec_le() {
        const ppc_64_isa_altivec_le: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_64_isa_altivec_le.sla");
        parse(ppc_64_isa_altivec_le).expect("Failed to parse ppc_64_isa_altivec_le sla");
    }
    #[test]
    fn test_ppc_64_isa_altivec_vle_be() {
        const ppc_64_isa_altivec_vle_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_64_isa_altivec_vle_be.sla");
        parse(ppc_64_isa_altivec_vle_be).expect("Failed to parse ppc_64_isa_altivec_vle_be sla");
    }
    #[test]
    fn test_ppc_64_isa_be() {
        const ppc_64_isa_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_64_isa_be.sla");
        parse(ppc_64_isa_be).expect("Failed to parse ppc_64_isa_be sla");
    }
    #[test]
    fn test_ppc_64_isa_le() {
        const ppc_64_isa_le: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_64_isa_le.sla");
        parse(ppc_64_isa_le).expect("Failed to parse ppc_64_isa_le sla");
    }
    #[test]
    fn test_ppc_64_isa_vle_be() {
        const ppc_64_isa_vle_be: &str =
            include_str!("../Processors/PowerPC/data/languages/ppc_64_isa_vle_be.sla");
        parse(ppc_64_isa_vle_be).expect("Failed to parse ppc_64_isa_vle_be sla");
    }
    #[test]
    fn test_ppc_64_le() {
        const ppc_64_le: &str = include_str!("../Processors/PowerPC/data/languages/ppc_64_le.sla");
        parse(ppc_64_le).expect("Failed to parse ppc_64_le sla");
    }

    #[test]
    fn test_riscv_ilp32d() {
        const riscv_ilp32d: &str =
            include_str!("../Processors/RISCV/data/languages/riscv.ilp32d.sla");
        parse(riscv_ilp32d).expect("Failed to parse riscv_ilp32d sla");
    }

    #[test]
    fn test_riscv_lp64d() {
        const riscv_lp64d: &str =
            include_str!("../Processors/RISCV/data/languages/riscv.lp64d.sla");
        parse(riscv_lp64d).expect("Failed to parse riscv_lp64d sla");
    }

    #[test]
    fn test_SparcV9_32() {
        const SparcV9_32: &str = include_str!("../Processors/Sparc/data/languages/SparcV9_32.sla");
        parse(SparcV9_32).expect("Failed to parse SparcV9_32 sla");
    }
    #[test]
    fn test_SparcV9_64() {
        const SparcV9_64: &str = include_str!("../Processors/Sparc/data/languages/SparcV9_64.sla");
        parse(SparcV9_64).expect("Failed to parse SparcV9_64 sla");
    }
    #[test]
    fn test_sh_1() {
        const sh_1: &str = include_str!("../Processors/SuperH/data/languages/sh-1.sla");
        parse(sh_1).expect("Failed to parse sh-1 sla");
    }
    #[test]
    fn test_sh_2() {
        const sh_2: &str = include_str!("../Processors/SuperH/data/languages/sh-2.sla");
        parse(sh_2).expect("Failed to parse sh-2 sla");
    }
    #[test]
    fn test_sh_2a() {
        const sh_2a: &str = include_str!("../Processors/SuperH/data/languages/sh-2a.sla");
        parse(sh_2a).expect("Failed to parse sh-2a sla");
    }
    #[test]
    fn test_SuperH4_be() {
        const SuperH4_be: &str =
            include_str!("../Processors/SuperH4/data/languages/SuperH4_be.sla");
        parse(SuperH4_be).expect("Failed to parse SuperH4_be sla");
    }
    #[test]
    fn test_SuperH4_le() {
        const SuperH4_le: &str =
            include_str!("../Processors/SuperH4/data/languages/SuperH4_le.sla");
        parse(SuperH4_le).expect("Failed to parse SuperH4_le sla");
    }
    #[test]
    fn test_TI_MSP430() {
        const TI_MSP430: &str =
            include_str!("../Processors/TI_MSP430/data/languages/TI_MSP430.sla");
        parse(TI_MSP430).expect("Failed to parse TI_MSP430 sla");
    }
    #[test]
    fn test_TI_MSP430X() {
        const TI_MSP430X: &str =
            include_str!("../Processors/TI_MSP430/data/languages/TI_MSP430X.sla");
        parse(TI_MSP430X).expect("Failed to parse TI_MSP430X sla");
    }
    #[test]
    fn test_toy64_be() {
        const toy64_be: &str = include_str!("../Processors/Toy/data/languages/toy64_be.sla");
        parse(toy64_be).expect("Failed to parse toy64_be sla");
    }
    #[test]
    fn test_toy64_be_harvard() {
        const toy64_be_harvard: &str =
            include_str!("../Processors/Toy/data/languages/toy64_be_harvard.sla");
        parse(toy64_be_harvard).expect("Failed to parse toy64_be_harvard sla");
    }
    #[test]
    fn test_toy64_le() {
        const toy64_le: &str = include_str!("../Processors/Toy/data/languages/toy64_le.sla");
        parse(toy64_le).expect("Failed to parse toy64_le sla");
    }
    #[test]
    fn test_toy_be() {
        const toy_be: &str = include_str!("../Processors/Toy/data/languages/toy_be.sla");
        parse(toy_be).expect("Failed to parse toy_be sla");
    }
    #[test]
    fn test_toy_be_posStack() {
        const toy_be_posStack: &str =
            include_str!("../Processors/Toy/data/languages/toy_be_posStack.sla");
        parse(toy_be_posStack).expect("Failed to parse toy_be_posStack sla");
    }
    #[test]
    fn test_toy_builder_be() {
        const toy_builder_be: &str =
            include_str!("../Processors/Toy/data/languages/toy_builder_be.sla");
        parse(toy_builder_be).expect("Failed to parse toy_builder_be sla");
    }
    #[test]
    fn test_toy_builder_be_align2() {
        const toy_builder_be_align2: &str =
            include_str!("../Processors/Toy/data/languages/toy_builder_be_align2.sla");
        parse(toy_builder_be_align2).expect("Failed to parse toy_builder_be_align2 sla");
    }
    #[test]
    fn test_toy_builder_le() {
        const toy_builder_le: &str =
            include_str!("../Processors/Toy/data/languages/toy_builder_le.sla");
        parse(toy_builder_le).expect("Failed to parse toy_builder_le sla");
    }
    #[test]
    fn test_toy_builder_le_align2() {
        const toy_builder_le_align2: &str =
            include_str!("../Processors/Toy/data/languages/toy_builder_le_align2.sla");
        parse(toy_builder_le_align2).expect("Failed to parse toy_builder_le_align2 sla");
    }
    #[test]
    fn test_toy_le() {
        const toy_le: &str = include_str!("../Processors/Toy/data/languages/toy_le.sla");
        parse(toy_le).expect("Failed to parse toy_le sla");
    }
    #[test]
    fn test_toy_wsz_be() {
        const toy_wsz_be: &str = include_str!("../Processors/Toy/data/languages/toy_wsz_be.sla");
        parse(toy_wsz_be).expect("Failed to parse toy_wsz_be sla");
    }
    #[test]
    fn test_toy_wsz_le() {
        const toy_wsz_le: &str = include_str!("../Processors/Toy/data/languages/toy_wsz_le.sla");
        parse(toy_wsz_le).expect("Failed to parse toy_wsz_le sla");
    }
    #[test]
    fn test_V850() {
        const V850: &str = include_str!("../Processors/V850/data/languages/V850.sla");
        parse(V850).expect("Failed to parse V850 sla");
    }
    #[test]
    fn test_z180() {
        const z180: &str = include_str!("../Processors/Z80/data/languages/z180.sla");
        parse(z180).expect("Failed to parse z180 sla");
    }
    #[test]
    fn test_z80() {
        const z80: &str = include_str!("../Processors/Z80/data/languages/z80.sla");
        parse(z80).expect("Failed to parse z80 sla");
    }
    #[test]
    fn test_eBPF_le() {
        const E_BPF_LE: &str = include_str!("../Processors/eBPF/data/languages/eBPF_le.sla");
        parse(E_BPF_LE).expect("Failed to parse eBPF_le sla");
    }
    #[test]
    fn test_tricore() {
        const TRICORE: &str = include_str!("../Processors/tricore/data/languages/tricore.sla");
        parse(TRICORE).expect("Failed to parse tricore sla");
    }
    #[test]
    fn test_x86_64() {
        const x86_64: &str = include_str!("../Processors/x86/data/languages/x86-64.sla");
        parse(x86_64).expect("Failed to parse x86-64 sla");
    }
    #[test]
    fn test_x86() {
        const X86: &str = include_str!("../Processors/x86/data/languages/x86.sla");
        parse(X86).expect("Failed to parse x86 sla");
    }
}
