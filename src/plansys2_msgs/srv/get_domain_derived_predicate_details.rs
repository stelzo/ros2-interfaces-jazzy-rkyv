use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainDerivedPredicateDetailsRequest {
    pub predicate: ::std::string::String,
}

impl Default for GetDomainDerivedPredicateDetailsRequest {
    fn default() -> Self {
        GetDomainDerivedPredicateDetailsRequest {
            predicate: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainDerivedPredicateDetailsResponse {
    pub predicates: Vec<crate::plansys2_msgs::msg::Derived>,
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainDerivedPredicateDetailsResponse {
    fn default() -> Self {
        GetDomainDerivedPredicateDetailsResponse {
            predicates: Vec::new(),
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainDerivedPredicateDetails;
