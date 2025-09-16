use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidNameError {}
impl Error for InvalidNameError {}
impl Display for InvalidNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"InvalidNameError")
    }
}

#[derive(Clone)]
pub struct ResourceName {
    pub namespace: String,
    pub identifier: String
}

impl <'s> TryFrom<&'s str> for ResourceName {
    type Error = InvalidNameError;
    
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        match value.split_once(':') {
            Some((namespace,identifier)) => Ok(Self{ namespace: String::from(namespace), identifier: String::from(identifier) }),
            None => Err(InvalidNameError {}),
        }
    }
}

impl Display for ResourceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}:{}",self.namespace,self.identifier)
    }
}