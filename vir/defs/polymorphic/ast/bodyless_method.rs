// © 2019, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::polymorphic::ast::*;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BodylessMethod {
    pub name: String,
    pub formal_args: Vec<LocalVar>,
    pub formal_returns: Vec<LocalVar>,
}

impl WithIdentifier for BodylessMethod {
    fn get_identifier(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for BodylessMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method {}(", self.name)?;
        let mut first = true;
        for arg in &self.formal_args {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", arg)?;
            first = false
        }
        write!(f, ") returns (")?;
        for arg in &self.formal_returns {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", arg)?;
            first = false
        }
        write!(f, ");")
    }
}
