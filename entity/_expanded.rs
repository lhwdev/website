#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod post {
    use rocket::serde::{Deserialize, Serialize};
    use sea_orm::entity::prelude::*;
    #[serde(crate = "rocket::serde")]
    pub struct ModelCreatePatch {
        pub title: String,
        pub text: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for ModelCreatePatch {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                "ModelCreatePatch", false as usize + 1 + 1) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "title", &self.title) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "text", &self.text) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for ModelCreatePatch {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "title" => _serde::__private::Ok(__Field::__field0),
                                "text" => _serde::__private::Ok(__Field::__field1),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"title" => _serde::__private::Ok(__Field::__field0),
                                b"text" => _serde::__private::Ok(__Field::__field1),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ModelCreatePatch>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ModelCreatePatch;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "struct ModelCreatePatch")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"struct ModelCreatePatch with 2 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                    &"struct ModelCreatePatch with 2 elements"));
                                    }
                                };
                            _serde::__private::Ok(ModelCreatePatch {
                                    title: __field0,
                                    text: __field1,
                                })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("title"));
                                            }
                                        __field0 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("text"));
                                            }
                                        __field1 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("title") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("text") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            _serde::__private::Ok(ModelCreatePatch {
                                    title: __field0,
                                    text: __field1,
                                })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["title", "text"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                        "ModelCreatePatch", FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ModelCreatePatch>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    #[serde(crate = "rocket::serde")]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: u32,
        pub title: String,
        #[sea_orm(column_type = "Text")]
        pub text: String,
        #[sea_orm()]
        pub author: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Model {
        #[inline]
        fn clone(&self) -> Model {
            match *self {
                Self {
                    id: ref __self_0_0,
                    title: ref __self_0_1,
                    text: ref __self_0_2,
                    author: ref __self_0_3 } =>
                    Model {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        title: ::core::clone::Clone::clone(&(*__self_0_1)),
                        text: ::core::clone::Clone::clone(&(*__self_0_2)),
                        author: ::core::clone::Clone::clone(&(*__self_0_3)),
                    },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Model {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    id: ref __self_0_0,
                    title: ref __self_0_1,
                    text: ref __self_0_2,
                    author: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Model");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "title", &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "text",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "author", &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Model {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Model {
        #[inline]
        fn eq(&self, other: &Model) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    title: ref __self_1_1,
                    text: ref __self_1_2,
                    author: ref __self_1_3 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            title: ref __self_0_1,
                            text: ref __self_0_2,
                            author: ref __self_0_3 } =>
                            (*__self_0_0) == (*__self_1_0) &&
                                        (*__self_0_1) == (*__self_1_1) &&
                                    (*__self_0_2) == (*__self_1_2) &&
                                (*__self_0_3) == (*__self_1_3),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &Model) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    title: ref __self_1_1,
                    text: ref __self_1_2,
                    author: ref __self_1_3 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            title: ref __self_0_1,
                            text: ref __self_0_2,
                            author: ref __self_0_3 } =>
                            (*__self_0_0) != (*__self_1_0) ||
                                        (*__self_0_1) != (*__self_1_1) ||
                                    (*__self_0_2) != (*__self_1_2) ||
                                (*__self_0_3) != (*__self_1_3),
                    },
            }
        }
    }
    pub enum Column { Id, Title, Text, Author, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Column { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Column {
        #[inline]
        fn clone(&self) -> Column { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Column {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Column::Id,) => {
                    ::core::fmt::Formatter::write_str(f, "Id")
                }
                (&Column::Title,) => {
                    ::core::fmt::Formatter::write_str(f, "Title")
                }
                (&Column::Text,) => {
                    ::core::fmt::Formatter::write_str(f, "Text")
                }
                (&Column::Author,) => {
                    ::core::fmt::Formatter::write_str(f, "Author")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct ColumnIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl ColumnIter {
        fn get(&self, idx: usize) -> Option<Column> {
            match idx {
                0usize => ::core::option::Option::Some(Column::Id),
                1usize => ::core::option::Option::Some(Column::Title),
                2usize => ::core::option::Option::Some(Column::Text),
                3usize => ::core::option::Option::Some(Column::Author),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for Column {
        type Iterator = ColumnIter;
        fn iter() -> ColumnIter {
            ColumnIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for ColumnIter {
        type Item = Column;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 4usize {
                        0
                    } else { 4usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 4usize {
                    self.idx = 4usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for ColumnIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for ColumnIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 4usize {
                    self.back_idx = 4usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(4usize - self.back_idx)
               }
        }
    }
    impl Clone for ColumnIter {
        fn clone(&self) -> ColumnIter {
            ColumnIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl Column {
        fn default_as_str(&self) -> &str {
            match self {
                Self::Id => "id",
                Self::Title => "title",
                Self::Text => "text",
                Self::Author => "author",
            }
        }
    }
    #[automatically_derived]
    impl std::str::FromStr for Column {
        type Err = sea_orm::ColumnFromStrErr;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            match s {
                "id" | "id" => Ok(Column::Id),
                "title" | "title" => Ok(Column::Title),
                "text" | "text" => Ok(Column::Text),
                "author" | "author" => Ok(Column::Author),
                _ =>
                    Err(sea_orm::ColumnFromStrErr({
                                let res =
                                    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["Failed to parse \'",
                                                        "\' as `", "`"],
                                            &match (&s, &"Column") {
                                                    args =>
                                                        [::core::fmt::ArgumentV1::new_display(args.0),
                                                                ::core::fmt::ArgumentV1::new_display(args.1)],
                                                }));
                                res
                            })),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::Iden for Column {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for Column {
        fn as_str(&self) -> &str { self.default_as_str() }
    }
    #[automatically_derived]
    impl sea_orm::prelude::ColumnTrait for Column {
        type EntityName = Entity;
        fn def(&self) -> sea_orm::prelude::ColumnDef {
            match self {
                Self::Id => sea_orm::prelude::ColumnType::Unsigned.def(),
                Self::Title =>
                    sea_orm::prelude::ColumnType::String(None).def(),
                Self::Text => sea_orm::prelude::ColumnType::Text.def(),
                Self::Author => sea_orm::prelude::ColumnType::Unsigned.def(),
            }
        }
    }
    pub struct Entity;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Entity { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Entity {
        #[inline]
        fn clone(&self) -> Entity { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Entity {
        #[inline]
        fn default() -> Entity { Entity {} }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Entity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self => { ::core::fmt::Formatter::write_str(f, "Entity") }
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::entity::EntityTrait for Entity {
        type Model = Model;
        type Column = Column;
        type PrimaryKey = PrimaryKey;
        type Relation = Relation;
    }
    #[automatically_derived]
    impl sea_orm::Iden for Entity {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for Entity {
        fn as_str(&self) -> &str {
            <Self as sea_orm::EntityName>::table_name(self)
        }
    }
    #[automatically_derived]
    impl sea_orm::prelude::EntityName for Entity {
        fn schema_name(&self) -> Option<&str> { None }
        fn table_name(&self) -> &str { "posts" }
    }
    pub enum PrimaryKey { Id, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PrimaryKey { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PrimaryKey {
        #[inline]
        fn clone(&self) -> PrimaryKey { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PrimaryKey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&PrimaryKey::Id,) => {
                    ::core::fmt::Formatter::write_str(f, "Id")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct PrimaryKeyIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl PrimaryKeyIter {
        fn get(&self, idx: usize) -> Option<PrimaryKey> {
            match idx {
                0usize => ::core::option::Option::Some(PrimaryKey::Id),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for PrimaryKey {
        type Iterator = PrimaryKeyIter;
        fn iter() -> PrimaryKeyIter {
            PrimaryKeyIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for PrimaryKeyIter {
        type Item = PrimaryKey;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 1usize {
                        0
                    } else { 1usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 1usize {
                    self.idx = 1usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for PrimaryKeyIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for PrimaryKeyIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 1usize {
                    self.back_idx = 1usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(1usize - self.back_idx)
               }
        }
    }
    impl Clone for PrimaryKeyIter {
        fn clone(&self) -> PrimaryKeyIter {
            PrimaryKeyIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::Iden for PrimaryKey {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for PrimaryKey {
        fn as_str(&self) -> &str { match self { Self::Id => "id", } }
    }
    #[automatically_derived]
    impl sea_orm::PrimaryKeyToColumn for PrimaryKey {
        type Column = Column;
        fn into_column(self) -> Self::Column {
            match self { Self::Id => Self::Column::Id, }
        }
        fn from_column(col: Self::Column) -> Option<Self> {
            match col { Self::Column::Id => Some(Self::Id), _ => None, }
        }
    }
    #[automatically_derived]
    impl PrimaryKeyTrait for PrimaryKey {
        type ValueType = u32;
        fn auto_increment() -> bool { true }
    }
    #[automatically_derived]
    impl sea_orm::FromQueryResult for Model {
        fn from_query_result(row: &sea_orm::QueryResult, pre: &str)
            -> std::result::Result<Self, sea_orm::DbErr> {
            Ok(Self {
                    id: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Id).into())?,
                    title: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Title).into())?,
                    text: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Text).into())?,
                    author: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Author).into())?,
                })
        }
    }
    #[automatically_derived]
    impl sea_orm::ModelTrait for Model {
        type Entity = Entity;
        fn get(&self,
            c: <Self::Entity as sea_orm::entity::EntityTrait>::Column)
            -> sea_orm::Value {
            match c {
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id =>
                    self.id.clone().into(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Title
                    => self.title.clone().into(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Text
                    => self.text.clone().into(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Author
                    => self.author.clone().into(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["field does not exist on Model"],
                            &[])),
            }
        }
        fn set(&mut self,
            c: <Self::Entity as sea_orm::entity::EntityTrait>::Column,
            v: sea_orm::Value) {
            match c {
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id =>
                    self.id = v.unwrap(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Title
                    => self.title = v.unwrap(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Text
                    => self.text = v.unwrap(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Author
                    => self.author = v.unwrap(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["field does not exist on Model"],
                            &[])),
            }
        }
    }
    pub struct ActiveModel {
        pub id: sea_orm::ActiveValue<u32>,
        pub title: sea_orm::ActiveValue<String>,
        pub text: sea_orm::ActiveValue<String>,
        pub author: sea_orm::ActiveValue<u32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ActiveModel {
        #[inline]
        fn clone(&self) -> ActiveModel {
            match *self {
                Self {
                    id: ref __self_0_0,
                    title: ref __self_0_1,
                    text: ref __self_0_2,
                    author: ref __self_0_3 } =>
                    ActiveModel {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        title: ::core::clone::Clone::clone(&(*__self_0_1)),
                        text: ::core::clone::Clone::clone(&(*__self_0_2)),
                        author: ::core::clone::Clone::clone(&(*__self_0_3)),
                    },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ActiveModel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    id: ref __self_0_0,
                    title: ref __self_0_1,
                    text: ref __self_0_2,
                    author: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ActiveModel");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "title", &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "text",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "author", &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ActiveModel {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ActiveModel {
        #[inline]
        fn eq(&self, other: &ActiveModel) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    title: ref __self_1_1,
                    text: ref __self_1_2,
                    author: ref __self_1_3 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            title: ref __self_0_1,
                            text: ref __self_0_2,
                            author: ref __self_0_3 } =>
                            (*__self_0_0) == (*__self_1_0) &&
                                        (*__self_0_1) == (*__self_1_1) &&
                                    (*__self_0_2) == (*__self_1_2) &&
                                (*__self_0_3) == (*__self_1_3),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &ActiveModel) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    title: ref __self_1_1,
                    text: ref __self_1_2,
                    author: ref __self_1_3 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            title: ref __self_0_1,
                            text: ref __self_0_2,
                            author: ref __self_0_3 } =>
                            (*__self_0_0) != (*__self_1_0) ||
                                        (*__self_0_1) != (*__self_1_1) ||
                                    (*__self_0_2) != (*__self_1_2) ||
                                (*__self_0_3) != (*__self_1_3),
                    },
            }
        }
    }
    #[automatically_derived]
    impl std::default::Default for ActiveModel {
        fn default() -> Self { <Self as sea_orm::ActiveModelBehavior>::new() }
    }
    #[automatically_derived]
    impl std::convert::From<<Entity as EntityTrait>::Model> for ActiveModel {
        fn from(m: <Entity as EntityTrait>::Model) -> Self {
            Self {
                id: sea_orm::ActiveValue::unchanged(m.id),
                title: sea_orm::ActiveValue::unchanged(m.title),
                text: sea_orm::ActiveValue::unchanged(m.text),
                author: sea_orm::ActiveValue::unchanged(m.author),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::IntoActiveModel<ActiveModel> for
        <Entity as EntityTrait>::Model {
        fn into_active_model(self) -> ActiveModel { self.into() }
    }
    #[automatically_derived]
    impl sea_orm::ActiveModelTrait for ActiveModel {
        type Entity = Entity;
        fn take(&mut self, c: <Self::Entity as EntityTrait>::Column)
            -> sea_orm::ActiveValue<sea_orm::Value> {
            match c {
                <Self::Entity as EntityTrait>::Column::Id => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.id);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Title => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.title);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Text => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.text);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Author => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.author);
                    value.into_wrapped_value()
                }
                _ => sea_orm::ActiveValue::not_set(),
            }
        }
        fn get(&self, c: <Self::Entity as EntityTrait>::Column)
            -> sea_orm::ActiveValue<sea_orm::Value> {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Title =>
                    self.title.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Text =>
                    self.text.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Author =>
                    self.author.clone().into_wrapped_value(),
                _ => sea_orm::ActiveValue::not_set(),
            }
        }
        fn set(&mut self, c: <Self::Entity as EntityTrait>::Column,
            v: sea_orm::Value) {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Title =>
                    self.title = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Text =>
                    self.text = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Author =>
                    self.author = sea_orm::ActiveValue::set(v.unwrap()),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["This ActiveModel does not have this field"],
                            &[])),
            }
        }
        fn not_set(&mut self, c: <Self::Entity as EntityTrait>::Column) {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Title =>
                    self.title = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Text =>
                    self.text = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Author =>
                    self.author = sea_orm::ActiveValue::not_set(),
                _ => {}
            }
        }
        fn is_not_set(&self, c: <Self::Entity as EntityTrait>::Column)
            -> bool {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Title =>
                    self.title.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Text =>
                    self.text.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Author =>
                    self.author.is_not_set(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["This ActiveModel does not have this field"],
                            &[])),
            }
        }
        fn default() -> Self {
            Self {
                id: sea_orm::ActiveValue::not_set(),
                title: sea_orm::ActiveValue::not_set(),
                text: sea_orm::ActiveValue::not_set(),
                author: sea_orm::ActiveValue::not_set(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for Model {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                "Model", false as usize + 1 + 1 + 1 + 1) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "id", &self.id) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "title", &self.title) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "text", &self.text) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "author", &self.author) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for Model {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "title" => _serde::__private::Ok(__Field::__field1),
                                "text" => _serde::__private::Ok(__Field::__field2),
                                "author" => _serde::__private::Ok(__Field::__field3),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"title" => _serde::__private::Ok(__Field::__field1),
                                b"text" => _serde::__private::Ok(__Field::__field2),
                                b"author" => _serde::__private::Ok(__Field::__field3),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Model>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Model;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "struct Model")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"struct Model with 4 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                    &"struct Model with 4 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(2usize,
                                                    &"struct Model with 4 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(3usize,
                                                    &"struct Model with 4 elements"));
                                    }
                                };
                            _serde::__private::Ok(Model {
                                    id: __field0,
                                    title: __field1,
                                    text: __field2,
                                    author: __field3,
                                })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("id"));
                                            }
                                        __field0 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("title"));
                                            }
                                        __field1 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("text"));
                                            }
                                        __field2 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("author"));
                                            }
                                        __field3 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("title") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("text") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field3 =
                                match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("author") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            _serde::__private::Ok(Model {
                                    id: __field0,
                                    title: __field1,
                                    text: __field2,
                                    author: __field3,
                                })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["id", "title", "text", "author"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                        "Model", FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Model>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    pub enum Relation {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Relation { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Relation {
        #[inline]
        fn clone(&self) -> Relation { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Relation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            unsafe { ::core::intrinsics::unreachable() }
        }
    }
    #[allow(missing_docs)]
    pub struct RelationIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl RelationIter {
        fn get(&self, idx: usize) -> Option<Relation> {
            match idx { _ => ::core::option::Option::None, }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for Relation {
        type Iterator = RelationIter;
        fn iter() -> RelationIter {
            RelationIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for RelationIter {
        type Item = Relation;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 0usize {
                        0
                    } else { 0usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 0usize {
                    self.idx = 0usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for RelationIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for RelationIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 0usize {
                    self.back_idx = 0usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(0usize - self.back_idx)
               }
        }
    }
    impl Clone for RelationIter {
        fn clone(&self) -> RelationIter {
            RelationIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::entity::RelationTrait for Relation {
        fn def(&self) -> sea_orm::entity::RelationDef {
            match self {
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["No RelationDef for Relation"],
                            &[])),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}
pub mod user {
    use rocket::serde::{Deserialize, Serialize};
    use sea_orm::entity::prelude::*;
    use super::session;
    use utils::ThinWrapperSerde;
    #[serde(crate = "rocket::serde")]
    pub struct UserCreatePatch {
        pub name: String,
        pub nickname: String,
        pub password: String,
        pub email: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for UserCreatePatch {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "name" => _serde::__private::Ok(__Field::__field0),
                                "nickname" => _serde::__private::Ok(__Field::__field1),
                                "password" => _serde::__private::Ok(__Field::__field2),
                                "email" => _serde::__private::Ok(__Field::__field3),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"name" => _serde::__private::Ok(__Field::__field0),
                                b"nickname" => _serde::__private::Ok(__Field::__field1),
                                b"password" => _serde::__private::Ok(__Field::__field2),
                                b"email" => _serde::__private::Ok(__Field::__field3),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<UserCreatePatch>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = UserCreatePatch;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "struct UserCreatePatch")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"struct UserCreatePatch with 4 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                    &"struct UserCreatePatch with 4 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(2usize,
                                                    &"struct UserCreatePatch with 4 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(3usize,
                                                    &"struct UserCreatePatch with 4 elements"));
                                    }
                                };
                            _serde::__private::Ok(UserCreatePatch {
                                    name: __field0,
                                    nickname: __field1,
                                    password: __field2,
                                    email: __field3,
                                })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("name"));
                                            }
                                        __field0 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("nickname"));
                                            }
                                        __field1 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("password"));
                                            }
                                        __field2 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("email"));
                                            }
                                        __field3 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("name") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("nickname") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("password") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field3 =
                                match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("email") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            _serde::__private::Ok(UserCreatePatch {
                                    name: __field0,
                                    nickname: __field1,
                                    password: __field2,
                                    email: __field3,
                                })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["name", "nickname", "password", "email"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                        "UserCreatePatch", FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<UserCreatePatch>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    #[serde(crate = "rocket::serde")]
    pub struct UserEditPatch {
        pub nickname: Option<String>,
        pub password: Option<String>,
        pub email: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for UserEditPatch {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "nickname" => _serde::__private::Ok(__Field::__field0),
                                "password" => _serde::__private::Ok(__Field::__field1),
                                "email" => _serde::__private::Ok(__Field::__field2),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"nickname" => _serde::__private::Ok(__Field::__field0),
                                b"password" => _serde::__private::Ok(__Field::__field1),
                                b"email" => _serde::__private::Ok(__Field::__field2),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<UserEditPatch>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = UserEditPatch;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "struct UserEditPatch")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"struct UserEditPatch with 3 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                    &"struct UserEditPatch with 3 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(2usize,
                                                    &"struct UserEditPatch with 3 elements"));
                                    }
                                };
                            _serde::__private::Ok(UserEditPatch {
                                    nickname: __field0,
                                    password: __field1,
                                    email: __field2,
                                })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::MapAccess<'de> {
                            let mut __field0:
                                    _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field1:
                                    _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field2:
                                    _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("nickname"));
                                            }
                                        __field0 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("password"));
                                            }
                                        __field1 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("email"));
                                            }
                                        __field2 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("nickname") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("password") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("email") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            _serde::__private::Ok(UserEditPatch {
                                    nickname: __field0,
                                    password: __field1,
                                    email: __field2,
                                })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["nickname", "password", "email"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                        "UserEditPatch", FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<UserEditPatch>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    pub type Uid = u32;
    pub enum Privilege { Me, Admin, }
    impl ::core::marker::StructuralPartialEq for Privilege {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Privilege {
        #[inline]
        fn eq(&self, other: &Privilege) -> bool {
            {
                let __self_vi =
                    ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi =
                    ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) { _ => true, }
                    } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Privilege {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Privilege::Me,) => {
                    ::core::fmt::Formatter::write_str(f, "Me")
                }
                (&Privilege::Admin,) => {
                    ::core::fmt::Formatter::write_str(f, "Admin")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Privilege {
        #[inline]
        fn clone(&self) -> Privilege { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Privilege { }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            #[allow(unused_extern_crates, clippy :: useless_attribute)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl _serde::Serialize for Privilege {
                fn serialize<__S>(&self, __serializer: __S)
                    -> _serde::__private::Result<__S::Ok, __S::Error> where
                    __S: _serde::Serializer {
                    match *self {
                        Privilege::Me =>
                            _serde::Serializer::serialize_unit_variant(__serializer,
                                "Privilege", 0u32, "Me"),
                        Privilege::Admin =>
                            _serde::Serializer::serialize_unit_variant(__serializer,
                                "Privilege", 1u32, "Admin"),
                    }
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            #[allow(unused_extern_crates, clippy :: useless_attribute)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Privilege {
                fn deserialize<__D>(__deserializer: __D)
                    -> _serde::__private::Result<Self, __D::Error> where
                    __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "variant identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ =>
                                    _serde::__private::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "Me" => _serde::__private::Ok(__Field::__field0),
                                "Admin" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(__value,
                                            VARIANTS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"Me" => _serde::__private::Ok(__Field::__field0),
                                b"Admin" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(__value,
                                            VARIANTS))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Privilege>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Privilege;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "enum Privilege")
                        }
                        fn visit_enum<__A>(self, __data: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::EnumAccess<'de> {
                            match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(Privilege::Me)
                                }
                                (__Field::__field1, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(Privilege::Admin)
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &["Me", "Admin"];
                    _serde::Deserializer::deserialize_enum(__deserializer,
                        "Privilege", VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Privilege>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    pub struct Privileges(pub Vec<Privilege>);
    impl Privileges {
        fn into_inner(self) -> Self::Inner { self.0 }
    }
    impl std::ops::Deref for Privileges {
        type Target = Vec<Privilege>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl std::ops::DerefMut for Privileges {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    impl serde::Serialize for Privileges {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: serde::Serializer {
            self.0.serialize(serializer)
        }
    }
    impl<'de> serde::Deserialize<'de> for Privileges {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
            D: serde::Deserializer<'de> {
            Ok(Privileges::new(Vec::deserialize(deserializer)?))
        }
    }
    impl ::core::marker::StructuralPartialEq for Privileges {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Privileges {
        #[inline]
        fn eq(&self, other: &Privileges) -> bool {
            match *other {
                Self(ref __self_1_0) =>
                    match *self {
                        Self(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &Privileges) -> bool {
            match *other {
                Self(ref __self_1_0) =>
                    match *self {
                        Self(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                    },
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::TryGetableFromJson for Privileges { }
    #[automatically_derived]
    impl std::convert::From<Privileges> for sea_orm::Value {
        fn from(source: Privileges) -> Self {
            sea_orm::Value::Json(serde_json::to_value(&source).ok().map(|s|
                        std::boxed::Box::new(s)))
        }
    }
    #[automatically_derived]
    impl sea_query::ValueType for Privileges {
        fn try_from(v: sea_orm::Value)
            -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
            match v {
                sea_orm::Value::Json(Some(json)) =>
                    Ok(serde_json::from_value(*json).map_err(|_|
                                    sea_orm::sea_query::ValueTypeErr)?),
                _ => Err(sea_orm::sea_query::ValueTypeErr),
            }
        }
        fn type_name() -> String { "Privileges".to_owned() }
        fn column_type() -> sea_orm::sea_query::ColumnType {
            sea_orm::sea_query::ColumnType::Json
        }
    }
    #[automatically_derived]
    impl sea_orm::sea_query::Nullable for Privileges {
        fn null() -> sea_orm::Value { sea_orm::Value::Json(None) }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Privileges {
        #[inline]
        fn clone(&self) -> Privileges {
            match *self {
                Self(ref __self_0_0) =>
                    Privileges(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Privileges {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Privileges");
                    let _ =
                        ::core::fmt::DebugTuple::field(debug_trait_builder,
                            &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[serde(crate = "rocket::serde")]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key,)]
        pub id: Uid,
        pub name: String,
        pub nickname: String,
        pub password_phc: String,
        pub email: String,
        pub privileges: Privileges,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Model {
        #[inline]
        fn clone(&self) -> Model {
            match *self {
                Self {
                    id: ref __self_0_0,
                    name: ref __self_0_1,
                    nickname: ref __self_0_2,
                    password_phc: ref __self_0_3,
                    email: ref __self_0_4,
                    privileges: ref __self_0_5 } =>
                    Model {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        name: ::core::clone::Clone::clone(&(*__self_0_1)),
                        nickname: ::core::clone::Clone::clone(&(*__self_0_2)),
                        password_phc: ::core::clone::Clone::clone(&(*__self_0_3)),
                        email: ::core::clone::Clone::clone(&(*__self_0_4)),
                        privileges: ::core::clone::Clone::clone(&(*__self_0_5)),
                    },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Model {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    id: ref __self_0_0,
                    name: ref __self_0_1,
                    nickname: ref __self_0_2,
                    password_phc: ref __self_0_3,
                    email: ref __self_0_4,
                    privileges: ref __self_0_5 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Model");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "nickname", &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "password_phc", &&(*__self_0_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "email", &&(*__self_0_4));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "privileges", &&(*__self_0_5));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Model {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Model {
        #[inline]
        fn eq(&self, other: &Model) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    name: ref __self_1_1,
                    nickname: ref __self_1_2,
                    password_phc: ref __self_1_3,
                    email: ref __self_1_4,
                    privileges: ref __self_1_5 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            name: ref __self_0_1,
                            nickname: ref __self_0_2,
                            password_phc: ref __self_0_3,
                            email: ref __self_0_4,
                            privileges: ref __self_0_5 } =>
                            (*__self_0_0) == (*__self_1_0) &&
                                                (*__self_0_1) == (*__self_1_1) &&
                                            (*__self_0_2) == (*__self_1_2) &&
                                        (*__self_0_3) == (*__self_1_3) &&
                                    (*__self_0_4) == (*__self_1_4) &&
                                (*__self_0_5) == (*__self_1_5),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &Model) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    name: ref __self_1_1,
                    nickname: ref __self_1_2,
                    password_phc: ref __self_1_3,
                    email: ref __self_1_4,
                    privileges: ref __self_1_5 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            name: ref __self_0_1,
                            nickname: ref __self_0_2,
                            password_phc: ref __self_0_3,
                            email: ref __self_0_4,
                            privileges: ref __self_0_5 } =>
                            (*__self_0_0) != (*__self_1_0) ||
                                                (*__self_0_1) != (*__self_1_1) ||
                                            (*__self_0_2) != (*__self_1_2) ||
                                        (*__self_0_3) != (*__self_1_3) ||
                                    (*__self_0_4) != (*__self_1_4) ||
                                (*__self_0_5) != (*__self_1_5),
                    },
            }
        }
    }
    pub enum Column { Id, Name, Nickname, PasswordPhc, Email, Privileges, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Column { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Column {
        #[inline]
        fn clone(&self) -> Column { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Column {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Column::Id,) => {
                    ::core::fmt::Formatter::write_str(f, "Id")
                }
                (&Column::Name,) => {
                    ::core::fmt::Formatter::write_str(f, "Name")
                }
                (&Column::Nickname,) => {
                    ::core::fmt::Formatter::write_str(f, "Nickname")
                }
                (&Column::PasswordPhc,) => {
                    ::core::fmt::Formatter::write_str(f, "PasswordPhc")
                }
                (&Column::Email,) => {
                    ::core::fmt::Formatter::write_str(f, "Email")
                }
                (&Column::Privileges,) => {
                    ::core::fmt::Formatter::write_str(f, "Privileges")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct ColumnIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl ColumnIter {
        fn get(&self, idx: usize) -> Option<Column> {
            match idx {
                0usize => ::core::option::Option::Some(Column::Id),
                1usize => ::core::option::Option::Some(Column::Name),
                2usize => ::core::option::Option::Some(Column::Nickname),
                3usize => ::core::option::Option::Some(Column::PasswordPhc),
                4usize => ::core::option::Option::Some(Column::Email),
                5usize => ::core::option::Option::Some(Column::Privileges),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for Column {
        type Iterator = ColumnIter;
        fn iter() -> ColumnIter {
            ColumnIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for ColumnIter {
        type Item = Column;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 6usize {
                        0
                    } else { 6usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 6usize {
                    self.idx = 6usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for ColumnIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for ColumnIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 6usize {
                    self.back_idx = 6usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(6usize - self.back_idx)
               }
        }
    }
    impl Clone for ColumnIter {
        fn clone(&self) -> ColumnIter {
            ColumnIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl Column {
        fn default_as_str(&self) -> &str {
            match self {
                Self::Id => "id",
                Self::Name => "name",
                Self::Nickname => "nickname",
                Self::PasswordPhc => "password_phc",
                Self::Email => "email",
                Self::Privileges => "privileges",
            }
        }
    }
    #[automatically_derived]
    impl std::str::FromStr for Column {
        type Err = sea_orm::ColumnFromStrErr;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            match s {
                "id" | "id" => Ok(Column::Id),
                "name" | "name" => Ok(Column::Name),
                "nickname" | "nickname" => Ok(Column::Nickname),
                "password_phc" | "passwordPhc" => Ok(Column::PasswordPhc),
                "email" | "email" => Ok(Column::Email),
                "privileges" | "privileges" => Ok(Column::Privileges),
                _ =>
                    Err(sea_orm::ColumnFromStrErr({
                                let res =
                                    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["Failed to parse \'",
                                                        "\' as `", "`"],
                                            &match (&s, &"Column") {
                                                    args =>
                                                        [::core::fmt::ArgumentV1::new_display(args.0),
                                                                ::core::fmt::ArgumentV1::new_display(args.1)],
                                                }));
                                res
                            })),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::Iden for Column {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for Column {
        fn as_str(&self) -> &str { self.default_as_str() }
    }
    #[automatically_derived]
    impl sea_orm::prelude::ColumnTrait for Column {
        type EntityName = Entity;
        fn def(&self) -> sea_orm::prelude::ColumnDef {
            match self {
                Self::Id => {
                    std::convert::Into::<sea_orm::ColumnType>::into(<Uid as
                                    sea_orm::sea_query::ValueType>::column_type()).def()
                }
                Self::Name =>
                    sea_orm::prelude::ColumnType::String(None).def(),
                Self::Nickname =>
                    sea_orm::prelude::ColumnType::String(None).def(),
                Self::PasswordPhc =>
                    sea_orm::prelude::ColumnType::String(None).def(),
                Self::Email =>
                    sea_orm::prelude::ColumnType::String(None).def(),
                Self::Privileges => {
                    std::convert::Into::<sea_orm::ColumnType>::into(<Privileges
                                    as sea_orm::sea_query::ValueType>::column_type()).def()
                }
            }
        }
    }
    pub struct Entity;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Entity { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Entity {
        #[inline]
        fn clone(&self) -> Entity { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Entity {
        #[inline]
        fn default() -> Entity { Entity {} }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Entity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self => { ::core::fmt::Formatter::write_str(f, "Entity") }
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::entity::EntityTrait for Entity {
        type Model = Model;
        type Column = Column;
        type PrimaryKey = PrimaryKey;
        type Relation = Relation;
    }
    #[automatically_derived]
    impl sea_orm::Iden for Entity {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for Entity {
        fn as_str(&self) -> &str {
            <Self as sea_orm::EntityName>::table_name(self)
        }
    }
    #[automatically_derived]
    impl sea_orm::prelude::EntityName for Entity {
        fn schema_name(&self) -> Option<&str> { None }
        fn table_name(&self) -> &str { "users" }
    }
    pub enum PrimaryKey { Id, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PrimaryKey { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PrimaryKey {
        #[inline]
        fn clone(&self) -> PrimaryKey { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PrimaryKey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&PrimaryKey::Id,) => {
                    ::core::fmt::Formatter::write_str(f, "Id")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct PrimaryKeyIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl PrimaryKeyIter {
        fn get(&self, idx: usize) -> Option<PrimaryKey> {
            match idx {
                0usize => ::core::option::Option::Some(PrimaryKey::Id),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for PrimaryKey {
        type Iterator = PrimaryKeyIter;
        fn iter() -> PrimaryKeyIter {
            PrimaryKeyIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for PrimaryKeyIter {
        type Item = PrimaryKey;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 1usize {
                        0
                    } else { 1usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 1usize {
                    self.idx = 1usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for PrimaryKeyIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for PrimaryKeyIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 1usize {
                    self.back_idx = 1usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(1usize - self.back_idx)
               }
        }
    }
    impl Clone for PrimaryKeyIter {
        fn clone(&self) -> PrimaryKeyIter {
            PrimaryKeyIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::Iden for PrimaryKey {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for PrimaryKey {
        fn as_str(&self) -> &str { match self { Self::Id => "id", } }
    }
    #[automatically_derived]
    impl sea_orm::PrimaryKeyToColumn for PrimaryKey {
        type Column = Column;
        fn into_column(self) -> Self::Column {
            match self { Self::Id => Self::Column::Id, }
        }
        fn from_column(col: Self::Column) -> Option<Self> {
            match col { Self::Column::Id => Some(Self::Id), _ => None, }
        }
    }
    #[automatically_derived]
    impl PrimaryKeyTrait for PrimaryKey {
        type ValueType = Uid;
        fn auto_increment() -> bool { true }
    }
    #[automatically_derived]
    impl sea_orm::FromQueryResult for Model {
        fn from_query_result(row: &sea_orm::QueryResult, pre: &str)
            -> std::result::Result<Self, sea_orm::DbErr> {
            Ok(Self {
                    id: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Id).into())?,
                    name: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Name).into())?,
                    nickname: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Nickname).into())?,
                    password_phc: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::PasswordPhc).into())?,
                    email: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Email).into())?,
                    privileges: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Privileges).into())?,
                })
        }
    }
    #[automatically_derived]
    impl sea_orm::ModelTrait for Model {
        type Entity = Entity;
        fn get(&self,
            c: <Self::Entity as sea_orm::entity::EntityTrait>::Column)
            -> sea_orm::Value {
            match c {
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id =>
                    self.id.clone().into(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Name
                    => self.name.clone().into(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Nickname =>
                    self.nickname.clone().into(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::PasswordPhc =>
                    self.password_phc.clone().into(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Email
                    => self.email.clone().into(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Privileges =>
                    self.privileges.clone().into(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["field does not exist on Model"],
                            &[])),
            }
        }
        fn set(&mut self,
            c: <Self::Entity as sea_orm::entity::EntityTrait>::Column,
            v: sea_orm::Value) {
            match c {
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id =>
                    self.id = v.unwrap(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Name
                    => self.name = v.unwrap(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Nickname =>
                    self.nickname = v.unwrap(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::PasswordPhc =>
                    self.password_phc = v.unwrap(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Email
                    => self.email = v.unwrap(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Privileges =>
                    self.privileges = v.unwrap(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["field does not exist on Model"],
                            &[])),
            }
        }
    }
    pub struct ActiveModel {
        pub id: sea_orm::ActiveValue<Uid>,
        pub name: sea_orm::ActiveValue<String>,
        pub nickname: sea_orm::ActiveValue<String>,
        pub password_phc: sea_orm::ActiveValue<String>,
        pub email: sea_orm::ActiveValue<String>,
        pub privileges: sea_orm::ActiveValue<Privileges>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ActiveModel {
        #[inline]
        fn clone(&self) -> ActiveModel {
            match *self {
                Self {
                    id: ref __self_0_0,
                    name: ref __self_0_1,
                    nickname: ref __self_0_2,
                    password_phc: ref __self_0_3,
                    email: ref __self_0_4,
                    privileges: ref __self_0_5 } =>
                    ActiveModel {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        name: ::core::clone::Clone::clone(&(*__self_0_1)),
                        nickname: ::core::clone::Clone::clone(&(*__self_0_2)),
                        password_phc: ::core::clone::Clone::clone(&(*__self_0_3)),
                        email: ::core::clone::Clone::clone(&(*__self_0_4)),
                        privileges: ::core::clone::Clone::clone(&(*__self_0_5)),
                    },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ActiveModel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    id: ref __self_0_0,
                    name: ref __self_0_1,
                    nickname: ref __self_0_2,
                    password_phc: ref __self_0_3,
                    email: ref __self_0_4,
                    privileges: ref __self_0_5 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ActiveModel");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "name",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "nickname", &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "password_phc", &&(*__self_0_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "email", &&(*__self_0_4));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "privileges", &&(*__self_0_5));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ActiveModel {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ActiveModel {
        #[inline]
        fn eq(&self, other: &ActiveModel) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    name: ref __self_1_1,
                    nickname: ref __self_1_2,
                    password_phc: ref __self_1_3,
                    email: ref __self_1_4,
                    privileges: ref __self_1_5 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            name: ref __self_0_1,
                            nickname: ref __self_0_2,
                            password_phc: ref __self_0_3,
                            email: ref __self_0_4,
                            privileges: ref __self_0_5 } =>
                            (*__self_0_0) == (*__self_1_0) &&
                                                (*__self_0_1) == (*__self_1_1) &&
                                            (*__self_0_2) == (*__self_1_2) &&
                                        (*__self_0_3) == (*__self_1_3) &&
                                    (*__self_0_4) == (*__self_1_4) &&
                                (*__self_0_5) == (*__self_1_5),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &ActiveModel) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    name: ref __self_1_1,
                    nickname: ref __self_1_2,
                    password_phc: ref __self_1_3,
                    email: ref __self_1_4,
                    privileges: ref __self_1_5 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            name: ref __self_0_1,
                            nickname: ref __self_0_2,
                            password_phc: ref __self_0_3,
                            email: ref __self_0_4,
                            privileges: ref __self_0_5 } =>
                            (*__self_0_0) != (*__self_1_0) ||
                                                (*__self_0_1) != (*__self_1_1) ||
                                            (*__self_0_2) != (*__self_1_2) ||
                                        (*__self_0_3) != (*__self_1_3) ||
                                    (*__self_0_4) != (*__self_1_4) ||
                                (*__self_0_5) != (*__self_1_5),
                    },
            }
        }
    }
    #[automatically_derived]
    impl std::default::Default for ActiveModel {
        fn default() -> Self { <Self as sea_orm::ActiveModelBehavior>::new() }
    }
    #[automatically_derived]
    impl std::convert::From<<Entity as EntityTrait>::Model> for ActiveModel {
        fn from(m: <Entity as EntityTrait>::Model) -> Self {
            Self {
                id: sea_orm::ActiveValue::unchanged(m.id),
                name: sea_orm::ActiveValue::unchanged(m.name),
                nickname: sea_orm::ActiveValue::unchanged(m.nickname),
                password_phc: sea_orm::ActiveValue::unchanged(m.password_phc),
                email: sea_orm::ActiveValue::unchanged(m.email),
                privileges: sea_orm::ActiveValue::unchanged(m.privileges),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::IntoActiveModel<ActiveModel> for
        <Entity as EntityTrait>::Model {
        fn into_active_model(self) -> ActiveModel { self.into() }
    }
    #[automatically_derived]
    impl sea_orm::ActiveModelTrait for ActiveModel {
        type Entity = Entity;
        fn take(&mut self, c: <Self::Entity as EntityTrait>::Column)
            -> sea_orm::ActiveValue<sea_orm::Value> {
            match c {
                <Self::Entity as EntityTrait>::Column::Id => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.id);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Name => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.name);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Nickname => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.nickname);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::PasswordPhc => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.password_phc);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Email => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.email);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Privileges => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.privileges);
                    value.into_wrapped_value()
                }
                _ => sea_orm::ActiveValue::not_set(),
            }
        }
        fn get(&self, c: <Self::Entity as EntityTrait>::Column)
            -> sea_orm::ActiveValue<sea_orm::Value> {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Name =>
                    self.name.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Nickname =>
                    self.nickname.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::PasswordPhc =>
                    self.password_phc.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Email =>
                    self.email.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Privileges =>
                    self.privileges.clone().into_wrapped_value(),
                _ => sea_orm::ActiveValue::not_set(),
            }
        }
        fn set(&mut self, c: <Self::Entity as EntityTrait>::Column,
            v: sea_orm::Value) {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Name =>
                    self.name = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Nickname =>
                    self.nickname = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::PasswordPhc =>
                    self.password_phc = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Email =>
                    self.email = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Privileges =>
                    self.privileges = sea_orm::ActiveValue::set(v.unwrap()),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["This ActiveModel does not have this field"],
                            &[])),
            }
        }
        fn not_set(&mut self, c: <Self::Entity as EntityTrait>::Column) {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Name =>
                    self.name = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Nickname =>
                    self.nickname = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::PasswordPhc =>
                    self.password_phc = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Email =>
                    self.email = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Privileges =>
                    self.privileges = sea_orm::ActiveValue::not_set(),
                _ => {}
            }
        }
        fn is_not_set(&self, c: <Self::Entity as EntityTrait>::Column)
            -> bool {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Name =>
                    self.name.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Nickname =>
                    self.nickname.is_not_set(),
                <Self::Entity as EntityTrait>::Column::PasswordPhc =>
                    self.password_phc.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Email =>
                    self.email.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Privileges =>
                    self.privileges.is_not_set(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["This ActiveModel does not have this field"],
                            &[])),
            }
        }
        fn default() -> Self {
            Self {
                id: sea_orm::ActiveValue::not_set(),
                name: sea_orm::ActiveValue::not_set(),
                nickname: sea_orm::ActiveValue::not_set(),
                password_phc: sea_orm::ActiveValue::not_set(),
                email: sea_orm::ActiveValue::not_set(),
                privileges: sea_orm::ActiveValue::not_set(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for Model {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "nickname" => _serde::__private::Ok(__Field::__field2),
                                "password_phc" => _serde::__private::Ok(__Field::__field3),
                                "email" => _serde::__private::Ok(__Field::__field4),
                                "privileges" => _serde::__private::Ok(__Field::__field5),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"nickname" => _serde::__private::Ok(__Field::__field2),
                                b"password_phc" => _serde::__private::Ok(__Field::__field3),
                                b"email" => _serde::__private::Ok(__Field::__field4),
                                b"privileges" => _serde::__private::Ok(__Field::__field5),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Model>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Model;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "struct Model")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<Uid>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"struct Model with 6 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                    &"struct Model with 6 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(2usize,
                                                    &"struct Model with 6 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(3usize,
                                                    &"struct Model with 6 elements"));
                                    }
                                };
                            let __field4 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(4usize,
                                                    &"struct Model with 6 elements"));
                                    }
                                };
                            let __field5 =
                                match match _serde::de::SeqAccess::next_element::<Privileges>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(5usize,
                                                    &"struct Model with 6 elements"));
                                    }
                                };
                            _serde::__private::Ok(Model {
                                    id: __field0,
                                    name: __field1,
                                    nickname: __field2,
                                    password_phc: __field3,
                                    email: __field4,
                                    privileges: __field5,
                                })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::__private::Option<Uid> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Privileges> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("id"));
                                            }
                                        __field0 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Uid>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("name"));
                                            }
                                        __field1 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("nickname"));
                                            }
                                        __field2 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("password_phc"));
                                            }
                                        __field3 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("email"));
                                            }
                                        __field4 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("privileges"));
                                            }
                                        __field5 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Privileges>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("name") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("nickname") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field3 =
                                match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("password_phc") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field4 =
                                match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("email") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field5 =
                                match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("privileges") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            _serde::__private::Ok(Model {
                                    id: __field0,
                                    name: __field1,
                                    nickname: __field2,
                                    password_phc: __field3,
                                    email: __field4,
                                    privileges: __field5,
                                })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["id", "name", "nickname", "password_phc", "email",
                                    "privileges"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                        "Model", FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Model>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for Model {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                "Model", false as usize + 1 + 1 + 1 + 1 + 1 + 1) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "id", &self.id) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "name", &self.name) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "nickname", &self.nickname) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "password_phc", &self.password_phc) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "email", &self.email) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "privileges", &self.privileges) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    pub enum Relation {

        #[sea_orm(has_many = "session::Entity")]
        Session,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Relation { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Relation {
        #[inline]
        fn clone(&self) -> Relation { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Relation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Relation::Session,) => {
                    ::core::fmt::Formatter::write_str(f, "Session")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct RelationIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl RelationIter {
        fn get(&self, idx: usize) -> Option<Relation> {
            match idx {
                0usize => ::core::option::Option::Some(Relation::Session),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for Relation {
        type Iterator = RelationIter;
        fn iter() -> RelationIter {
            RelationIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for RelationIter {
        type Item = Relation;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 1usize {
                        0
                    } else { 1usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 1usize {
                    self.idx = 1usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for RelationIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for RelationIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 1usize {
                    self.back_idx = 1usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(1usize - self.back_idx)
               }
        }
    }
    impl Clone for RelationIter {
        fn clone(&self) -> RelationIter {
            RelationIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::entity::RelationTrait for Relation {
        fn def(&self) -> sea_orm::entity::RelationDef {
            match self {
                Self::Session => Entity::has_many(session::Entity).into(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["No RelationDef for Relation"],
                            &[])),
            }
        }
    }
    impl Related<session::Entity> for Entity {
        fn to() -> RelationDef { Relation::Session.def() }
    }
    impl ActiveModelBehavior for ActiveModel {}
    #[serde(crate = "rocket::serde")]
    pub struct AccessToken(pub String);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for AccessToken {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    _serde::Serializer::serialize_newtype_struct(__serializer,
                        "AccessToken", &self.0)
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for AccessToken {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<AccessToken>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = AccessToken;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "tuple struct AccessToken")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E)
                            -> _serde::__private::Result<Self::Value, __E::Error> where
                            __E: _serde::Deserializer<'de> {
                            let __field0: String =
                                match <String as _serde::Deserialize>::deserialize(__e) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            _serde::__private::Ok(AccessToken(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"tuple struct AccessToken with 1 element"));
                                    }
                                };
                            _serde::__private::Ok(AccessToken(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                        "AccessToken",
                        __Visitor {
                            marker: _serde::__private::PhantomData::<AccessToken>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    #[serde(crate = "rocket::serde")]
    pub struct RefreshToken(pub String);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for RefreshToken {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    _serde::Serializer::serialize_newtype_struct(__serializer,
                        "RefreshToken", &self.0)
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for RefreshToken {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<RefreshToken>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = RefreshToken;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "tuple struct RefreshToken")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E)
                            -> _serde::__private::Result<Self::Value, __E::Error> where
                            __E: _serde::Deserializer<'de> {
                            let __field0: String =
                                match <String as _serde::Deserialize>::deserialize(__e) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            _serde::__private::Ok(RefreshToken(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"tuple struct RefreshToken with 1 element"));
                                    }
                                };
                            _serde::__private::Ok(RefreshToken(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                        "RefreshToken",
                        __Visitor {
                            marker: _serde::__private::PhantomData::<RefreshToken>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
}
pub mod session {
    use rocket::serde::{Deserialize, Serialize};
    use sea_orm::entity::prelude::*;
    use super::user::{self, Uid};
    use crate::utils::Time;
    pub type SessionId = u32;
    #[serde(crate = "rocket::serde")]
    #[sea_orm(table_name = "sessions")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: SessionId,
        pub uid: Uid,
        pub expires_at: Time,
        pub persist: bool,
        pub counter: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Model {
        #[inline]
        fn clone(&self) -> Model {
            match *self {
                Self {
                    id: ref __self_0_0,
                    uid: ref __self_0_1,
                    expires_at: ref __self_0_2,
                    persist: ref __self_0_3,
                    counter: ref __self_0_4 } =>
                    Model {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        uid: ::core::clone::Clone::clone(&(*__self_0_1)),
                        expires_at: ::core::clone::Clone::clone(&(*__self_0_2)),
                        persist: ::core::clone::Clone::clone(&(*__self_0_3)),
                        counter: ::core::clone::Clone::clone(&(*__self_0_4)),
                    },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Model {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    id: ref __self_0_0,
                    uid: ref __self_0_1,
                    expires_at: ref __self_0_2,
                    persist: ref __self_0_3,
                    counter: ref __self_0_4 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Model");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "uid",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "expires_at", &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "persist", &&(*__self_0_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "counter", &&(*__self_0_4));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Model {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Model {
        #[inline]
        fn eq(&self, other: &Model) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    uid: ref __self_1_1,
                    expires_at: ref __self_1_2,
                    persist: ref __self_1_3,
                    counter: ref __self_1_4 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            uid: ref __self_0_1,
                            expires_at: ref __self_0_2,
                            persist: ref __self_0_3,
                            counter: ref __self_0_4 } =>
                            (*__self_0_0) == (*__self_1_0) &&
                                            (*__self_0_1) == (*__self_1_1) &&
                                        (*__self_0_2) == (*__self_1_2) &&
                                    (*__self_0_3) == (*__self_1_3) &&
                                (*__self_0_4) == (*__self_1_4),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &Model) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    uid: ref __self_1_1,
                    expires_at: ref __self_1_2,
                    persist: ref __self_1_3,
                    counter: ref __self_1_4 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            uid: ref __self_0_1,
                            expires_at: ref __self_0_2,
                            persist: ref __self_0_3,
                            counter: ref __self_0_4 } =>
                            (*__self_0_0) != (*__self_1_0) ||
                                            (*__self_0_1) != (*__self_1_1) ||
                                        (*__self_0_2) != (*__self_1_2) ||
                                    (*__self_0_3) != (*__self_1_3) ||
                                (*__self_0_4) != (*__self_1_4),
                    },
            }
        }
    }
    pub enum Column { Id, Uid, ExpiresAt, Persist, Counter, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Column { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Column {
        #[inline]
        fn clone(&self) -> Column { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Column {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Column::Id,) => {
                    ::core::fmt::Formatter::write_str(f, "Id")
                }
                (&Column::Uid,) => {
                    ::core::fmt::Formatter::write_str(f, "Uid")
                }
                (&Column::ExpiresAt,) => {
                    ::core::fmt::Formatter::write_str(f, "ExpiresAt")
                }
                (&Column::Persist,) => {
                    ::core::fmt::Formatter::write_str(f, "Persist")
                }
                (&Column::Counter,) => {
                    ::core::fmt::Formatter::write_str(f, "Counter")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct ColumnIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl ColumnIter {
        fn get(&self, idx: usize) -> Option<Column> {
            match idx {
                0usize => ::core::option::Option::Some(Column::Id),
                1usize => ::core::option::Option::Some(Column::Uid),
                2usize => ::core::option::Option::Some(Column::ExpiresAt),
                3usize => ::core::option::Option::Some(Column::Persist),
                4usize => ::core::option::Option::Some(Column::Counter),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for Column {
        type Iterator = ColumnIter;
        fn iter() -> ColumnIter {
            ColumnIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for ColumnIter {
        type Item = Column;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 5usize {
                        0
                    } else { 5usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 5usize {
                    self.idx = 5usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for ColumnIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for ColumnIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 5usize {
                    self.back_idx = 5usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(5usize - self.back_idx)
               }
        }
    }
    impl Clone for ColumnIter {
        fn clone(&self) -> ColumnIter {
            ColumnIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl Column {
        fn default_as_str(&self) -> &str {
            match self {
                Self::Id => "id",
                Self::Uid => "uid",
                Self::ExpiresAt => "expires_at",
                Self::Persist => "persist",
                Self::Counter => "counter",
            }
        }
    }
    #[automatically_derived]
    impl std::str::FromStr for Column {
        type Err = sea_orm::ColumnFromStrErr;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            match s {
                "id" | "id" => Ok(Column::Id),
                "uid" | "uid" => Ok(Column::Uid),
                "expires_at" | "expiresAt" => Ok(Column::ExpiresAt),
                "persist" | "persist" => Ok(Column::Persist),
                "counter" | "counter" => Ok(Column::Counter),
                _ =>
                    Err(sea_orm::ColumnFromStrErr({
                                let res =
                                    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["Failed to parse \'",
                                                        "\' as `", "`"],
                                            &match (&s, &"Column") {
                                                    args =>
                                                        [::core::fmt::ArgumentV1::new_display(args.0),
                                                                ::core::fmt::ArgumentV1::new_display(args.1)],
                                                }));
                                res
                            })),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::Iden for Column {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for Column {
        fn as_str(&self) -> &str { self.default_as_str() }
    }
    #[automatically_derived]
    impl sea_orm::prelude::ColumnTrait for Column {
        type EntityName = Entity;
        fn def(&self) -> sea_orm::prelude::ColumnDef {
            match self {
                Self::Id => {
                    std::convert::Into::<sea_orm::ColumnType>::into(<SessionId
                                    as sea_orm::sea_query::ValueType>::column_type()).def()
                }
                Self::Uid => {
                    std::convert::Into::<sea_orm::ColumnType>::into(<Uid as
                                    sea_orm::sea_query::ValueType>::column_type()).def()
                }
                Self::ExpiresAt => sea_orm::prelude::ColumnType::Time.def(),
                Self::Persist => sea_orm::prelude::ColumnType::Boolean.def(),
                Self::Counter => sea_orm::prelude::ColumnType::Unsigned.def(),
            }
        }
    }
    pub struct Entity;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Entity { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Entity {
        #[inline]
        fn clone(&self) -> Entity { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Entity {
        #[inline]
        fn default() -> Entity { Entity {} }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Entity {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self => { ::core::fmt::Formatter::write_str(f, "Entity") }
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::entity::EntityTrait for Entity {
        type Model = Model;
        type Column = Column;
        type PrimaryKey = PrimaryKey;
        type Relation = Relation;
    }
    #[automatically_derived]
    impl sea_orm::Iden for Entity {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for Entity {
        fn as_str(&self) -> &str {
            <Self as sea_orm::EntityName>::table_name(self)
        }
    }
    #[automatically_derived]
    impl sea_orm::prelude::EntityName for Entity {
        fn schema_name(&self) -> Option<&str> { None }
        fn table_name(&self) -> &str { "sessions" }
    }
    pub enum PrimaryKey { Id, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for PrimaryKey { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PrimaryKey {
        #[inline]
        fn clone(&self) -> PrimaryKey { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PrimaryKey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&PrimaryKey::Id,) => {
                    ::core::fmt::Formatter::write_str(f, "Id")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct PrimaryKeyIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl PrimaryKeyIter {
        fn get(&self, idx: usize) -> Option<PrimaryKey> {
            match idx {
                0usize => ::core::option::Option::Some(PrimaryKey::Id),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for PrimaryKey {
        type Iterator = PrimaryKeyIter;
        fn iter() -> PrimaryKeyIter {
            PrimaryKeyIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for PrimaryKeyIter {
        type Item = PrimaryKey;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 1usize {
                        0
                    } else { 1usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 1usize {
                    self.idx = 1usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for PrimaryKeyIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for PrimaryKeyIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 1usize {
                    self.back_idx = 1usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(1usize - self.back_idx)
               }
        }
    }
    impl Clone for PrimaryKeyIter {
        fn clone(&self) -> PrimaryKeyIter {
            PrimaryKeyIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::Iden for PrimaryKey {
        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
            {
                    let result =
                        s.write_fmt(::core::fmt::Arguments::new_v1(&[""],
                                &[::core::fmt::ArgumentV1::new_display(&self.as_str())]));
                    result
                }.unwrap();
        }
    }
    #[automatically_derived]
    impl sea_orm::IdenStatic for PrimaryKey {
        fn as_str(&self) -> &str { match self { Self::Id => "id", } }
    }
    #[automatically_derived]
    impl sea_orm::PrimaryKeyToColumn for PrimaryKey {
        type Column = Column;
        fn into_column(self) -> Self::Column {
            match self { Self::Id => Self::Column::Id, }
        }
        fn from_column(col: Self::Column) -> Option<Self> {
            match col { Self::Column::Id => Some(Self::Id), _ => None, }
        }
    }
    #[automatically_derived]
    impl PrimaryKeyTrait for PrimaryKey {
        type ValueType = SessionId;
        fn auto_increment() -> bool { true }
    }
    #[automatically_derived]
    impl sea_orm::FromQueryResult for Model {
        fn from_query_result(row: &sea_orm::QueryResult, pre: &str)
            -> std::result::Result<Self, sea_orm::DbErr> {
            Ok(Self {
                    id: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Id).into())?,
                    uid: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Uid).into())?,
                    expires_at: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::ExpiresAt).into())?,
                    persist: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Persist).into())?,
                    counter: row.try_get(pre,
                            sea_orm::IdenStatic::as_str(&<<Self as
                                            sea_orm::ModelTrait>::Entity as
                                            sea_orm::entity::EntityTrait>::Column::Counter).into())?,
                })
        }
    }
    #[automatically_derived]
    impl sea_orm::ModelTrait for Model {
        type Entity = Entity;
        fn get(&self,
            c: <Self::Entity as sea_orm::entity::EntityTrait>::Column)
            -> sea_orm::Value {
            match c {
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id =>
                    self.id.clone().into(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Uid =>
                    self.uid.clone().into(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::ExpiresAt =>
                    self.expires_at.clone().into(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Persist =>
                    self.persist.clone().into(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Counter =>
                    self.counter.clone().into(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["field does not exist on Model"],
                            &[])),
            }
        }
        fn set(&mut self,
            c: <Self::Entity as sea_orm::entity::EntityTrait>::Column,
            v: sea_orm::Value) {
            match c {
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Id =>
                    self.id = v.unwrap(),
                <Self::Entity as sea_orm::entity::EntityTrait>::Column::Uid =>
                    self.uid = v.unwrap(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::ExpiresAt =>
                    self.expires_at = v.unwrap(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Persist =>
                    self.persist = v.unwrap(),
                <Self::Entity as
                    sea_orm::entity::EntityTrait>::Column::Counter =>
                    self.counter = v.unwrap(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["field does not exist on Model"],
                            &[])),
            }
        }
    }
    pub struct ActiveModel {
        pub id: sea_orm::ActiveValue<SessionId>,
        pub uid: sea_orm::ActiveValue<Uid>,
        pub expires_at: sea_orm::ActiveValue<Time>,
        pub persist: sea_orm::ActiveValue<bool>,
        pub counter: sea_orm::ActiveValue<u32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ActiveModel {
        #[inline]
        fn clone(&self) -> ActiveModel {
            match *self {
                Self {
                    id: ref __self_0_0,
                    uid: ref __self_0_1,
                    expires_at: ref __self_0_2,
                    persist: ref __self_0_3,
                    counter: ref __self_0_4 } =>
                    ActiveModel {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        uid: ::core::clone::Clone::clone(&(*__self_0_1)),
                        expires_at: ::core::clone::Clone::clone(&(*__self_0_2)),
                        persist: ::core::clone::Clone::clone(&(*__self_0_3)),
                        counter: ::core::clone::Clone::clone(&(*__self_0_4)),
                    },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ActiveModel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    id: ref __self_0_0,
                    uid: ref __self_0_1,
                    expires_at: ref __self_0_2,
                    persist: ref __self_0_3,
                    counter: ref __self_0_4 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ActiveModel");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "uid",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "expires_at", &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "persist", &&(*__self_0_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "counter", &&(*__self_0_4));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ActiveModel {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ActiveModel {
        #[inline]
        fn eq(&self, other: &ActiveModel) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    uid: ref __self_1_1,
                    expires_at: ref __self_1_2,
                    persist: ref __self_1_3,
                    counter: ref __self_1_4 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            uid: ref __self_0_1,
                            expires_at: ref __self_0_2,
                            persist: ref __self_0_3,
                            counter: ref __self_0_4 } =>
                            (*__self_0_0) == (*__self_1_0) &&
                                            (*__self_0_1) == (*__self_1_1) &&
                                        (*__self_0_2) == (*__self_1_2) &&
                                    (*__self_0_3) == (*__self_1_3) &&
                                (*__self_0_4) == (*__self_1_4),
                    },
            }
        }
        #[inline]
        fn ne(&self, other: &ActiveModel) -> bool {
            match *other {
                Self {
                    id: ref __self_1_0,
                    uid: ref __self_1_1,
                    expires_at: ref __self_1_2,
                    persist: ref __self_1_3,
                    counter: ref __self_1_4 } =>
                    match *self {
                        Self {
                            id: ref __self_0_0,
                            uid: ref __self_0_1,
                            expires_at: ref __self_0_2,
                            persist: ref __self_0_3,
                            counter: ref __self_0_4 } =>
                            (*__self_0_0) != (*__self_1_0) ||
                                            (*__self_0_1) != (*__self_1_1) ||
                                        (*__self_0_2) != (*__self_1_2) ||
                                    (*__self_0_3) != (*__self_1_3) ||
                                (*__self_0_4) != (*__self_1_4),
                    },
            }
        }
    }
    #[automatically_derived]
    impl std::default::Default for ActiveModel {
        fn default() -> Self { <Self as sea_orm::ActiveModelBehavior>::new() }
    }
    #[automatically_derived]
    impl std::convert::From<<Entity as EntityTrait>::Model> for ActiveModel {
        fn from(m: <Entity as EntityTrait>::Model) -> Self {
            Self {
                id: sea_orm::ActiveValue::unchanged(m.id),
                uid: sea_orm::ActiveValue::unchanged(m.uid),
                expires_at: sea_orm::ActiveValue::unchanged(m.expires_at),
                persist: sea_orm::ActiveValue::unchanged(m.persist),
                counter: sea_orm::ActiveValue::unchanged(m.counter),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::IntoActiveModel<ActiveModel> for
        <Entity as EntityTrait>::Model {
        fn into_active_model(self) -> ActiveModel { self.into() }
    }
    #[automatically_derived]
    impl sea_orm::ActiveModelTrait for ActiveModel {
        type Entity = Entity;
        fn take(&mut self, c: <Self::Entity as EntityTrait>::Column)
            -> sea_orm::ActiveValue<sea_orm::Value> {
            match c {
                <Self::Entity as EntityTrait>::Column::Id => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.id);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Uid => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.uid);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::ExpiresAt => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.expires_at);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Persist => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.persist);
                    value.into_wrapped_value()
                }
                <Self::Entity as EntityTrait>::Column::Counter => {
                    let mut value = sea_orm::ActiveValue::not_set();
                    std::mem::swap(&mut value, &mut self.counter);
                    value.into_wrapped_value()
                }
                _ => sea_orm::ActiveValue::not_set(),
            }
        }
        fn get(&self, c: <Self::Entity as EntityTrait>::Column)
            -> sea_orm::ActiveValue<sea_orm::Value> {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Uid =>
                    self.uid.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::ExpiresAt =>
                    self.expires_at.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Persist =>
                    self.persist.clone().into_wrapped_value(),
                <Self::Entity as EntityTrait>::Column::Counter =>
                    self.counter.clone().into_wrapped_value(),
                _ => sea_orm::ActiveValue::not_set(),
            }
        }
        fn set(&mut self, c: <Self::Entity as EntityTrait>::Column,
            v: sea_orm::Value) {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Uid =>
                    self.uid = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::ExpiresAt =>
                    self.expires_at = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Persist =>
                    self.persist = sea_orm::ActiveValue::set(v.unwrap()),
                <Self::Entity as EntityTrait>::Column::Counter =>
                    self.counter = sea_orm::ActiveValue::set(v.unwrap()),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["This ActiveModel does not have this field"],
                            &[])),
            }
        }
        fn not_set(&mut self, c: <Self::Entity as EntityTrait>::Column) {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Uid =>
                    self.uid = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::ExpiresAt =>
                    self.expires_at = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Persist =>
                    self.persist = sea_orm::ActiveValue::not_set(),
                <Self::Entity as EntityTrait>::Column::Counter =>
                    self.counter = sea_orm::ActiveValue::not_set(),
                _ => {}
            }
        }
        fn is_not_set(&self, c: <Self::Entity as EntityTrait>::Column)
            -> bool {
            match c {
                <Self::Entity as EntityTrait>::Column::Id =>
                    self.id.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Uid =>
                    self.uid.is_not_set(),
                <Self::Entity as EntityTrait>::Column::ExpiresAt =>
                    self.expires_at.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Persist =>
                    self.persist.is_not_set(),
                <Self::Entity as EntityTrait>::Column::Counter =>
                    self.counter.is_not_set(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["This ActiveModel does not have this field"],
                            &[])),
            }
        }
        fn default() -> Self {
            Self {
                id: sea_orm::ActiveValue::not_set(),
                uid: sea_orm::ActiveValue::not_set(),
                expires_at: sea_orm::ActiveValue::not_set(),
                persist: sea_orm::ActiveValue::not_set(),
                counter: sea_orm::ActiveValue::not_set(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for Model {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "uid" => _serde::__private::Ok(__Field::__field1),
                                "expires_at" => _serde::__private::Ok(__Field::__field2),
                                "persist" => _serde::__private::Ok(__Field::__field3),
                                "counter" => _serde::__private::Ok(__Field::__field4),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                            -> _serde::__private::Result<Self::Value, __E> where
                            __E: _serde::de::Error {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"uid" => _serde::__private::Ok(__Field::__field1),
                                b"expires_at" => _serde::__private::Ok(__Field::__field2),
                                b"persist" => _serde::__private::Ok(__Field::__field3),
                                b"counter" => _serde::__private::Ok(__Field::__field4),
                                _ => { _serde::__private::Ok(__Field::__ignore) }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                            -> _serde::__private::Result<Self, __D::Error> where
                            __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Model>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Model;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "struct Model")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<SessionId>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"struct Model with 5 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<Uid>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                    &"struct Model with 5 elements"));
                                    }
                                };
                            let __field2 =
                                match match _serde::de::SeqAccess::next_element::<Time>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(2usize,
                                                    &"struct Model with 5 elements"));
                                    }
                                };
                            let __field3 =
                                match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(3usize,
                                                    &"struct Model with 5 elements"));
                                    }
                                };
                            let __field4 =
                                match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(4usize,
                                                    &"struct Model with 5 elements"));
                                    }
                                };
                            _serde::__private::Ok(Model {
                                    id: __field0,
                                    uid: __field1,
                                    expires_at: __field2,
                                    persist: __field3,
                                    counter: __field4,
                                })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::__private::Option<SessionId> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Uid> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Time> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("id"));
                                            }
                                        __field0 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<SessionId>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("uid"));
                                            }
                                        __field1 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Uid>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("expires_at"));
                                            }
                                        __field2 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<Time>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("persist"));
                                            }
                                        __field3 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                                return _serde::__private::Err(<__A::Error as
                                                                _serde::de::Error>::duplicate_field("counter"));
                                            }
                                        __field4 =
                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
                                                    {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                });
                                    }
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("uid") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field2 =
                                match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("expires_at") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field3 =
                                match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("persist") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            let __field4 =
                                match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None =>
                                        match _serde::__private::de::missing_field("counter") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                };
                            _serde::__private::Ok(Model {
                                    id: __field0,
                                    uid: __field1,
                                    expires_at: __field2,
                                    persist: __field3,
                                    counter: __field4,
                                })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["id", "uid", "expires_at", "persist", "counter"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                        "Model", FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Model>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for Model {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                "Model", false as usize + 1 + 1 + 1 + 1 + 1) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "id", &self.id) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "uid", &self.uid) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "expires_at", &self.expires_at) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "persist", &self.persist) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                            "counter", &self.counter) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    pub enum Relation {

        #[sea_orm(belongs_to = "user::Entity", from = "Column::Uid", to =
        "user::Column::Id")]
        User,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Relation { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Relation {
        #[inline]
        fn clone(&self) -> Relation { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Relation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Relation::User,) => {
                    ::core::fmt::Formatter::write_str(f, "User")
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub struct RelationIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl RelationIter {
        fn get(&self, idx: usize) -> Option<Relation> {
            match idx {
                0usize => ::core::option::Option::Some(Relation::User),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for Relation {
        type Iterator = RelationIter;
        fn iter() -> RelationIter {
            RelationIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for RelationIter {
        type Item = Relation;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> { self.nth(0) }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t =
                if self.idx + self.back_idx >= 1usize {
                        0
                    } else { 1usize - self.idx - self.back_idx };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 1usize {
                    self.idx = 1usize;
                    None
                } else { self.idx = idx; self.get(idx - 1) }
        }
    }
    impl ExactSizeIterator for RelationIter {
        fn len(&self) -> usize { self.size_hint().0 }
    }
    impl DoubleEndedIterator for RelationIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 1usize {
                    self.back_idx = 1usize;
                    None
                } else {
                   self.back_idx = back_idx;
                   self.get(1usize - self.back_idx)
               }
        }
    }
    impl Clone for RelationIter {
        fn clone(&self) -> RelationIter {
            RelationIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    #[automatically_derived]
    impl sea_orm::entity::RelationTrait for Relation {
        fn def(&self) -> sea_orm::entity::RelationDef {
            match self {
                Self::User =>
                    Entity::belongs_to(user::Entity).from(Column::Uid).to(user::Column::Id).into(),
                _ =>
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["No RelationDef for Relation"],
                            &[])),
            }
        }
    }
    impl Related<user::Entity> for Entity {
        fn to() -> RelationDef { Relation::User.def() }
    }
    impl ActiveModelBehavior for ActiveModel {}
}
pub mod utils {
    use chrono::TimeZone;
    use rocket::serde::{Serialize, Deserialize};
    pub type Time = chrono::DateTime<chrono::Utc>;
    #[serde(crate = "rocket::serde")]
    pub struct RawTime(i64);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl rocket::serde::Serialize for RawTime {
                fn serialize<__S>(&self, __serializer: __S)
                    -> rocket::serde::__private::Result<__S::Ok, __S::Error>
                    where __S: rocket::serde::Serializer {
                    _serde::Serializer::serialize_newtype_struct(__serializer,
                        "RawTime", &self.0)
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            use rocket::serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try {
                ($__expr : expr) =>
                {
                    match $__expr
                    {
                        _serde :: __private :: Ok(__val) => __val, _serde ::
                        __private :: Err(__err) =>
                        { return _serde :: __private :: Err(__err) ; }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> rocket::serde::Deserialize<'de> for RawTime {
                fn deserialize<__D>(__deserializer: __D)
                    -> rocket::serde::__private::Result<Self, __D::Error> where
                    __D: rocket::serde::Deserializer<'de> {
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<RawTime>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = RawTime;
                        fn expecting(&self,
                            __formatter: &mut _serde::__private::Formatter)
                            -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                "tuple struct RawTime")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E)
                            -> _serde::__private::Result<Self::Value, __E::Error> where
                            __E: _serde::Deserializer<'de> {
                            let __field0: i64 =
                                match <i64 as _serde::Deserialize>::deserialize(__e) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            _serde::__private::Ok(RawTime(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                            -> _serde::__private::Result<Self::Value, __A::Error> where
                            __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                    &"tuple struct RawTime with 1 element"));
                                    }
                                };
                            _serde::__private::Ok(RawTime(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                        "RawTime",
                        __Visitor {
                            marker: _serde::__private::PhantomData::<RawTime>,
                            lifetime: _serde::__private::PhantomData,
                        })
                }
            }
        };
    impl Into<RawTime> for Time {
        fn into(self) -> RawTime { RawTime(self.timestamp_millis()) }
    }
    impl Into<Time> for RawTime {
        fn into(self) -> Time {
            chrono::Utc.timestamp_opt(self.0 / 1000,
                    ((self.0 % 1000) * 1_000_000) as u32).unwrap()
        }
    }
}
