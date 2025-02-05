use std::sync::Arc;

use crate::{extract_flags, DatafusionDBMS, TestFlags};
use anyhow::Result;
use datafusion::{execution::TaskContext, physical_plan::ExecutionPlan};

use super::PlannerBenchRunner;

/// A benchmark runner for evaluating execution time of optimized plan.
pub struct ExecutionBenchRunner {
    pub dbms: DatafusionDBMS,
    /// DDLs and DMLs to populate the tables.
    pub populate_sql: String,
}

impl ExecutionBenchRunner {
    pub async fn new(populate_sql: String) -> Result<Self> {
        Ok(ExecutionBenchRunner {
            dbms: DatafusionDBMS::new().await?,
            populate_sql,
        })
    }
}

/// With physical execution plan as input,
/// measures the time it takes to execute the plan generated by the optimizer.
impl PlannerBenchRunner for ExecutionBenchRunner {
    const BENCH_NAME: &str = "execution";
    type BenchInput = Vec<(Arc<dyn ExecutionPlan>, Arc<TaskContext>)>;
    async fn setup(
        &mut self,
        test_case: &sqlplannertest::ParsedTestCase,
    ) -> Result<(Self::BenchInput, TestFlags)> {
        for sql in &test_case.before_sql {
            self.dbms.execute(sql, &TestFlags::default()).await?;
        }

        // Populate the existing tables.
        for sql in self.populate_sql.split(";\n") {
            self.dbms.execute(sql, &TestFlags::default()).await?;
        }

        let bench_task = test_case
            .tasks
            .iter()
            .find(|x| x.starts_with("bench"))
            .unwrap();
        let flags = extract_flags(bench_task)?;

        self.dbms.setup(&flags).await?;
        let statements = self.dbms.parse_sql(&test_case.sql).await?;

        let mut physical_plans = Vec::new();
        for statement in statements {
            physical_plans.push(self.dbms.create_physical_plan(statement, &flags).await?);
        }

        Ok((physical_plans, flags))
    }
    async fn bench(
        self,
        input: Self::BenchInput,
        _test_case: &sqlplannertest::ParsedTestCase,
        _flags: &TestFlags,
    ) -> Result<()> {
        for (physical_plan, task_ctx) in input {
            self.dbms.execute_physical(physical_plan, task_ctx).await?;
        }
        Ok(())
    }
}
