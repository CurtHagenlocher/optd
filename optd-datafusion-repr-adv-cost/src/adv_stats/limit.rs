use optd_datafusion_repr::plan_nodes::{ArcDfPredNode, ConstantPred, DfReprPredNode};
use serde::{de::DeserializeOwned, Serialize};

use crate::adv_stats::stats::{Distribution, MostCommonValues};

use super::AdvStats;

impl<
        M: MostCommonValues + Serialize + DeserializeOwned,
        D: Distribution + Serialize + DeserializeOwned,
    > AdvStats<M, D>
{
    pub(crate) fn get_limit_row_cnt(&self, child_row_cnt: f64, fetch_expr: ArcDfPredNode) -> f64 {
        let fetch = ConstantPred::from_pred_node(fetch_expr)
            .unwrap()
            .value()
            .as_u64();
        // u64::MAX represents None
        if fetch == u64::MAX {
            child_row_cnt
        } else {
            child_row_cnt.min(fetch as f64)
        }
    }
}
