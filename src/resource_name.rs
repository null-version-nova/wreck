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
pub struct ResourceName<'s> {
    pub namespace: &'s str,
    pub identifier: &'s str
}

impl <'s> TryFrom<&'s str> for ResourceName<'s> {
    type Error = InvalidNameError;
    
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        match value.split_once(':') {
            Some((namespace,identifier)) => Ok(Self{ namespace, identifier }),
            None => Err(InvalidNameError {}),
        }
    }
}

impl Copy for ResourceName<'_> {}

impl Display for ResourceName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}:{}",self.namespace,self.identifier)
    }
}