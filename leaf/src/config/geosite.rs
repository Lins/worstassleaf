// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/config/geosite.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Domain {
    // message fields
    pub field_type: Domain_Type,
    pub value: ::std::string::String,
    pub attribute: ::protobuf::RepeatedField<Domain_Attribute>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Domain {
    fn default() -> &'a Domain {
        <Domain as ::protobuf::Message>::default_instance()
    }
}

impl Domain {
    pub fn new() -> Domain {
        ::std::default::Default::default()
    }

    // .Domain.Type type = 1;


    pub fn get_field_type(&self) -> Domain_Type {
        self.field_type
    }

    // string value = 2;


    pub fn get_value(&self) -> &str {
        &self.value
    }

    // repeated .Domain.Attribute attribute = 3;


    pub fn get_attribute(&self) -> &[Domain_Attribute] {
        &self.attribute
    }
}

impl ::protobuf::Message for Domain {
    fn is_initialized(&self) -> bool {
        for v in &self.attribute {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attribute)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != Domain_Type::Plain {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        for value in &self.attribute {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Domain_Type::Plain {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.field_type))?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
        }
        for v in &self.attribute {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Domain {
        Domain::new()
    }

    fn default_instance() -> &'static Domain {
        static instance: ::protobuf::rt::LazyV2<Domain> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Domain::new)
    }
}

impl ::protobuf::Clear for Domain {
    fn clear(&mut self) {
        self.field_type = Domain_Type::Plain;
        self.value.clear();
        self.attribute.clear();
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for Domain {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Domain_Attribute {
    // message fields
    pub key: ::std::string::String,
    // message oneof groups
    pub typed_value: ::std::option::Option<Domain_Attribute_oneof_typed_value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Domain_Attribute {
    fn default() -> &'a Domain_Attribute {
        <Domain_Attribute as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Domain_Attribute_oneof_typed_value {
    bool_value(bool),
    int_value(i64),
}

impl Domain_Attribute {
    pub fn new() -> Domain_Attribute {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }

    // bool bool_value = 2;


    pub fn get_bool_value(&self) -> bool {
        match self.typed_value {
            ::std::option::Option::Some(Domain_Attribute_oneof_typed_value::bool_value(v)) => v,
            _ => false,
        }
    }

    // int64 int_value = 3;


    pub fn get_int_value(&self) -> i64 {
        match self.typed_value {
            ::std::option::Option::Some(Domain_Attribute_oneof_typed_value::int_value(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for Domain_Attribute {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.typed_value = ::std::option::Option::Some(Domain_Attribute_oneof_typed_value::bool_value(is.read_bool()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.typed_value = ::std::option::Option::Some(Domain_Attribute_oneof_typed_value::int_value(is.read_int64()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if let ::std::option::Option::Some(ref v) = self.typed_value {
            match v {
                &Domain_Attribute_oneof_typed_value::bool_value(v) => {
                    my_size += 2;
                },
                &Domain_Attribute_oneof_typed_value::int_value(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if let ::std::option::Option::Some(ref v) = self.typed_value {
            match v {
                &Domain_Attribute_oneof_typed_value::bool_value(v) => {
                    os.write_bool(2, v)?;
                },
                &Domain_Attribute_oneof_typed_value::int_value(v) => {
                    os.write_int64(3, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Domain_Attribute {
        Domain_Attribute::new()
    }

    fn default_instance() -> &'static Domain_Attribute {
        static instance: ::protobuf::rt::LazyV2<Domain_Attribute> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Domain_Attribute::new)
    }
}

impl ::protobuf::Clear for Domain_Attribute {
    fn clear(&mut self) {
        self.key.clear();
        self.typed_value = ::std::option::Option::None;
        self.typed_value = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for Domain_Attribute {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Domain_Type {
    Plain = 0,
    Regex = 1,
    Domain = 2,
    Full = 3,
}

impl ::protobuf::ProtobufEnum for Domain_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Domain_Type> {
        match value {
            0 => ::std::option::Option::Some(Domain_Type::Plain),
            1 => ::std::option::Option::Some(Domain_Type::Regex),
            2 => ::std::option::Option::Some(Domain_Type::Domain),
            3 => ::std::option::Option::Some(Domain_Type::Full),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Domain_Type] = &[
            Domain_Type::Plain,
            Domain_Type::Regex,
            Domain_Type::Domain,
            Domain_Type::Full,
        ];
        values
    }
}

impl ::std::marker::Copy for Domain_Type {
}

impl ::std::default::Default for Domain_Type {
    fn default() -> Self {
        Domain_Type::Plain
    }
}

impl ::protobuf::reflect::ProtobufValue for Domain_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct SiteGroup {
    // message fields
    pub tag: ::std::string::String,
    pub domain: ::protobuf::RepeatedField<Domain>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SiteGroup {
    fn default() -> &'a SiteGroup {
        <SiteGroup as ::protobuf::Message>::default_instance()
    }
}

impl SiteGroup {
    pub fn new() -> SiteGroup {
        ::std::default::Default::default()
    }

    // string tag = 1;


    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    // repeated .Domain domain = 2;


    pub fn get_domain(&self) -> &[Domain] {
        &self.domain
    }
}

impl ::protobuf::Message for SiteGroup {
    fn is_initialized(&self) -> bool {
        for v in &self.domain {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tag)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.domain)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.tag.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tag);
        }
        for value in &self.domain {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.tag.is_empty() {
            os.write_string(1, &self.tag)?;
        }
        for v in &self.domain {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SiteGroup {
        SiteGroup::new()
    }

    fn default_instance() -> &'static SiteGroup {
        static instance: ::protobuf::rt::LazyV2<SiteGroup> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SiteGroup::new)
    }
}

impl ::protobuf::Clear for SiteGroup {
    fn clear(&mut self) {
        self.tag.clear();
        self.domain.clear();
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for SiteGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct SiteGroupList {
    // message fields
    pub site_group: ::protobuf::RepeatedField<SiteGroup>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SiteGroupList {
    fn default() -> &'a SiteGroupList {
        <SiteGroupList as ::protobuf::Message>::default_instance()
    }
}

impl SiteGroupList {
    pub fn new() -> SiteGroupList {
        ::std::default::Default::default()
    }

    // repeated .SiteGroup site_group = 1;


    pub fn get_site_group(&self) -> &[SiteGroup] {
        &self.site_group
    }
}

impl ::protobuf::Message for SiteGroupList {
    fn is_initialized(&self) -> bool {
        for v in &self.site_group {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.site_group)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.site_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.site_group {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SiteGroupList {
        SiteGroupList::new()
    }

    fn default_instance() -> &'static SiteGroupList {
        static instance: ::protobuf::rt::LazyV2<SiteGroupList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SiteGroupList::new)
    }
}

impl ::protobuf::Clear for SiteGroupList {
    fn clear(&mut self) {
        self.site_group.clear();
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for SiteGroupList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}
