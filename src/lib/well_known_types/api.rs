// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Api {
    // message fields
    pub name: ::std::string::String,
    methods: ::protobuf::RepeatedField<Method>,
    options: ::protobuf::RepeatedField<::protobuf::well_known_types::Option>,
    pub version: ::std::string::String,
    source_context: ::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext>,
    mixins: ::protobuf::RepeatedField<Mixin>,
    pub syntax: ::protobuf::well_known_types::Syntax,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Api {}

impl Api {
    pub fn new() -> Api {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Api {
        static mut instance: ::protobuf::lazy::Lazy<Api> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Api,
        };
        unsafe {
            instance.get(Api::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .google.protobuf.Method methods = 2;

    pub fn clear_methods(&mut self) {
        self.methods.clear();
    }

    // Param is passed by value, moved
    pub fn set_methods(&mut self, v: ::protobuf::RepeatedField<Method>) {
        self.methods = v;
    }

    // Mutable pointer to the field.
    pub fn mut_methods(&mut self) -> &mut ::protobuf::RepeatedField<Method> {
        &mut self.methods
    }

    // Take field
    pub fn take_methods(&mut self) -> ::protobuf::RepeatedField<Method> {
        ::std::mem::replace(&mut self.methods, ::protobuf::RepeatedField::new())
    }

    pub fn get_methods(&self) -> &[Method] {
        &self.methods
    }

    fn get_methods_for_reflect(&self) -> &::protobuf::RepeatedField<Method> {
        &self.methods
    }

    fn mut_methods_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Method> {
        &mut self.methods
    }

    // repeated .google.protobuf.Option options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[::protobuf::well_known_types::Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        &mut self.options
    }

    // string version = 4;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // .google.protobuf.SourceContext source_context = 5;

    pub fn clear_source_context(&mut self) {
        self.source_context.clear();
    }

    pub fn has_source_context(&self) -> bool {
        self.source_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_context(&mut self, v: ::protobuf::well_known_types::SourceContext) {
        self.source_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_context(&mut self) -> &mut ::protobuf::well_known_types::SourceContext {
        if self.source_context.is_none() {
            self.source_context.set_default();
        }
        self.source_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_context(&mut self) -> ::protobuf::well_known_types::SourceContext {
        self.source_context.take().unwrap_or_else(|| ::protobuf::well_known_types::SourceContext::new())
    }

    pub fn get_source_context(&self) -> &::protobuf::well_known_types::SourceContext {
        self.source_context.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::SourceContext::default_instance())
    }

    fn get_source_context_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext> {
        &self.source_context
    }

    fn mut_source_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext> {
        &mut self.source_context
    }

    // repeated .google.protobuf.Mixin mixins = 6;

    pub fn clear_mixins(&mut self) {
        self.mixins.clear();
    }

    // Param is passed by value, moved
    pub fn set_mixins(&mut self, v: ::protobuf::RepeatedField<Mixin>) {
        self.mixins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mixins(&mut self) -> &mut ::protobuf::RepeatedField<Mixin> {
        &mut self.mixins
    }

    // Take field
    pub fn take_mixins(&mut self) -> ::protobuf::RepeatedField<Mixin> {
        ::std::mem::replace(&mut self.mixins, ::protobuf::RepeatedField::new())
    }

    pub fn get_mixins(&self) -> &[Mixin] {
        &self.mixins
    }

    fn get_mixins_for_reflect(&self) -> &::protobuf::RepeatedField<Mixin> {
        &self.mixins
    }

    fn mut_mixins_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mixin> {
        &mut self.mixins
    }

    // .google.protobuf.Syntax syntax = 7;

    pub fn clear_syntax(&mut self) {
        self.syntax = ::protobuf::well_known_types::Syntax::SYNTAX_PROTO2;
    }

    // Param is passed by value, moved
    pub fn set_syntax(&mut self, v: ::protobuf::well_known_types::Syntax) {
        self.syntax = v;
    }

    pub fn get_syntax(&self) -> ::protobuf::well_known_types::Syntax {
        self.syntax
    }

    fn get_syntax_for_reflect(&self) -> &::protobuf::well_known_types::Syntax {
        &self.syntax
    }

    fn mut_syntax_for_reflect(&mut self) -> &mut ::protobuf::well_known_types::Syntax {
        &mut self.syntax
    }
}

impl ::protobuf::Message for Api {
    fn is_initialized(&self) -> bool {
        for v in &self.methods {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source_context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.mixins {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.methods)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source_context)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mixins)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.syntax = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.methods {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.version);
        }
        if let Some(v) = self.source_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.mixins {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.syntax != ::protobuf::well_known_types::Syntax::SYNTAX_PROTO2 {
            my_size += ::protobuf::rt::enum_size(7, self.syntax);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.methods {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.options {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.version.is_empty() {
            os.write_string(4, &self.version)?;
        }
        if let Some(v) = self.source_context.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.mixins {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.syntax != ::protobuf::well_known_types::Syntax::SYNTAX_PROTO2 {
            os.write_enum(7, self.syntax.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Api {
    fn new() -> Api {
        Api::new()
    }

    fn descriptor_static(_: ::std::option::Option<Api>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Api::get_name_for_reflect,
                    Api::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Method>>(
                    "methods",
                    Api::get_methods_for_reflect,
                    Api::mut_methods_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Option>>(
                    "options",
                    Api::get_options_for_reflect,
                    Api::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    Api::get_version_for_reflect,
                    Api::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::SourceContext>>(
                    "source_context",
                    Api::get_source_context_for_reflect,
                    Api::mut_source_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mixin>>(
                    "mixins",
                    Api::get_mixins_for_reflect,
                    Api::mut_mixins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<::protobuf::well_known_types::Syntax>>(
                    "syntax",
                    Api::get_syntax_for_reflect,
                    Api::mut_syntax_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Api>(
                    "Api",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Api {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_methods();
        self.clear_options();
        self.clear_version();
        self.clear_source_context();
        self.clear_mixins();
        self.clear_syntax();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Api {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Api {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Method {
    // message fields
    pub name: ::std::string::String,
    pub request_type_url: ::std::string::String,
    pub request_streaming: bool,
    pub response_type_url: ::std::string::String,
    pub response_streaming: bool,
    options: ::protobuf::RepeatedField<::protobuf::well_known_types::Option>,
    pub syntax: ::protobuf::well_known_types::Syntax,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Method {}

impl Method {
    pub fn new() -> Method {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Method {
        static mut instance: ::protobuf::lazy::Lazy<Method> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Method,
        };
        unsafe {
            instance.get(Method::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string request_type_url = 2;

    pub fn clear_request_type_url(&mut self) {
        self.request_type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_type_url(&mut self, v: ::std::string::String) {
        self.request_type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.request_type_url
    }

    // Take field
    pub fn take_request_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.request_type_url, ::std::string::String::new())
    }

    pub fn get_request_type_url(&self) -> &str {
        &self.request_type_url
    }

    fn get_request_type_url_for_reflect(&self) -> &::std::string::String {
        &self.request_type_url
    }

    fn mut_request_type_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.request_type_url
    }

    // bool request_streaming = 3;

    pub fn clear_request_streaming(&mut self) {
        self.request_streaming = false;
    }

    // Param is passed by value, moved
    pub fn set_request_streaming(&mut self, v: bool) {
        self.request_streaming = v;
    }

    pub fn get_request_streaming(&self) -> bool {
        self.request_streaming
    }

    fn get_request_streaming_for_reflect(&self) -> &bool {
        &self.request_streaming
    }

    fn mut_request_streaming_for_reflect(&mut self) -> &mut bool {
        &mut self.request_streaming
    }

    // string response_type_url = 4;

    pub fn clear_response_type_url(&mut self) {
        self.response_type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_type_url(&mut self, v: ::std::string::String) {
        self.response_type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.response_type_url
    }

    // Take field
    pub fn take_response_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.response_type_url, ::std::string::String::new())
    }

    pub fn get_response_type_url(&self) -> &str {
        &self.response_type_url
    }

    fn get_response_type_url_for_reflect(&self) -> &::std::string::String {
        &self.response_type_url
    }

    fn mut_response_type_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.response_type_url
    }

    // bool response_streaming = 5;

    pub fn clear_response_streaming(&mut self) {
        self.response_streaming = false;
    }

    // Param is passed by value, moved
    pub fn set_response_streaming(&mut self, v: bool) {
        self.response_streaming = v;
    }

    pub fn get_response_streaming(&self) -> bool {
        self.response_streaming
    }

    fn get_response_streaming_for_reflect(&self) -> &bool {
        &self.response_streaming
    }

    fn mut_response_streaming_for_reflect(&mut self) -> &mut bool {
        &mut self.response_streaming
    }

    // repeated .google.protobuf.Option options = 6;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[::protobuf::well_known_types::Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Option> {
        &mut self.options
    }

    // .google.protobuf.Syntax syntax = 7;

    pub fn clear_syntax(&mut self) {
        self.syntax = ::protobuf::well_known_types::Syntax::SYNTAX_PROTO2;
    }

    // Param is passed by value, moved
    pub fn set_syntax(&mut self, v: ::protobuf::well_known_types::Syntax) {
        self.syntax = v;
    }

    pub fn get_syntax(&self) -> ::protobuf::well_known_types::Syntax {
        self.syntax
    }

    fn get_syntax_for_reflect(&self) -> &::protobuf::well_known_types::Syntax {
        &self.syntax
    }

    fn mut_syntax_for_reflect(&mut self) -> &mut ::protobuf::well_known_types::Syntax {
        &mut self.syntax
    }
}

impl ::protobuf::Message for Method {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.request_type_url)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.request_streaming = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.response_type_url)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.response_streaming = tmp;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.syntax = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.request_type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.request_type_url);
        }
        if self.request_streaming != false {
            my_size += 2;
        }
        if !self.response_type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.response_type_url);
        }
        if self.response_streaming != false {
            my_size += 2;
        }
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.syntax != ::protobuf::well_known_types::Syntax::SYNTAX_PROTO2 {
            my_size += ::protobuf::rt::enum_size(7, self.syntax);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.request_type_url.is_empty() {
            os.write_string(2, &self.request_type_url)?;
        }
        if self.request_streaming != false {
            os.write_bool(3, self.request_streaming)?;
        }
        if !self.response_type_url.is_empty() {
            os.write_string(4, &self.response_type_url)?;
        }
        if self.response_streaming != false {
            os.write_bool(5, self.response_streaming)?;
        }
        for v in &self.options {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.syntax != ::protobuf::well_known_types::Syntax::SYNTAX_PROTO2 {
            os.write_enum(7, self.syntax.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Method {
    fn new() -> Method {
        Method::new()
    }

    fn descriptor_static(_: ::std::option::Option<Method>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Method::get_name_for_reflect,
                    Method::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "request_type_url",
                    Method::get_request_type_url_for_reflect,
                    Method::mut_request_type_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "request_streaming",
                    Method::get_request_streaming_for_reflect,
                    Method::mut_request_streaming_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "response_type_url",
                    Method::get_response_type_url_for_reflect,
                    Method::mut_response_type_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "response_streaming",
                    Method::get_response_streaming_for_reflect,
                    Method::mut_response_streaming_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Option>>(
                    "options",
                    Method::get_options_for_reflect,
                    Method::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<::protobuf::well_known_types::Syntax>>(
                    "syntax",
                    Method::get_syntax_for_reflect,
                    Method::mut_syntax_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Method>(
                    "Method",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Method {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_request_type_url();
        self.clear_request_streaming();
        self.clear_response_type_url();
        self.clear_response_streaming();
        self.clear_options();
        self.clear_syntax();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Method {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Method {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mixin {
    // message fields
    pub name: ::std::string::String,
    pub root: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mixin {}

impl Mixin {
    pub fn new() -> Mixin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mixin {
        static mut instance: ::protobuf::lazy::Lazy<Mixin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mixin,
        };
        unsafe {
            instance.get(Mixin::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string root = 2;

    pub fn clear_root(&mut self) {
        self.root.clear();
    }

    // Param is passed by value, moved
    pub fn set_root(&mut self, v: ::std::string::String) {
        self.root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root(&mut self) -> &mut ::std::string::String {
        &mut self.root
    }

    // Take field
    pub fn take_root(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.root, ::std::string::String::new())
    }

    pub fn get_root(&self) -> &str {
        &self.root
    }

    fn get_root_for_reflect(&self) -> &::std::string::String {
        &self.root
    }

    fn mut_root_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.root
    }
}

impl ::protobuf::Message for Mixin {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.root)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.root.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.root);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.root.is_empty() {
            os.write_string(2, &self.root)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mixin {
    fn new() -> Mixin {
        Mixin::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mixin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Mixin::get_name_for_reflect,
                    Mixin::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "root",
                    Mixin::get_root_for_reflect,
                    Mixin::mut_root_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mixin>(
                    "Mixin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mixin {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_root();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mixin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mixin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19google/protobuf/api.proto\x12\x0fgoogle.protobuf\x1a$google/protob\
    uf/source_context.proto\x1a\x1agoogle/protobuf/type.proto\"\xc1\x02\n\
    \x03Api\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x121\n\x07methods\
    \x18\x02\x20\x03(\x0b2\x17.google.protobuf.MethodR\x07methods\x121\n\x07\
    options\x18\x03\x20\x03(\x0b2\x17.google.protobuf.OptionR\x07options\x12\
    \x18\n\x07version\x18\x04\x20\x01(\tR\x07version\x12E\n\x0esource_contex\
    t\x18\x05\x20\x01(\x0b2\x1e.google.protobuf.SourceContextR\rsourceContex\
    t\x12.\n\x06mixins\x18\x06\x20\x03(\x0b2\x16.google.protobuf.MixinR\x06m\
    ixins\x12/\n\x06syntax\x18\x07\x20\x01(\x0e2\x17.google.protobuf.SyntaxR\
    \x06syntax\"\xb2\x02\n\x06Method\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x12(\n\x10request_type_url\x18\x02\x20\x01(\tR\x0erequestTypeUr\
    l\x12+\n\x11request_streaming\x18\x03\x20\x01(\x08R\x10requestStreaming\
    \x12*\n\x11response_type_url\x18\x04\x20\x01(\tR\x0fresponseTypeUrl\x12-\
    \n\x12response_streaming\x18\x05\x20\x01(\x08R\x11responseStreaming\x121\
    \n\x07options\x18\x06\x20\x03(\x0b2\x17.google.protobuf.OptionR\x07optio\
    ns\x12/\n\x06syntax\x18\x07\x20\x01(\x0e2\x17.google.protobuf.SyntaxR\
    \x06syntax\"/\n\x05Mixin\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x12\n\x04root\x18\x02\x20\x01(\tR\x04rootBH\n\x13com.google.protobu\
    fB\x08ApiProtoP\x01\xa2\x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.WellKnownT\
    ypesJ\x92;\n\x07\x12\x05\x1e\0\xc8\x01\x01\n\xcc\x0c\n\x01\x0c\x12\x03\
    \x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20data\x20i\
    nterchange\x20format\n\x20Copyright\x202008\x20Google\x20Inc.\x20\x20All\
    \x20rights\x20reserved.\n\x20https://developers.google.com/protocol-buff\
    ers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\x20and\x20binar\
    y\x20forms,\x20with\x20or\x20without\n\x20modification,\x20are\x20permit\
    ted\x20provided\x20that\x20the\x20following\x20conditions\x20are\n\x20me\
    t:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\x20source\x20code\
    \x20must\x20retain\x20the\x20above\x20copyright\n\x20notice,\x20this\x20\
    list\x20of\x20conditions\x20and\x20the\x20following\x20disclaimer.\n\x20\
    \x20\x20\x20\x20*\x20Redistributions\x20in\x20binary\x20form\x20must\x20\
    reproduce\x20the\x20above\n\x20copyright\x20notice,\x20this\x20list\x20o\
    f\x20conditions\x20and\x20the\x20following\x20disclaimer\n\x20in\x20the\
    \x20documentation\x20and/or\x20other\x20materials\x20provided\x20with\
    \x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\x20Neither\x20the\x20n\
    ame\x20of\x20Google\x20Inc.\x20nor\x20the\x20names\x20of\x20its\n\x20con\
    tributors\x20may\x20be\x20used\x20to\x20endorse\x20or\x20promote\x20prod\
    ucts\x20derived\x20from\n\x20this\x20software\x20without\x20specific\x20\
    prior\x20written\x20permission.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDE\
    D\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\
    \x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INC\
    LUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIE\
    S\x20OF\x20MERCHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\
    \x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\
    \x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\
    \x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLAR\
    Y,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20L\
    IMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVI\
    CES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINES\
    S\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\
    \x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILIT\
    Y,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20AR\
    ISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20\
    SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20\
    SUCH\x20DAMAGE.\n\n\x08\n\x01\x02\x12\x03\x20\x08\x17\n\t\n\x02\x03\0\
    \x12\x03\"\x07-\n\t\n\x02\x03\x01\x12\x03#\x07#\n\x08\n\x01\x08\x12\x03%\
    \0;\n\x0b\n\x04\x08\xe7\x07\0\x12\x03%\0;\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03%\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03%\x07\x17\n\x0e\n\
    \x07\x08\xe7\x07\0\x02\0\x01\x12\x03%\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\
    \x07\x12\x03%\x1a:\n\x08\n\x01\x08\x12\x03&\0,\n\x0b\n\x04\x08\xe7\x07\
    \x01\x12\x03&\0,\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03&\x07\x13\n\r\n\
    \x06\x08\xe7\x07\x01\x02\0\x12\x03&\x07\x13\n\x0e\n\x07\x08\xe7\x07\x01\
    \x02\0\x01\x12\x03&\x07\x13\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03&\x16\
    +\n\x08\n\x01\x08\x12\x03'\0)\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03'\0)\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03'\x07\x1b\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03'\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03'\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03'\x1e(\n\x08\n\x01\
    \x08\x12\x03(\0\"\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03(\0\"\n\x0c\n\x05\
    \x08\xe7\x07\x03\x02\x12\x03(\x07\x1a\n\r\n\x06\x08\xe7\x07\x03\x02\0\
    \x12\x03(\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03(\x07\x1a\
    \n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03(\x1d!\n\x08\n\x01\x08\x12\x03)\
    \0!\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03)\0!\n\x0c\n\x05\x08\xe7\x07\x04\
    \x02\x12\x03)\x07\x18\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03)\x07\x18\n\
    \x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03)\x07\x18\n\x0c\n\x05\x08\
    \xe7\x07\x04\x07\x12\x03)\x1b\x20\nM\n\x02\x04\0\x12\x04,\0Y\x01\x1aA\
    \x20Api\x20is\x20a\x20light-weight\x20descriptor\x20for\x20a\x20protocol\
    \x20buffer\x20service.\n\n\n\n\x03\x04\0\x01\x12\x03,\x08\x0b\no\n\x04\
    \x04\0\x02\0\x12\x030\x02\x12\x1ab\x20The\x20fully\x20qualified\x20name\
    \x20of\x20this\x20api,\x20including\x20package\x20name\n\x20followed\x20\
    by\x20the\x20api's\x20simple\x20name.\n\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x040\x02,\r\n\x0c\n\x05\x04\0\x02\0\x05\x12\x030\x02\x08\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x030\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x030\x10\
    \x11\n=\n\x04\x04\0\x02\x01\x12\x033\x02\x1e\x1a0\x20The\x20methods\x20o\
    f\x20this\x20api,\x20in\x20unspecified\x20order.\n\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x033\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x033\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x033\x12\x19\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x033\x1c\x1d\n0\n\x04\x04\0\x02\x02\x12\x036\x02\x1e\x1a#\x20An\
    y\x20metadata\x20attached\x20to\x20the\x20API.\n\n\x0c\n\x05\x04\0\x02\
    \x02\x04\x12\x036\x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x036\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x036\x12\x19\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x036\x1c\x1d\n\xf2\x07\n\x04\x04\0\x02\x03\x12\x03N\x02\x15\x1a\
    \xe4\x07\x20A\x20version\x20string\x20for\x20this\x20api.\x20If\x20speci\
    fied,\x20must\x20have\x20the\x20form\n\x20`major-version.minor-version`,\
    \x20as\x20in\x20`1.10`.\x20If\x20the\x20minor\x20version\n\x20is\x20omit\
    ted,\x20it\x20defaults\x20to\x20zero.\x20If\x20the\x20entire\x20version\
    \x20field\x20is\n\x20empty,\x20the\x20major\x20version\x20is\x20derived\
    \x20from\x20the\x20package\x20name,\x20as\n\x20outlined\x20below.\x20If\
    \x20the\x20field\x20is\x20not\x20empty,\x20the\x20version\x20in\x20the\n\
    \x20package\x20name\x20will\x20be\x20verified\x20to\x20be\x20consistent\
    \x20with\x20what\x20is\n\x20provided\x20here.\n\n\x20The\x20versioning\
    \x20schema\x20uses\x20[semantic\n\x20versioning](http://semver.org)\x20w\
    here\x20the\x20major\x20version\x20number\n\x20indicates\x20a\x20breakin\
    g\x20change\x20and\x20the\x20minor\x20version\x20an\x20additive,\n\x20no\
    n-breaking\x20change.\x20Both\x20version\x20numbers\x20are\x20signals\
    \x20to\x20users\n\x20what\x20to\x20expect\x20from\x20different\x20versio\
    ns,\x20and\x20should\x20be\x20carefully\n\x20chosen\x20based\x20on\x20th\
    e\x20product\x20plan.\n\n\x20The\x20major\x20version\x20is\x20also\x20re\
    flected\x20in\x20the\x20package\x20name\x20of\x20the\n\x20API,\x20which\
    \x20must\x20end\x20in\x20`v<major-version>`,\x20as\x20in\n\x20`google.fe\
    ature.v1`.\x20For\x20major\x20versions\x200\x20and\x201,\x20the\x20suffi\
    x\x20can\n\x20be\x20omitted.\x20Zero\x20major\x20versions\x20must\x20onl\
    y\x20be\x20used\x20for\n\x20experimental,\x20none-GA\x20apis.\n\n\n\n\r\
    \n\x05\x04\0\x02\x03\x04\x12\x04N\x026\x1e\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03N\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03N\t\x10\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03N\x13\x14\n[\n\x04\x04\0\x02\x04\x12\x03R\
    \x02#\x1aN\x20Source\x20context\x20for\x20the\x20protocol\x20buffer\x20s\
    ervice\x20represented\x20by\x20this\n\x20message.\n\n\r\n\x05\x04\0\x02\
    \x04\x04\x12\x04R\x02N\x15\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03R\x02\
    \x0f\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03R\x10\x1e\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03R!\"\n,\n\x04\x04\0\x02\x05\x12\x03U\x02\x1c\x1a\x1f\
    \x20Included\x20APIs.\x20See\x20[Mixin][].\n\n\x0c\n\x05\x04\0\x02\x05\
    \x04\x12\x03U\x02\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03U\x0b\x10\n\x0c\
    \n\x05\x04\0\x02\x05\x01\x12\x03U\x11\x17\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03U\x1a\x1b\n0\n\x04\x04\0\x02\x06\x12\x03X\x02\x14\x1a#\x20The\
    \x20source\x20syntax\x20of\x20the\x20service.\n\n\r\n\x05\x04\0\x02\x06\
    \x04\x12\x04X\x02U\x1c\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03X\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x06\x01\x12\x03X\t\x0f\n\x0c\n\x05\x04\0\x02\x06\
    \x03\x12\x03X\x12\x13\n3\n\x02\x04\x01\x12\x04\\\0r\x01\x1a'\x20Method\
    \x20represents\x20a\x20method\x20of\x20an\x20api.\n\n\n\n\x03\x04\x01\
    \x01\x12\x03\\\x08\x0e\n.\n\x04\x04\x01\x02\0\x12\x03_\x02\x12\x1a!\x20T\
    he\x20simple\x20name\x20of\x20this\x20method.\n\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04_\x02\\\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03_\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03_\t\r\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03_\x10\x11\n/\n\x04\x04\x01\x02\x01\x12\x03b\x02\x1e\x1a\"\x20A\
    \x20URL\x20of\x20the\x20input\x20message\x20type.\n\n\r\n\x05\x04\x01\
    \x02\x01\x04\x12\x04b\x02_\x12\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03b\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03b\t\x19\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03b\x1c\x1d\n0\n\x04\x04\x01\x02\x02\x12\x03e\x02\
    \x1d\x1a#\x20If\x20true,\x20the\x20request\x20is\x20streamed.\n\n\r\n\
    \x05\x04\x01\x02\x02\x04\x12\x04e\x02b\x1e\n\x0c\n\x05\x04\x01\x02\x02\
    \x05\x12\x03e\x02\x06\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03e\x07\x18\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03e\x1b\x1c\n2\n\x04\x04\x01\x02\x03\
    \x12\x03h\x02\x1f\x1a%\x20The\x20URL\x20of\x20the\x20output\x20message\
    \x20type.\n\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04h\x02e\x1d\n\x0c\n\x05\
    \x04\x01\x02\x03\x05\x12\x03h\x02\x08\n\x0c\n\x05\x04\x01\x02\x03\x01\
    \x12\x03h\t\x1a\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03h\x1d\x1e\n1\n\
    \x04\x04\x01\x02\x04\x12\x03k\x02\x1e\x1a$\x20If\x20true,\x20the\x20resp\
    onse\x20is\x20streamed.\n\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04k\x02h\
    \x1f\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03k\x02\x06\n\x0c\n\x05\x04\
    \x01\x02\x04\x01\x12\x03k\x07\x19\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\
    \x03k\x1c\x1d\n3\n\x04\x04\x01\x02\x05\x12\x03n\x02\x1e\x1a&\x20Any\x20m\
    etadata\x20attached\x20to\x20the\x20method.\n\n\x0c\n\x05\x04\x01\x02\
    \x05\x04\x12\x03n\x02\n\n\x0c\n\x05\x04\x01\x02\x05\x06\x12\x03n\x0b\x11\
    \n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03n\x12\x19\n\x0c\n\x05\x04\x01\
    \x02\x05\x03\x12\x03n\x1c\x1d\n0\n\x04\x04\x01\x02\x06\x12\x03q\x02\x14\
    \x1a#\x20The\x20source\x20syntax\x20of\x20this\x20method.\n\n\r\n\x05\
    \x04\x01\x02\x06\x04\x12\x04q\x02n\x1e\n\x0c\n\x05\x04\x01\x02\x06\x06\
    \x12\x03q\x02\x08\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03q\t\x0f\n\x0c\n\
    \x05\x04\x01\x02\x06\x03\x12\x03q\x12\x13\n\xa4\x13\n\x02\x04\x02\x12\
    \x06\xc1\x01\0\xc8\x01\x01\x1a\x95\x13\x20Declares\x20an\x20API\x20to\
    \x20be\x20included\x20in\x20this\x20API.\x20The\x20including\x20API\x20m\
    ust\n\x20redeclare\x20all\x20the\x20methods\x20from\x20the\x20included\
    \x20API,\x20but\x20documentation\n\x20and\x20options\x20are\x20inherited\
    \x20as\x20follows:\n\n\x20-\x20If\x20after\x20comment\x20and\x20whitespa\
    ce\x20stripping,\x20the\x20documentation\n\x20\x20\x20string\x20of\x20th\
    e\x20redeclared\x20method\x20is\x20empty,\x20it\x20will\x20be\x20inherit\
    ed\n\x20\x20\x20from\x20the\x20original\x20method.\n\n\x20-\x20Each\x20a\
    nnotation\x20belonging\x20to\x20the\x20service\x20config\x20(http,\n\x20\
    \x20\x20visibility)\x20which\x20is\x20not\x20set\x20in\x20the\x20redecla\
    red\x20method\x20will\x20be\n\x20\x20\x20inherited.\n\n\x20-\x20If\x20an\
    \x20http\x20annotation\x20is\x20inherited,\x20the\x20path\x20pattern\x20\
    will\x20be\n\x20\x20\x20modified\x20as\x20follows.\x20Any\x20version\x20\
    prefix\x20will\x20be\x20replaced\x20by\x20the\n\x20\x20\x20version\x20of\
    \x20the\x20including\x20API\x20plus\x20the\x20[root][]\x20path\x20if\x20\
    specified.\n\n\x20Example\x20of\x20a\x20simple\x20mixin:\n\n\x20\x20\x20\
    \x20\x20package\x20google.acl.v1;\n\x20\x20\x20\x20\x20service\x20Access\
    Control\x20{\n\x20\x20\x20\x20\x20\x20\x20//\x20Get\x20the\x20underlying\
    \x20ACL\x20object.\n\x20\x20\x20\x20\x20\x20\x20rpc\x20GetAcl(GetAclRequ\
    est)\x20returns\x20(Acl)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20optio\
    n\x20(google.api.http).get\x20=\x20\"/v1/{resource=**}:getAcl\";\n\x20\
    \x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x20\x20\x20\x20\x20\
    package\x20google.storage.v2;\n\x20\x20\x20\x20\x20service\x20Storage\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20rpc\x20GetAcl(GetAclRequest)\x20retur\
    ns\x20(Acl);\n\n\x20\x20\x20\x20\x20\x20\x20//\x20Get\x20a\x20data\x20re\
    cord.\n\x20\x20\x20\x20\x20\x20\x20rpc\x20GetData(GetDataRequest)\x20ret\
    urns\x20(Data)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20option\x20(goog\
    le.api.http).get\x20=\x20\"/v2/{resource=**}\";\n\x20\x20\x20\x20\x20\
    \x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x20Example\x20of\x20a\x20mixin\x20c\
    onfiguration:\n\n\x20\x20\x20\x20\x20apis:\n\x20\x20\x20\x20\x20-\x20nam\
    e:\x20google.storage.v2.Storage\n\x20\x20\x20\x20\x20\x20\x20mixins:\n\
    \x20\x20\x20\x20\x20\x20\x20-\x20name:\x20google.acl.v1.AccessControl\n\
    \n\x20The\x20mixin\x20construct\x20implies\x20that\x20all\x20methods\x20\
    in\x20`AccessControl`\x20are\n\x20also\x20declared\x20with\x20same\x20na\
    me\x20and\x20request/response\x20types\x20in\n\x20`Storage`.\x20A\x20doc\
    umentation\x20generator\x20or\x20annotation\x20processor\x20will\n\x20se\
    e\x20the\x20effective\x20`Storage.GetAcl`\x20method\x20after\x20inhertin\
    g\n\x20documentation\x20and\x20annotations\x20as\x20follows:\n\n\x20\x20\
    \x20\x20\x20service\x20Storage\x20{\n\x20\x20\x20\x20\x20\x20\x20//\x20G\
    et\x20the\x20underlying\x20ACL\x20object.\n\x20\x20\x20\x20\x20\x20\x20r\
    pc\x20GetAcl(GetAclRequest)\x20returns\x20(Acl)\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20option\x20(google.api.http).get\x20=\x20\"/v2/{resou\
    rce=**}:getAcl\";\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\
    \x20\x20...\n\x20\x20\x20\x20\x20}\n\n\x20Note\x20how\x20the\x20version\
    \x20in\x20the\x20path\x20pattern\x20changed\x20from\x20`v1`\x20to\x20`v2\
    `.\n\n\x20If\x20the\x20`root`\x20field\x20in\x20the\x20mixin\x20is\x20sp\
    ecified,\x20it\x20should\x20be\x20a\n\x20relative\x20path\x20under\x20wh\
    ich\x20inherited\x20HTTP\x20paths\x20are\x20placed.\x20Example:\n\n\x20\
    \x20\x20\x20\x20apis:\n\x20\x20\x20\x20\x20-\x20name:\x20google.storage.\
    v2.Storage\n\x20\x20\x20\x20\x20\x20\x20mixins:\n\x20\x20\x20\x20\x20\
    \x20\x20-\x20name:\x20google.acl.v1.AccessControl\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20root:\x20acls\n\n\x20This\x20implies\x20the\x20following\
    \x20inherited\x20HTTP\x20annotation:\n\n\x20\x20\x20\x20\x20service\x20S\
    torage\x20{\n\x20\x20\x20\x20\x20\x20\x20//\x20Get\x20the\x20underlying\
    \x20ACL\x20object.\n\x20\x20\x20\x20\x20\x20\x20rpc\x20GetAcl(GetAclRequ\
    est)\x20returns\x20(Acl)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20optio\
    n\x20(google.api.http).get\x20=\x20\"/v2/acls/{resource=**}:getAcl\";\n\
    \x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20...\n\x20\x20\
    \x20\x20\x20}\n\n\x0b\n\x03\x04\x02\x01\x12\x04\xc1\x01\x08\r\nF\n\x04\
    \x04\x02\x02\0\x12\x04\xc3\x01\x02\x12\x1a8\x20The\x20fully\x20qualified\
    \x20name\x20of\x20the\x20API\x20which\x20is\x20included.\n\n\x0f\n\x05\
    \x04\x02\x02\0\x04\x12\x06\xc3\x01\x02\xc1\x01\x0f\n\r\n\x05\x04\x02\x02\
    \0\x05\x12\x04\xc3\x01\x02\x08\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\xc3\
    \x01\t\r\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\xc3\x01\x10\x11\n[\n\x04\
    \x04\x02\x02\x01\x12\x04\xc7\x01\x02\x12\x1aM\x20If\x20non-empty\x20spec\
    ifies\x20a\x20path\x20under\x20which\x20inherited\x20HTTP\x20paths\n\x20\
    are\x20rooted.\n\n\x0f\n\x05\x04\x02\x02\x01\x04\x12\x06\xc7\x01\x02\xc3\
    \x01\x12\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\xc7\x01\x02\x08\n\r\n\x05\
    \x04\x02\x02\x01\x01\x12\x04\xc7\x01\t\r\n\r\n\x05\x04\x02\x02\x01\x03\
    \x12\x04\xc7\x01\x10\x11b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
