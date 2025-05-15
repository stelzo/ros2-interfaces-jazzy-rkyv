use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FieldType {
    pub type_id: u8, // default: 0
    pub capacity: u64,
    pub string_capacity: u64,
    pub nested_type_name: ::std::string::String,
}

impl FieldType {
    pub const FIELD_TYPE_NOT_SET: u8 = 0;
    pub const FIELD_TYPE_NESTED_TYPE: u8 = 1;
    pub const FIELD_TYPE_INT8: u8 = 2;
    pub const FIELD_TYPE_UINT8: u8 = 3;
    pub const FIELD_TYPE_INT16: u8 = 4;
    pub const FIELD_TYPE_UINT16: u8 = 5;
    pub const FIELD_TYPE_INT32: u8 = 6;
    pub const FIELD_TYPE_UINT32: u8 = 7;
    pub const FIELD_TYPE_INT64: u8 = 8;
    pub const FIELD_TYPE_UINT64: u8 = 9;
    pub const FIELD_TYPE_FLOAT: u8 = 10;
    pub const FIELD_TYPE_DOUBLE: u8 = 11;
    pub const FIELD_TYPE_LONG_DOUBLE: u8 = 12;
    pub const FIELD_TYPE_CHAR: u8 = 13;
    pub const FIELD_TYPE_WCHAR: u8 = 14;
    pub const FIELD_TYPE_BOOLEAN: u8 = 15;
    pub const FIELD_TYPE_BYTE: u8 = 16;
    pub const FIELD_TYPE_STRING: u8 = 17;
    pub const FIELD_TYPE_WSTRING: u8 = 18;
    pub const FIELD_TYPE_FIXED_STRING: u8 = 19;
    pub const FIELD_TYPE_FIXED_WSTRING: u8 = 20;
    pub const FIELD_TYPE_BOUNDED_STRING: u8 = 21;
    pub const FIELD_TYPE_BOUNDED_WSTRING: u8 = 22;
    pub const FIELD_TYPE_NESTED_TYPE_ARRAY: u8 = 49;
    pub const FIELD_TYPE_INT8_ARRAY: u8 = 50;
    pub const FIELD_TYPE_UINT8_ARRAY: u8 = 51;
    pub const FIELD_TYPE_INT16_ARRAY: u8 = 52;
    pub const FIELD_TYPE_UINT16_ARRAY: u8 = 53;
    pub const FIELD_TYPE_INT32_ARRAY: u8 = 54;
    pub const FIELD_TYPE_UINT32_ARRAY: u8 = 55;
    pub const FIELD_TYPE_INT64_ARRAY: u8 = 56;
    pub const FIELD_TYPE_UINT64_ARRAY: u8 = 57;
    pub const FIELD_TYPE_FLOAT_ARRAY: u8 = 58;
    pub const FIELD_TYPE_DOUBLE_ARRAY: u8 = 59;
    pub const FIELD_TYPE_LONG_DOUBLE_ARRAY: u8 = 60;
    pub const FIELD_TYPE_CHAR_ARRAY: u8 = 61;
    pub const FIELD_TYPE_WCHAR_ARRAY: u8 = 62;
    pub const FIELD_TYPE_BOOLEAN_ARRAY: u8 = 63;
    pub const FIELD_TYPE_BYTE_ARRAY: u8 = 64;
    pub const FIELD_TYPE_STRING_ARRAY: u8 = 65;
    pub const FIELD_TYPE_WSTRING_ARRAY: u8 = 66;
    pub const FIELD_TYPE_FIXED_STRING_ARRAY: u8 = 67;
    pub const FIELD_TYPE_FIXED_WSTRING_ARRAY: u8 = 68;
    pub const FIELD_TYPE_BOUNDED_STRING_ARRAY: u8 = 69;
    pub const FIELD_TYPE_BOUNDED_WSTRING_ARRAY: u8 = 70;
    pub const FIELD_TYPE_NESTED_TYPE_BOUNDED_SEQUENCE: u8 = 97;
    pub const FIELD_TYPE_INT8_BOUNDED_SEQUENCE: u8 = 98;
    pub const FIELD_TYPE_UINT8_BOUNDED_SEQUENCE: u8 = 99;
    pub const FIELD_TYPE_INT16_BOUNDED_SEQUENCE: u8 = 100;
    pub const FIELD_TYPE_UINT16_BOUNDED_SEQUENCE: u8 = 101;
    pub const FIELD_TYPE_INT32_BOUNDED_SEQUENCE: u8 = 102;
    pub const FIELD_TYPE_UINT32_BOUNDED_SEQUENCE: u8 = 103;
    pub const FIELD_TYPE_INT64_BOUNDED_SEQUENCE: u8 = 104;
    pub const FIELD_TYPE_UINT64_BOUNDED_SEQUENCE: u8 = 105;
    pub const FIELD_TYPE_FLOAT_BOUNDED_SEQUENCE: u8 = 106;
    pub const FIELD_TYPE_DOUBLE_BOUNDED_SEQUENCE: u8 = 107;
    pub const FIELD_TYPE_LONG_DOUBLE_BOUNDED_SEQUENCE: u8 = 108;
    pub const FIELD_TYPE_CHAR_BOUNDED_SEQUENCE: u8 = 109;
    pub const FIELD_TYPE_WCHAR_BOUNDED_SEQUENCE: u8 = 110;
    pub const FIELD_TYPE_BOOLEAN_BOUNDED_SEQUENCE: u8 = 111;
    pub const FIELD_TYPE_BYTE_BOUNDED_SEQUENCE: u8 = 112;
    pub const FIELD_TYPE_STRING_BOUNDED_SEQUENCE: u8 = 113;
    pub const FIELD_TYPE_WSTRING_BOUNDED_SEQUENCE: u8 = 114;
    pub const FIELD_TYPE_FIXED_STRING_BOUNDED_SEQUENCE: u8 = 115;
    pub const FIELD_TYPE_FIXED_WSTRING_BOUNDED_SEQUENCE: u8 = 116;
    pub const FIELD_TYPE_BOUNDED_STRING_BOUNDED_SEQUENCE: u8 = 117;
    pub const FIELD_TYPE_BOUNDED_WSTRING_BOUNDED_SEQUENCE: u8 = 118;
    pub const FIELD_TYPE_NESTED_TYPE_UNBOUNDED_SEQUENCE: u8 = 145;
    pub const FIELD_TYPE_INT8_UNBOUNDED_SEQUENCE: u8 = 146;
    pub const FIELD_TYPE_UINT8_UNBOUNDED_SEQUENCE: u8 = 147;
    pub const FIELD_TYPE_INT16_UNBOUNDED_SEQUENCE: u8 = 148;
    pub const FIELD_TYPE_UINT16_UNBOUNDED_SEQUENCE: u8 = 149;
    pub const FIELD_TYPE_INT32_UNBOUNDED_SEQUENCE: u8 = 150;
    pub const FIELD_TYPE_UINT32_UNBOUNDED_SEQUENCE: u8 = 151;
    pub const FIELD_TYPE_INT64_UNBOUNDED_SEQUENCE: u8 = 152;
    pub const FIELD_TYPE_UINT64_UNBOUNDED_SEQUENCE: u8 = 153;
    pub const FIELD_TYPE_FLOAT_UNBOUNDED_SEQUENCE: u8 = 154;
    pub const FIELD_TYPE_DOUBLE_UNBOUNDED_SEQUENCE: u8 = 155;
    pub const FIELD_TYPE_LONG_DOUBLE_UNBOUNDED_SEQUENCE: u8 = 156;
    pub const FIELD_TYPE_CHAR_UNBOUNDED_SEQUENCE: u8 = 157;
    pub const FIELD_TYPE_WCHAR_UNBOUNDED_SEQUENCE: u8 = 158;
    pub const FIELD_TYPE_BOOLEAN_UNBOUNDED_SEQUENCE: u8 = 159;
    pub const FIELD_TYPE_BYTE_UNBOUNDED_SEQUENCE: u8 = 160;
    pub const FIELD_TYPE_STRING_UNBOUNDED_SEQUENCE: u8 = 161;
    pub const FIELD_TYPE_WSTRING_UNBOUNDED_SEQUENCE: u8 = 162;
    pub const FIELD_TYPE_FIXED_STRING_UNBOUNDED_SEQUENCE: u8 = 163;
    pub const FIELD_TYPE_FIXED_WSTRING_UNBOUNDED_SEQUENCE: u8 = 164;
    pub const FIELD_TYPE_BOUNDED_STRING_UNBOUNDED_SEQUENCE: u8 = 165;
    pub const FIELD_TYPE_BOUNDED_WSTRING_UNBOUNDED_SEQUENCE: u8 = 166;
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType {
            type_id: 0,
            capacity: 0,
            string_capacity: 0,
            nested_type_name: ::std::string::String::new(),
        }
    }
}
