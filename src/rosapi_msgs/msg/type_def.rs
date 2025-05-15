use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TypeDef {
    pub r#type: ::std::string::String,
    pub fieldnames: Vec<::std::string::String>,
    pub fieldtypes: Vec<::std::string::String>,
    pub fieldarraylen: Vec<i32>,
    pub examples: Vec<::std::string::String>,
    pub constnames: Vec<::std::string::String>,
    pub constvalues: Vec<::std::string::String>,
}

impl Default for TypeDef {
    fn default() -> Self {
        TypeDef {
            r#type: ::std::string::String::new(),
            fieldnames: Vec::new(),
            fieldtypes: Vec::new(),
            fieldarraylen: Vec::new(),
            examples: Vec::new(),
            constnames: Vec::new(),
            constvalues: Vec::new(),
        }
    }
}
