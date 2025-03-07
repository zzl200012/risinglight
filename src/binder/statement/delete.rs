// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

use super::*;

/// A bound `delete` statement.
#[derive(Debug, PartialEq, Clone)]
pub struct BoundDelete {
    pub from_table: BoundTableRef,
    pub where_clause: Option<BoundExpr>,
}

impl Binder {
    pub fn bind_delete(&mut self, stmt: &Statement) -> Result<Box<BoundDelete>, BindError> {
        self.push_context();
        let ret = self.bind_delete_internal(stmt);
        self.pop_context();
        ret
    }

    pub fn bind_delete_internal(
        &mut self,
        stmt: &Statement,
    ) -> Result<Box<BoundDelete>, BindError> {
        if let Statement::Delete {
            table_name,
            selection,
        } = stmt
        {
            let table_name = &lower_case_name(table_name);
            let (database_name, schema_name, table_name) = split_name(table_name)?;
            let mut from_table =
                self.bind_table_ref_with_name(database_name, schema_name, table_name)?;
            let where_clause = match &selection {
                Some(expr) => Some(self.bind_expr(expr)?),
                None => None,
            };
            self.bind_column_ids(&mut from_table);
            Ok(Box::new(BoundDelete {
                from_table,
                where_clause,
            }))
        } else {
            panic!("unmatched statement type")
        }
    }
}
