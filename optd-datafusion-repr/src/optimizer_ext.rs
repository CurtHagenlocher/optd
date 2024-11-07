use optd_core::{nodes::PlanNodeOrGroup, optimizer::Optimizer};

use crate::{
    plan_nodes::DfNodeType,
    properties::{
        column_ref::{ColumnRefPropertyBuilder, GroupColumnRefs},
        schema::{Schema, SchemaPropertyBuilder},
    },
};

pub trait OptimizerExt {
    fn get_schema_of(&self, root_rel: PlanNodeOrGroup<DfNodeType>) -> Schema;
    fn get_column_ref_of(&self, root_rel: PlanNodeOrGroup<DfNodeType>) -> GroupColumnRefs;
}

impl<O: Optimizer<DfNodeType>> OptimizerExt for O {
    fn get_schema_of(&self, root_rel: PlanNodeOrGroup<DfNodeType>) -> Schema {
        self.get_property::<SchemaPropertyBuilder>(root_rel, 0)
    }

    fn get_column_ref_of(&self, root_rel: PlanNodeOrGroup<DfNodeType>) -> GroupColumnRefs {
        self.get_property::<ColumnRefPropertyBuilder>(root_rel, 1)
    }
}
