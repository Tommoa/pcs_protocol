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
pub struct Verify {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Verify {}

impl Verify {
    pub fn new() -> Verify {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Verify {
        static mut instance: ::protobuf::lazy::Lazy<Verify> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Verify,
        };
        unsafe {
            instance.get(Verify::new)
        }
    }
}

impl ::protobuf::Message for Verify {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for Verify {
    fn new() -> Verify {
        Verify::new()
    }

    fn descriptor_static(_: ::std::option::Option<Verify>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Verify>(
                    "Verify",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Verify {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Verify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Verify {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Decline {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Decline {}

impl Decline {
    pub fn new() -> Decline {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Decline {
        static mut instance: ::protobuf::lazy::Lazy<Decline> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Decline,
        };
        unsafe {
            instance.get(Decline::new)
        }
    }
}

impl ::protobuf::Message for Decline {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for Decline {
    fn new() -> Decline {
        Decline::new()
    }

    fn descriptor_static(_: ::std::option::Option<Decline>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Decline>(
                    "Decline",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Decline {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Decline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Decline {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Accept {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Accept {}

impl Accept {
    pub fn new() -> Accept {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Accept {
        static mut instance: ::protobuf::lazy::Lazy<Accept> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Accept,
        };
        unsafe {
            instance.get(Accept::new)
        }
    }
}

impl ::protobuf::Message for Accept {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for Accept {
    fn new() -> Accept {
        Accept::new()
    }

    fn descriptor_static(_: ::std::option::Option<Accept>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Accept>(
                    "Accept",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Accept {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Accept {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Accept {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mark {
    // message fields
    pub question_hash: ::std::vec::Vec<u32>,
    pub batch: u32,
    pub answer: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mark {}

impl Mark {
    pub fn new() -> Mark {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mark {
        static mut instance: ::protobuf::lazy::Lazy<Mark> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mark,
        };
        unsafe {
            instance.get(Mark::new)
        }
    }

    // repeated uint32 question_hash = 1;

    pub fn clear_question_hash(&mut self) {
        self.question_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_question_hash(&mut self, v: ::std::vec::Vec<u32>) {
        self.question_hash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_question_hash(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.question_hash
    }

    // Take field
    pub fn take_question_hash(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.question_hash, ::std::vec::Vec::new())
    }

    pub fn get_question_hash(&self) -> &[u32] {
        &self.question_hash
    }

    fn get_question_hash_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.question_hash
    }

    fn mut_question_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.question_hash
    }

    // uint32 batch = 2;

    pub fn clear_batch(&mut self) {
        self.batch = 0;
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: u32) {
        self.batch = v;
    }

    pub fn get_batch(&self) -> u32 {
        self.batch
    }

    fn get_batch_for_reflect(&self) -> &u32 {
        &self.batch
    }

    fn mut_batch_for_reflect(&mut self) -> &mut u32 {
        &mut self.batch
    }

    // string answer = 3;

    pub fn clear_answer(&mut self) {
        self.answer.clear();
    }

    // Param is passed by value, moved
    pub fn set_answer(&mut self, v: ::std::string::String) {
        self.answer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_answer(&mut self) -> &mut ::std::string::String {
        &mut self.answer
    }

    // Take field
    pub fn take_answer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.answer, ::std::string::String::new())
    }

    pub fn get_answer(&self) -> &str {
        &self.answer
    }

    fn get_answer_for_reflect(&self) -> &::std::string::String {
        &self.answer
    }

    fn mut_answer_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.answer
    }
}

impl ::protobuf::Message for Mark {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.question_hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.batch = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.answer)?;
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
        if !self.question_hash.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.question_hash);
        }
        if self.batch != 0 {
            my_size += ::protobuf::rt::value_size(2, self.batch, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.answer.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.answer);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.question_hash.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.question_hash))?;
            for v in &self.question_hash {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if self.batch != 0 {
            os.write_uint32(2, self.batch)?;
        }
        if !self.answer.is_empty() {
            os.write_string(3, &self.answer)?;
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

impl ::protobuf::MessageStatic for Mark {
    fn new() -> Mark {
        Mark::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mark>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "question_hash",
                    Mark::get_question_hash_for_reflect,
                    Mark::mut_question_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "batch",
                    Mark::get_batch_for_reflect,
                    Mark::mut_batch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "answer",
                    Mark::get_answer_for_reflect,
                    Mark::mut_answer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mark>(
                    "Mark",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mark {
    fn clear(&mut self) {
        self.clear_question_hash();
        self.clear_batch();
        self.clear_answer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mark {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mark {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Result {
    // message fields
    pub batch: u32,
    pub test: u32,
    pub result: Result_RESULT,
    pub time_sec: i64,
    pub time_usec: i64,
    pub blocked: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Result {}

impl Result {
    pub fn new() -> Result {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Result {
        static mut instance: ::protobuf::lazy::Lazy<Result> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Result,
        };
        unsafe {
            instance.get(Result::new)
        }
    }

    // uint32 batch = 1;

    pub fn clear_batch(&mut self) {
        self.batch = 0;
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: u32) {
        self.batch = v;
    }

    pub fn get_batch(&self) -> u32 {
        self.batch
    }

    fn get_batch_for_reflect(&self) -> &u32 {
        &self.batch
    }

    fn mut_batch_for_reflect(&mut self) -> &mut u32 {
        &mut self.batch
    }

    // uint32 test = 2;

    pub fn clear_test(&mut self) {
        self.test = 0;
    }

    // Param is passed by value, moved
    pub fn set_test(&mut self, v: u32) {
        self.test = v;
    }

    pub fn get_test(&self) -> u32 {
        self.test
    }

    fn get_test_for_reflect(&self) -> &u32 {
        &self.test
    }

    fn mut_test_for_reflect(&mut self) -> &mut u32 {
        &mut self.test
    }

    // .Result.RESULT result = 3;

    pub fn clear_result(&mut self) {
        self.result = Result_RESULT::FAIL;
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: Result_RESULT) {
        self.result = v;
    }

    pub fn get_result(&self) -> Result_RESULT {
        self.result
    }

    fn get_result_for_reflect(&self) -> &Result_RESULT {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut Result_RESULT {
        &mut self.result
    }

    // int64 time_sec = 4;

    pub fn clear_time_sec(&mut self) {
        self.time_sec = 0;
    }

    // Param is passed by value, moved
    pub fn set_time_sec(&mut self, v: i64) {
        self.time_sec = v;
    }

    pub fn get_time_sec(&self) -> i64 {
        self.time_sec
    }

    fn get_time_sec_for_reflect(&self) -> &i64 {
        &self.time_sec
    }

    fn mut_time_sec_for_reflect(&mut self) -> &mut i64 {
        &mut self.time_sec
    }

    // int64 time_usec = 5;

    pub fn clear_time_usec(&mut self) {
        self.time_usec = 0;
    }

    // Param is passed by value, moved
    pub fn set_time_usec(&mut self, v: i64) {
        self.time_usec = v;
    }

    pub fn get_time_usec(&self) -> i64 {
        self.time_usec
    }

    fn get_time_usec_for_reflect(&self) -> &i64 {
        &self.time_usec
    }

    fn mut_time_usec_for_reflect(&mut self) -> &mut i64 {
        &mut self.time_usec
    }

    // uint32 blocked = 6;

    pub fn clear_blocked(&mut self) {
        self.blocked = 0;
    }

    // Param is passed by value, moved
    pub fn set_blocked(&mut self, v: u32) {
        self.blocked = v;
    }

    pub fn get_blocked(&self) -> u32 {
        self.blocked
    }

    fn get_blocked_for_reflect(&self) -> &u32 {
        &self.blocked
    }

    fn mut_blocked_for_reflect(&mut self) -> &mut u32 {
        &mut self.blocked
    }
}

impl ::protobuf::Message for Result {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.batch = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.test = tmp;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.result, 3, &mut self.unknown_fields)?
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.time_sec = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.time_usec = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.blocked = tmp;
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
        if self.batch != 0 {
            my_size += ::protobuf::rt::value_size(1, self.batch, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.test != 0 {
            my_size += ::protobuf::rt::value_size(2, self.test, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.result != Result_RESULT::FAIL {
            my_size += ::protobuf::rt::enum_size(3, self.result);
        }
        if self.time_sec != 0 {
            my_size += ::protobuf::rt::value_size(4, self.time_sec, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.time_usec != 0 {
            my_size += ::protobuf::rt::value_size(5, self.time_usec, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.blocked != 0 {
            my_size += ::protobuf::rt::value_size(6, self.blocked, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.batch != 0 {
            os.write_uint32(1, self.batch)?;
        }
        if self.test != 0 {
            os.write_uint32(2, self.test)?;
        }
        if self.result != Result_RESULT::FAIL {
            os.write_enum(3, self.result.value())?;
        }
        if self.time_sec != 0 {
            os.write_int64(4, self.time_sec)?;
        }
        if self.time_usec != 0 {
            os.write_int64(5, self.time_usec)?;
        }
        if self.blocked != 0 {
            os.write_uint32(6, self.blocked)?;
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

impl ::protobuf::MessageStatic for Result {
    fn new() -> Result {
        Result::new()
    }

    fn descriptor_static(_: ::std::option::Option<Result>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "batch",
                    Result::get_batch_for_reflect,
                    Result::mut_batch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "test",
                    Result::get_test_for_reflect,
                    Result::mut_test_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Result_RESULT>>(
                    "result",
                    Result::get_result_for_reflect,
                    Result::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "time_sec",
                    Result::get_time_sec_for_reflect,
                    Result::mut_time_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "time_usec",
                    Result::get_time_usec_for_reflect,
                    Result::mut_time_usec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "blocked",
                    Result::get_blocked_for_reflect,
                    Result::mut_blocked_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Result>(
                    "Result",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Result {
    fn clear(&mut self) {
        self.clear_batch();
        self.clear_test();
        self.clear_result();
        self.clear_time_sec();
        self.clear_time_usec();
        self.clear_blocked();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Result {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Result_RESULT {
    FAIL = 0,
    SUCCESS = 1,
    CE = 2,
    RTE = 3,
    TLE = 4,
    BLOCKED = 5,
}

impl ::protobuf::ProtobufEnum for Result_RESULT {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Result_RESULT> {
        match value {
            0 => ::std::option::Option::Some(Result_RESULT::FAIL),
            1 => ::std::option::Option::Some(Result_RESULT::SUCCESS),
            2 => ::std::option::Option::Some(Result_RESULT::CE),
            3 => ::std::option::Option::Some(Result_RESULT::RTE),
            4 => ::std::option::Option::Some(Result_RESULT::TLE),
            5 => ::std::option::Option::Some(Result_RESULT::BLOCKED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Result_RESULT] = &[
            Result_RESULT::FAIL,
            Result_RESULT::SUCCESS,
            Result_RESULT::CE,
            Result_RESULT::RTE,
            Result_RESULT::TLE,
            Result_RESULT::BLOCKED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Result_RESULT>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Result_RESULT", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Result_RESULT {
}

impl ::std::default::Default for Result_RESULT {
    fn default() -> Self {
        Result_RESULT::FAIL
    }
}

impl ::protobuf::reflect::ProtobufValue for Result_RESULT {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tpcs.proto\"\x08\n\x06Verify\"\t\n\x07Decline\"\x08\n\x06Accept\"]\n\
    \x04Mark\x12'\n\rquestion_hash\x18\x01\x20\x03(\rR\x0cquestionHashB\x02\
    \x10\x01\x12\x14\n\x05batch\x18\x02\x20\x01(\rR\x05batch\x12\x16\n\x06an\
    swer\x18\x03\x20\x01(\tR\x06answer\"\xf4\x01\n\x06Result\x12\x14\n\x05ba\
    tch\x18\x01\x20\x01(\rR\x05batch\x12\x12\n\x04test\x18\x02\x20\x01(\rR\
    \x04test\x12&\n\x06result\x18\x03\x20\x01(\x0e2\x0e.Result.RESULTR\x06re\
    sult\x12\x19\n\x08time_sec\x18\x04\x20\x01(\x03R\x07timeSec\x12\x1b\n\tt\
    ime_usec\x18\x05\x20\x01(\x03R\x08timeUsec\x12\x18\n\x07blocked\x18\x06\
    \x20\x01(\rR\x07blocked\"F\n\x06RESULT\x12\x08\n\x04FAIL\x10\0\x12\x0b\n\
    \x07SUCCESS\x10\x01\x12\x06\n\x02CE\x10\x02\x12\x07\n\x03RTE\x10\x03\x12\
    \x07\n\x03TLE\x10\x04\x12\x0b\n\x07BLOCKED\x10\x05b\x06proto3\
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
