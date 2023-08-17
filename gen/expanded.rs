#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod filters {
    pub(crate) mod case {
        use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
        use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
        use liquid_core::{Result, Runtime};
        use liquid_core::{Value, ValueView};
        #[filter(
            name = "shouty_snake_case",
            description = "Convert input to a SHOUTY_SNAKE_CASE",
            parsed(ShoutySnakeCaseFilter)
        )]
        pub struct ShoutySnakeCase;
        #[automatically_derived]
        impl ::core::clone::Clone for ShoutySnakeCase {
            #[inline]
            fn clone(&self) -> ShoutySnakeCase {
                ShoutySnakeCase
            }
        }
        impl ::liquid_core::parser::ParseFilter for ShoutySnakeCase {
            fn parse(
                &self,
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 0 positional arguments"),
                    );
                }
                if let ::std::option::Option::Some(arg) = args.keyword.next() {
                    return ::std::result::Result::Err(::liquid_core::error::Error::with_msg({
                        let res = ::alloc::fmt::format(format_args!(
                            "Unexpected named argument `{0}`",
                            arg.0
                        ));
                        res
                    }));
                }
                #[allow(unknown_lints)]
                #[allow(clippy::box_default)]
                ::std::result::Result::Ok(::std::boxed::Box::new(
                    <ShoutySnakeCaseFilter as ::std::default::Default>::default(),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for ShoutySnakeCase {
            fn name(&self) -> &'static str {
                "shouty_snake_case"
            }
            fn description(&self) -> &'static str {
                "Convert input to a SHOUTY_SNAKE_CASE"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        #[name = "shouty_snake_case"]
        pub struct ShoutySnakeCaseFilter {}
        #[automatically_derived]
        impl ::core::fmt::Debug for ShoutySnakeCaseFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "ShoutySnakeCaseFilter")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ShoutySnakeCaseFilter {
            #[inline]
            fn default() -> ShoutySnakeCaseFilter {
                ShoutySnakeCaseFilter {}
            }
        }
        impl ::std::fmt::Display for ShoutySnakeCaseFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", "shouty_snake_case"))
            }
        }
        impl Filter for ShoutySnakeCaseFilter {
            fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
                Ok(Value::scalar(input.to_kstr().to_shouty_snake_case()))
            }
        }
        #[filter(
            name = "lower_camel_case",
            description = "Convert input to a lowerCamelCase",
            parsed(LowerCamelCaseFilter)
        )]
        pub struct LowerCamelCase;
        #[automatically_derived]
        impl ::core::clone::Clone for LowerCamelCase {
            #[inline]
            fn clone(&self) -> LowerCamelCase {
                LowerCamelCase
            }
        }
        impl ::liquid_core::parser::ParseFilter for LowerCamelCase {
            fn parse(
                &self,
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 0 positional arguments"),
                    );
                }
                if let ::std::option::Option::Some(arg) = args.keyword.next() {
                    return ::std::result::Result::Err(::liquid_core::error::Error::with_msg({
                        let res = ::alloc::fmt::format(format_args!(
                            "Unexpected named argument `{0}`",
                            arg.0
                        ));
                        res
                    }));
                }
                #[allow(unknown_lints)]
                #[allow(clippy::box_default)]
                ::std::result::Result::Ok(::std::boxed::Box::new(
                    <LowerCamelCaseFilter as ::std::default::Default>::default(),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for LowerCamelCase {
            fn name(&self) -> &'static str {
                "lower_camel_case"
            }
            fn description(&self) -> &'static str {
                "Convert input to a lowerCamelCase"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        #[name = "lower_camel_case"]
        pub struct LowerCamelCaseFilter {}
        #[automatically_derived]
        impl ::core::fmt::Debug for LowerCamelCaseFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "LowerCamelCaseFilter")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for LowerCamelCaseFilter {
            #[inline]
            fn default() -> LowerCamelCaseFilter {
                LowerCamelCaseFilter {}
            }
        }
        impl ::std::fmt::Display for LowerCamelCaseFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", "lower_camel_case"))
            }
        }
        impl Filter for LowerCamelCaseFilter {
            fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
                Ok(Value::scalar(input.to_kstr().to_lower_camel_case()))
            }
        }
        #[filter(
            name = "snake_case",
            description = "Convert input to a snake_case",
            parsed(SnakeCaseFilter)
        )]
        pub struct SnakeCase;
        #[automatically_derived]
        impl ::core::clone::Clone for SnakeCase {
            #[inline]
            fn clone(&self) -> SnakeCase {
                SnakeCase
            }
        }
        impl ::liquid_core::parser::ParseFilter for SnakeCase {
            fn parse(
                &self,
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 0 positional arguments"),
                    );
                }
                if let ::std::option::Option::Some(arg) = args.keyword.next() {
                    return ::std::result::Result::Err(::liquid_core::error::Error::with_msg({
                        let res = ::alloc::fmt::format(format_args!(
                            "Unexpected named argument `{0}`",
                            arg.0
                        ));
                        res
                    }));
                }
                #[allow(unknown_lints)]
                #[allow(clippy::box_default)]
                ::std::result::Result::Ok(::std::boxed::Box::new(
                    <SnakeCaseFilter as ::std::default::Default>::default(),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for SnakeCase {
            fn name(&self) -> &'static str {
                "snake_case"
            }
            fn description(&self) -> &'static str {
                "Convert input to a snake_case"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        #[name = "snake_case"]
        pub struct SnakeCaseFilter {}
        #[automatically_derived]
        impl ::core::fmt::Debug for SnakeCaseFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "SnakeCaseFilter")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for SnakeCaseFilter {
            #[inline]
            fn default() -> SnakeCaseFilter {
                SnakeCaseFilter {}
            }
        }
        impl ::std::fmt::Display for SnakeCaseFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", "snake_case"))
            }
        }
        impl Filter for SnakeCaseFilter {
            fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
                Ok(Value::scalar(input.to_kstr().to_snake_case()))
            }
        }
        #[filter(
            name = "upper_camel_case",
            description = "Convert input to a UpperCamelCase",
            parsed(UpperCamelCaseFilter)
        )]
        pub struct UpperCamelCase;
        #[automatically_derived]
        impl ::core::clone::Clone for UpperCamelCase {
            #[inline]
            fn clone(&self) -> UpperCamelCase {
                UpperCamelCase
            }
        }
        impl ::liquid_core::parser::ParseFilter for UpperCamelCase {
            fn parse(
                &self,
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 0 positional arguments"),
                    );
                }
                if let ::std::option::Option::Some(arg) = args.keyword.next() {
                    return ::std::result::Result::Err(::liquid_core::error::Error::with_msg({
                        let res = ::alloc::fmt::format(format_args!(
                            "Unexpected named argument `{0}`",
                            arg.0
                        ));
                        res
                    }));
                }
                #[allow(unknown_lints)]
                #[allow(clippy::box_default)]
                ::std::result::Result::Ok(::std::boxed::Box::new(
                    <UpperCamelCaseFilter as ::std::default::Default>::default(),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for UpperCamelCase {
            fn name(&self) -> &'static str {
                "upper_camel_case"
            }
            fn description(&self) -> &'static str {
                "Convert input to a UpperCamelCase"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        #[name = "upper_camel_case"]
        pub struct UpperCamelCaseFilter {}
        #[automatically_derived]
        impl ::core::fmt::Debug for UpperCamelCaseFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "UpperCamelCaseFilter")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UpperCamelCaseFilter {
            #[inline]
            fn default() -> UpperCamelCaseFilter {
                UpperCamelCaseFilter {}
            }
        }
        impl ::std::fmt::Display for UpperCamelCaseFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", "upper_camel_case"))
            }
        }
        impl Filter for UpperCamelCaseFilter {
            fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
                Ok(Value::scalar(input.to_kstr().to_upper_camel_case()))
            }
        }
    }
    pub(crate) mod collect {
        use super::error::{invalid_argument, invalid_input};
        use liquid_core::{
            Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters,
            ParseFilter,
        };
        use liquid_core::{Expression, Result, Runtime};
        use liquid_core::{Value, ValueView};
        use std::collections::BTreeMap;
        struct CollectArgs {
            #[parameter(description = "")]
            name: Expression,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CollectArgs {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CollectArgs",
                    "name",
                    &&self.name,
                )
            }
        }
        impl<'a> ::liquid_core::parser::FilterParameters<'a> for CollectArgs {
            type EvaluatedFilterParameters = EvaluatedCollectArgs<'a>;
            fn from_args(
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<Self> {
                let name = args.positional.next().ok_or_else(|| {
                    ::liquid_core::error::Error::with_msg("Invalid number of arguments")
                        .context("cause", "expected at least 1 positional argument")
                })?;
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 1 positional argument"),
                    );
                }
                #[allow(clippy::never_loop)]
                while let ::std::option::Option::Some(arg) = args.keyword.next() {
                    match arg.0 {
                        keyword => {
                            return ::std::result::Result::Err(
                                ::liquid_core::error::Error::with_msg({
                                    let res = ::alloc::fmt::format(format_args!(
                                        "Unexpected named argument `{0}`",
                                        keyword
                                    ));
                                    res
                                }),
                            )
                        }
                    }
                }
                Ok(CollectArgs { name })
            }
            fn evaluate(
                &'a self,
                runtime: &'a dyn ::liquid_core::runtime::Runtime,
            ) -> ::liquid_core::error::Result<Self::EvaluatedFilterParameters> {
                let name = self.name.evaluate(runtime)?;
                let name = ::std::result::Result::Ok(name)?;
                Ok(EvaluatedCollectArgs {
                    name,
                    __phantom_data: ::std::marker::PhantomData,
                })
            }
        }
        impl ::liquid_core::parser::FilterParametersReflection for CollectArgs {
            fn positional_parameters() -> &'static [::liquid_core::parser::ParameterReflection] {
                &[::liquid_core::parser::ParameterReflection {
                    name: "name",
                    description: "",
                    is_optional: false,
                }]
            }
            fn keyword_parameters() -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        impl ::std::fmt::Display for CollectArgs {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let positional = [::std::option::Option::Some(&self.name)];
                let keyword = [];
                let positional = positional
                    .iter()
                    .filter_map(
                        |p: &::std::option::Option<&::liquid_core::runtime::Expression>| p.as_ref(),
                    )
                    .map(|p| p.to_string());
                let keyword = keyword.iter().filter_map(
                    |p: &(
                        &str,
                        ::std::option::Option<&::liquid_core::runtime::Expression>,
                    )| match p.1 {
                        ::std::option::Option::Some(p1) => ::std::option::Option::Some({
                            let res = ::alloc::fmt::format(format_args!("{0}: {1}", p.0, p1));
                            res
                        }),
                        ::std::option::Option::None => ::std::option::Option::None,
                    },
                );
                let parameters = positional
                    .chain(keyword)
                    .collect::<::std::vec::Vec<::std::string::String>>()
                    .join(", ");
                f.write_fmt(format_args!("{0}", parameters))
            }
        }
        struct EvaluatedCollectArgs<'a> {
            name: ::liquid_core::model::ValueCow<'a>,
            __phantom_data: ::std::marker::PhantomData<&'a ()>,
        }
        #[filter(
            name = "collect",
            description = "render a collect member of a struct",
            parameters(CollectArgs),
            parsed(CollectFilter)
        )]
        pub struct Collect;
        #[automatically_derived]
        impl ::core::clone::Clone for Collect {
            #[inline]
            fn clone(&self) -> Collect {
                Collect
            }
        }
        impl ::liquid_core::parser::ParseFilter for Collect {
            fn parse(
                &self,
                args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                let args =
                    <CollectArgs as ::liquid_core::parser::FilterParameters>::from_args(args)?;
                Ok(::std::boxed::Box::new(
                    <CollectFilter as ::std::convert::From<CollectArgs>>::from(args),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for Collect {
            fn name(&self) -> &'static str {
                "collect"
            }
            fn description(&self) -> &'static str {
                "render a collect member of a struct"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                < CollectArgs as :: liquid_core :: parser :: FilterParametersReflection > :: positional_parameters ()
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                < CollectArgs as :: liquid_core :: parser :: FilterParametersReflection > :: keyword_parameters ()
            }
        }
        #[name = "collect"]
        pub struct CollectFilter {
            #[parameters]
            args: CollectArgs,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CollectFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CollectFilter",
                    "args",
                    &&self.args,
                )
            }
        }
        impl ::std::convert::From<CollectArgs> for CollectFilter {
            fn from(parameters: CollectArgs) -> Self {
                Self { args: parameters }
            }
        }
        impl ::std::fmt::Display for CollectFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0} : {1}", "collect", &self.args))
            }
        }
        impl Filter for CollectFilter {
            fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
                let args = self.args.evaluate(runtime)?;
                let name = args
                    .name
                    .as_scalar()
                    .ok_or_else(|| invalid_argument("name", "string expected"))?
                    .into_cow_str();
                Ok(input
                    .as_object()
                    .ok_or_else(|| invalid_input("Object expected"))?
                    .iter()
                    .filter_map(|(key, val)| is_type(&name, val).map(|inner| (key, inner)))
                    .collect::<BTreeMap<_, Value>>()
                    .to_value())
            }
        }
        fn is_type(ty: &str, val: &dyn liquid_core::ValueView) -> Option<Value> {
            let inner = val.as_object()?;
            match inner.get("type")?.as_scalar()?.into_cow_str().as_ref() {
                n if n == ty => Some(inner.to_value()),
                _ => None,
            }
        }
    }
    pub(crate) mod error {
        use liquid_core::Error;
        pub(super) fn invalid_input<S>(cause: S) -> Error
        where
            S: Into<liquid_core::model::KString>,
        {
            Error::with_msg("Invalid input").context("cause", cause)
        }
        pub(super) fn invalid_argument<S>(argument: S, cause: S) -> Error
        where
            S: Into<liquid_core::model::KString>,
        {
            Error::with_msg("Invalid argument")
                .context("argument", argument)
                .context("cause", cause)
        }
    }
    pub(crate) mod field {
        use super::error::invalid_argument;
        use crate::language::Language;
        use liquid_core::Error;
        use liquid_core::{
            Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters,
            ParseFilter,
        };
        use liquid_core::{Expression, Result, Runtime};
        use liquid_core::{Value, ValueView};
        use seedle_parser::*;
        use serde::Deserialize;
        use std::fmt;
        struct FieldJsonArgs {
            language: Language,
            public: bool,
            required: bool,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for FieldJsonArgs {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "language" => _serde::__private::Ok(__Field::__field0),
                                "public" => _serde::__private::Ok(__Field::__field1),
                                "required" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"language" => _serde::__private::Ok(__Field::__field0),
                                b"public" => _serde::__private::Ok(__Field::__field1),
                                b"required" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<FieldJsonArgs>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = FieldJsonArgs;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct FieldJsonArgs",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<Language>(
                                &mut __seq,
                            )? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct FieldJsonArgs with 3 elements",
                                        ),
                                    )
                                }
                            };
                            let __field1 =
                                match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct FieldJsonArgs with 3 elements",
                                            ),
                                        )
                                    }
                                };
                            let __field2 =
                                match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct FieldJsonArgs with 3 elements",
                                            ),
                                        )
                                    }
                                };
                            _serde::__private::Ok(FieldJsonArgs {
                                language: __field0,
                                public: __field1,
                                required: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Language> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "language",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Language>(
                                                &mut __map,
                                            )?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "public",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "required",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        )?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("language")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("public")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("required")?
                                }
                            };
                            _serde::__private::Ok(FieldJsonArgs {
                                language: __field0,
                                public: __field1,
                                required: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["language", "public", "required"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "FieldJsonArgs",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<FieldJsonArgs>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        struct FieldArgs {
            #[parameter(description = "Langauge (C,Rust,Typescript)")]
            language: Expression,
            #[parameter(description = "Is this member public or private?")]
            public: Expression,
            #[parameter(description = "Is this member required or optional?")]
            required: Expression,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FieldArgs {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "FieldArgs",
                    "language",
                    &self.language,
                    "public",
                    &self.public,
                    "required",
                    &&self.required,
                )
            }
        }
        impl<'a> ::liquid_core::parser::FilterParameters<'a> for FieldArgs {
            type EvaluatedFilterParameters = EvaluatedFieldArgs<'a>;
            fn from_args(
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<Self> {
                let language = args.positional.next().ok_or_else(|| {
                    ::liquid_core::error::Error::with_msg("Invalid number of arguments")
                        .context("cause", "expected at least 3 positional arguments")
                })?;
                let public = args.positional.next().ok_or_else(|| {
                    ::liquid_core::error::Error::with_msg("Invalid number of arguments")
                        .context("cause", "expected at least 3 positional arguments")
                })?;
                let required = args.positional.next().ok_or_else(|| {
                    ::liquid_core::error::Error::with_msg("Invalid number of arguments")
                        .context("cause", "expected at least 3 positional arguments")
                })?;
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 3 positional arguments"),
                    );
                }
                #[allow(clippy::never_loop)]
                while let ::std::option::Option::Some(arg) = args.keyword.next() {
                    match arg.0 {
                        keyword => {
                            return ::std::result::Result::Err(
                                ::liquid_core::error::Error::with_msg({
                                    let res = ::alloc::fmt::format(format_args!(
                                        "Unexpected named argument `{0}`",
                                        keyword
                                    ));
                                    res
                                }),
                            )
                        }
                    }
                }
                Ok(FieldArgs {
                    language,
                    public,
                    required,
                })
            }
            fn evaluate(
                &'a self,
                runtime: &'a dyn ::liquid_core::runtime::Runtime,
            ) -> ::liquid_core::error::Result<Self::EvaluatedFilterParameters> {
                let language = self.language.evaluate(runtime)?;
                let language = ::std::result::Result::Ok(language)?;
                let public = self.public.evaluate(runtime)?;
                let public = ::std::result::Result::Ok(public)?;
                let required = self.required.evaluate(runtime)?;
                let required = ::std::result::Result::Ok(required)?;
                Ok(EvaluatedFieldArgs {
                    language,
                    public,
                    required,
                    __phantom_data: ::std::marker::PhantomData,
                })
            }
        }
        impl ::liquid_core::parser::FilterParametersReflection for FieldArgs {
            fn positional_parameters() -> &'static [::liquid_core::parser::ParameterReflection] {
                &[
                    ::liquid_core::parser::ParameterReflection {
                        name: "language",
                        description: "Langauge (C,Rust,Typescript)",
                        is_optional: false,
                    },
                    ::liquid_core::parser::ParameterReflection {
                        name: "public",
                        description: "Is this member public or private?",
                        is_optional: false,
                    },
                    ::liquid_core::parser::ParameterReflection {
                        name: "required",
                        description: "Is this member required or optional?",
                        is_optional: false,
                    },
                ]
            }
            fn keyword_parameters() -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        impl ::std::fmt::Display for FieldArgs {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let positional = [
                    ::std::option::Option::Some(&self.language),
                    ::std::option::Option::Some(&self.public),
                    ::std::option::Option::Some(&self.required),
                ];
                let keyword = [];
                let positional = positional
                    .iter()
                    .filter_map(
                        |p: &::std::option::Option<&::liquid_core::runtime::Expression>| p.as_ref(),
                    )
                    .map(|p| p.to_string());
                let keyword = keyword.iter().filter_map(
                    |p: &(
                        &str,
                        ::std::option::Option<&::liquid_core::runtime::Expression>,
                    )| match p.1 {
                        ::std::option::Option::Some(p1) => ::std::option::Option::Some({
                            let res = ::alloc::fmt::format(format_args!("{0}: {1}", p.0, p1));
                            res
                        }),
                        ::std::option::Option::None => ::std::option::Option::None,
                    },
                );
                let parameters = positional
                    .chain(keyword)
                    .collect::<::std::vec::Vec<::std::string::String>>()
                    .join(", ");
                f.write_fmt(format_args!("{0}", parameters))
            }
        }
        struct EvaluatedFieldArgs<'a> {
            language: ::liquid_core::model::ValueCow<'a>,
            public: ::liquid_core::model::ValueCow<'a>,
            required: ::liquid_core::model::ValueCow<'a>,
            __phantom_data: ::std::marker::PhantomData<&'a ()>,
        }
        #[filter(
            name = "field",
            description = "render a field member of a struct",
            parameters(FieldArgs),
            parsed(FieldFilter)
        )]
        pub struct Field;
        #[automatically_derived]
        impl ::core::clone::Clone for Field {
            #[inline]
            fn clone(&self) -> Field {
                Field
            }
        }
        impl ::liquid_core::parser::ParseFilter for Field {
            fn parse(
                &self,
                args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                let args = <FieldArgs as ::liquid_core::parser::FilterParameters>::from_args(args)?;
                Ok(::std::boxed::Box::new(
                    <FieldFilter as ::std::convert::From<FieldArgs>>::from(args),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for Field {
            fn name(&self) -> &'static str {
                "field"
            }
            fn description(&self) -> &'static str {
                "render a field member of a struct"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                < FieldArgs as :: liquid_core :: parser :: FilterParametersReflection > :: positional_parameters ()
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                <FieldArgs as ::liquid_core::parser::FilterParametersReflection>::keyword_parameters(
                )
            }
        }
        #[name = "field"]
        pub struct FieldFilter {
            #[parameters]
            args: FieldArgs,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FieldFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "FieldFilter",
                    "args",
                    &&self.args,
                )
            }
        }
        impl ::std::convert::From<FieldArgs> for FieldFilter {
            fn from(parameters: FieldArgs) -> Self {
                Self { args: parameters }
            }
        }
        impl ::std::fmt::Display for FieldFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0} : {1}", "field", &self.args))
            }
        }
        impl Filter for FieldFilter {
            fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
                let args = self.args.evaluate(runtime)?;
                let node = LinkedKeyVal::try_from(input.to_value())
                    .map_err(|e| Error::with_msg("invalid argument").cause(e))?;
                let language = &Language::try_from(args.language.to_value())?;
                let public = args
                    .public
                    .as_scalar()
                    .ok_or_else(|| invalid_argument("public", "Boolean expected"))?
                    .to_bool()
                    .ok_or_else(|| invalid_argument("public", "Boolean expected"))?;
                let required = args
                    .required
                    .as_scalar()
                    .ok_or_else(|| invalid_argument("required", "Boolean expected"))?
                    .to_bool()
                    .ok_or_else(|| invalid_argument("required", "Boolean expected"))?;
                let field = FieldFormatter {
                    language,
                    required,
                    public,
                    node: &node,
                };
                Ok(Value::Scalar(
                    {
                        let res = ::alloc::fmt::format(format_args!("{0}", field));
                        res
                    }
                    .into(),
                ))
            }
        }
        struct FieldFormatter<'s> {
            language: &'s Language,
            required: bool,
            public: bool,
            node: &'s LinkedKeyVal,
        }
        impl<'s> fmt::Display for FieldFormatter<'s> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let language = self.language;
                let key = language.fieldify(self.node.key());
                let node = self.node.val();
                if self.public {
                    f.write_fmt(format_args!("pub {0}: ", key))?;
                } else {
                    f.write_fmt(format_args!("{0}: ", key))?;
                }
                match self.required {
                    true => f.write_fmt(format_args!("{0}", NodeFormatter { language, node })),
                    false => f.write_fmt(format_args!(
                        "Option<{0}>",
                        NodeFormatter { language, node }
                    )),
                }
            }
        }
        struct NodeFormatter<'s> {
            language: &'s Language,
            node: &'s LinkedNode,
        }
        impl<'s> fmt::Display for NodeFormatter<'s> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let language = self.language;
                match self.node {
                    LinkedNode::Primative(p) => PrimativeFormatter(p).fmt(f),
                    LinkedNode::Array(a) => ArrayFormatter { language, node: a }.fmt(f),
                    LinkedNode::ForeignStruct(s) => StructFormatter { language, node: s }.fmt(f),
                    _ => Err(fmt::Error),
                }
            }
        }
        struct PrimativeFormatter<'s>(&'s ConstrainedPrimative);
        impl<'s> fmt::Display for PrimativeFormatter<'s> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    ConstrainedPrimative::U8 => f.write_fmt(format_args!("u8")),
                    ConstrainedPrimative::U16 => f.write_fmt(format_args!("u16")),
                    ConstrainedPrimative::U32 => f.write_fmt(format_args!("u32")),
                    ConstrainedPrimative::U64 => f.write_fmt(format_args!("u64")),
                    ConstrainedPrimative::I8 => f.write_fmt(format_args!("i8")),
                    ConstrainedPrimative::I16 => f.write_fmt(format_args!("i16")),
                    ConstrainedPrimative::I32 => f.write_fmt(format_args!("i32")),
                    ConstrainedPrimative::I64 => f.write_fmt(format_args!("i64")),
                    ConstrainedPrimative::Bool => f.write_fmt(format_args!("bool")),
                    ConstrainedPrimative::Str(n) => f.write_fmt(format_args!("[u8; {0}]", n)),
                    ConstrainedPrimative::Bytes(n) => f.write_fmt(format_args!("[u8; {0}]", n)),
                }
            }
        }
        struct StructFormatter<'s> {
            language: &'s Language,
            node: &'s str,
        }
        impl<'s> fmt::Display for StructFormatter<'s> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(format_args!("{0}", self.language.structify(self.node)))
            }
        }
        struct ArrayFormatter<'s> {
            language: &'s Language,
            node: &'s LinkedArray,
        }
        impl<'s> fmt::Display for ArrayFormatter<'s> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let formatter = NodeFormatter {
                    language: self.language,
                    node: self.node.ty.as_ref(),
                };
                f.write_fmt(format_args!("[{0}; {1}]", formatter, self.node.len))
            }
        }
    }
    pub(crate) mod json {
        use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
        use liquid_core::{Error, Result, Runtime};
        use liquid_core::{Value, ValueView};
        #[filter(
            name = "json",
            description = "Convert a JSON string into a liquid object",
            parsed(JsonFilter)
        )]
        pub struct Json;
        #[automatically_derived]
        impl ::core::clone::Clone for Json {
            #[inline]
            fn clone(&self) -> Json {
                Json
            }
        }
        impl ::liquid_core::parser::ParseFilter for Json {
            fn parse(
                &self,
                mut args: ::liquid_core::parser::FilterArguments,
            ) -> ::liquid_core::error::Result<::std::boxed::Box<dyn ::liquid_core::parser::Filter>>
            {
                if let ::std::option::Option::Some(arg) = args.positional.next() {
                    return ::std::result::Result::Err(
                        ::liquid_core::error::Error::with_msg(
                            "Invalid number of positional arguments",
                        )
                        .context("cause", "expected at most 0 positional arguments"),
                    );
                }
                if let ::std::option::Option::Some(arg) = args.keyword.next() {
                    return ::std::result::Result::Err(::liquid_core::error::Error::with_msg({
                        let res = ::alloc::fmt::format(format_args!(
                            "Unexpected named argument `{0}`",
                            arg.0
                        ));
                        res
                    }));
                }
                #[allow(unknown_lints)]
                #[allow(clippy::box_default)]
                ::std::result::Result::Ok(::std::boxed::Box::new(
                    <JsonFilter as ::std::default::Default>::default(),
                ))
            }
            fn reflection(&self) -> &dyn ::liquid_core::parser::FilterReflection {
                self
            }
        }
        impl ::liquid_core::parser::FilterReflection for Json {
            fn name(&self) -> &'static str {
                "json"
            }
            fn description(&self) -> &'static str {
                "Convert a JSON string into a liquid object"
            }
            fn positional_parameters(
                &self,
            ) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
            fn keyword_parameters(&self) -> &'static [::liquid_core::parser::ParameterReflection] {
                &[]
            }
        }
        #[name = "shouty_snake_case"]
        pub struct JsonFilter {}
        #[automatically_derived]
        impl ::core::fmt::Debug for JsonFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "JsonFilter")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for JsonFilter {
            #[inline]
            fn default() -> JsonFilter {
                JsonFilter {}
            }
        }
        impl ::std::fmt::Display for JsonFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", "shouty_snake_case"))
            }
        }
        impl Filter for JsonFilter {
            fn evaluate(&self, input: &dyn ValueView, _: &dyn Runtime) -> Result<Value> {
                serde_json::from_str(&input.to_kstr().as_str())
                    .map_err(|e| Error::with_msg(e.to_string()))
            }
        }
    }
}
mod language {
    use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
    use liquid_core::model::ScalarCow;
    use liquid_core::{Error, Value, ValueView};
    use serde::Deserialize;
    #[serde(rename_all = "camelCase")]
    pub enum Language {
        C,
        Typescript,
        Rust,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Language {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Language::C => "C",
                    Language::Typescript => "Typescript",
                    Language::Rust => "Rust",
                },
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Language {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "c" => _serde::__private::Ok(__Field::__field0),
                            "typescript" => _serde::__private::Ok(__Field::__field1),
                            "rust" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"c" => _serde::__private::Ok(__Field::__field0),
                            b"typescript" => _serde::__private::Ok(__Field::__field1),
                            b"rust" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Language>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Language;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum Language")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Language::C)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Language::Typescript)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Language::Rust)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["c", "typescript", "rust"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Language",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Language>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Language {
        pub fn structify(&self, name: &str) -> String {
            match self {
                Language::C => name.to_snake_case(),
                _ => name.to_upper_camel_case(),
            }
        }
        pub fn fieldify(&self, name: &str) -> String {
            match self {
                Language::C => name.to_snake_case(),
                Language::Rust => name.to_snake_case(),
                Language::Typescript => name.to_lower_camel_case(),
            }
        }
    }
    impl From<Language> for &'static str {
        fn from(value: Language) -> Self {
            match value {
                Language::C => "c",
                Language::Rust => "rust",
                Language::Typescript => "typescript",
            }
        }
    }
    impl TryFrom<&str> for Language {
        type Error = Error;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "c" => Ok(Language::C),
                "C" => Ok(Language::C),
                "rust" => Ok(Language::Rust),
                "Rust" => Ok(Language::Rust),
                "RUST" => Ok(Language::Rust),
                "typescript" => Ok(Language::Typescript),
                "Typescript" => Ok(Language::Typescript),
                "TYPESCRIPT" => Ok(Language::Typescript),
                s => Err(Error::with_msg({
                    let res = ::alloc::fmt::format(format_args!("invalid language {0}", s));
                    res
                })),
            }
        }
    }
    impl From<Language> for ScalarCow<'static> {
        fn from(value: Language) -> Self {
            let s: &'static str = value.into();
            ScalarCow::from(s)
        }
    }
    impl From<Language> for Value {
        fn from(value: Language) -> Self {
            Value::Scalar(value.into())
        }
    }
    impl TryFrom<Value> for Language {
        type Error = Error;
        fn try_from(value: Value) -> Result<Self, Self::Error> {
            Language::try_from(value.to_kstr().as_str())
        }
    }
}
pub mod templates {
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub struct C {
        __private_field: (),
    }
    #[doc(hidden)]
    pub static C: C = C {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for C {
        type Target = &'static str;
        fn deref(&self) -> &&'static str {
            #[inline(always)]
            fn __static_ref_initialize() -> &'static str {
                "pub use minicbor::encode::write::Cursor;\npub use minicbor::{self, CborLen, Decode, Decoder, Encode, Encoder};\n{% assign structs = types | collect: \"struct\" -%}\n\n{%- comment -%}\nRender all the structures. Must loop through each struct. Inside each struct\nmust loop through each member and render a field.\n\nIn C we use snake_case for struct types.\n\nNOTE in future we might want to rename the members key to members instead of \nvalue. We used \"value\" because all objects have \"type\" and \"value\" fields...\n{%- endcomment -%}\n{%- for item in structs -%}\n\t{%- comment -%}\n\t#[repr(C)]\n\t#[derive(Copy, Clone, CborLen, Encode, Decode)]\n\t#[allow(non_camel_case_types)]\n\t{%- endcomment -%}\n\tpub struct {{ item[0] | snake_case }} {\n\t\t{%- for member in item[1].value -%}\n\t\t\t{{ member | field: \"C\", false, true }},\n\t\t{%- endfor -%}\n\t}\n{%- endfor -%}\n"
            }
            #[inline(always)]
            fn __stability() -> &'static &'static str {
                static LAZY: ::lazy_static::lazy::Lazy<&'static str> =
                    ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for C {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
}
use liquid::Parser as LiquidParser;
use liquid_core::model::KString;
use liquid_core::{Object, Value, ValueView};
use seedle_parser::FlattenError;
use std::collections::BTreeMap;
pub struct Parser {
    context: Object,
    parser: LiquidParser,
}
impl Parser {
    pub fn build() -> liquid_core::Result<Parser> {
        let context = Object::new();
        liquid::ParserBuilder::with_stdlib()
            .filter(crate::filters::case::LowerCamelCase)
            .filter(crate::filters::case::UpperCamelCase)
            .filter(crate::filters::case::SnakeCase)
            .filter(crate::filters::case::ShoutySnakeCase)
            .filter(crate::filters::collect::Collect)
            .filter(crate::filters::field::Field)
            .build()
            .map(|parser| Parser { parser, context })
    }
    pub fn load_cddl<K: Into<KString>>(&mut self, key: K, cddl: &str) -> Result<(), FlattenError> {
        let nodes = seedle_parser::parse(cddl)?
            .into_iter()
            .map(|(k, v)| (k, Value::from(v)))
            .collect::<BTreeMap<_, Value>>();
        self.context.insert(key.into(), nodes.to_value());
        Ok(())
    }
    pub fn render(&self, text: &str) -> liquid_core::Result<String> {
        self.parser.parse(text)?.render(&self.context)
    }
    pub fn render_to(
        &self,
        writer: &mut dyn std::io::Write,
        text: &str,
    ) -> liquid_core::Result<()> {
        self.parser.parse(text)?.render_to(writer, &self.context)
    }
}
