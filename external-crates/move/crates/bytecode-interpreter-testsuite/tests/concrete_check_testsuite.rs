// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{env, path::Path};

use move_command_line_common::{env::read_bool_env_var, testing::EXP_EXT};
use move_prover_test_utils::baseline_test::verify_or_update_baseline;
use move_stdlib::move_stdlib_files;
use move_unit_test::{UnitTestingConfig, DEFAULT_EXECUTION_BOUND_SOLANA};

fn test_runner(path: &Path) -> datatest_stable::Result<()> {
    env::set_var("NO_COLOR", "1");

    let source_files = vec![path.to_str().unwrap().to_owned()];
    let config = UnitTestingConfig {
        gas_limit: Some(5000),
        num_threads: 1,
        source_files,
        dep_files: move_stdlib_files(),
        check_stackless_vm: true,
        verbose: read_bool_env_var("VERBOSE"),
        named_address_values: move_stdlib::move_stdlib_named_addresses()
            .into_iter()
            .collect(),

        ..UnitTestingConfig::default_with_bound(None)
    };

    let test_plan = config.build_test_plan().unwrap();
    let mut buffer = vec![];
    config.run_and_report_unit_tests(test_plan, None, None, &mut buffer, DEFAULT_EXECUTION_BOUND_SOLANA)?;
    let output = String::from_utf8(buffer)?;

    let baseline_path = path.with_extension(EXP_EXT);
    verify_or_update_baseline(&baseline_path, &output)?;
    Ok(())
}

datatest_stable::harness!(test_runner, "tests/concrete_check", r".*\.move$");
