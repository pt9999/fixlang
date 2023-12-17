from math import ceil
import subprocess
import re
import itertools
import sys
import random
from statsmodels.stats.weightstats import ttest_ind
import numpy as np

# Optimizes llvm optimization passes by updating llvm_passes.rs.
SOURCE_FILE = './src/llvm_passes.rs'

INITIAL_PASSES = '''
add_scalar_repl_aggregates_pass
add_function_inlining_pass
add_cfg_simplification_pass
add_early_cse_pass
add_loop_deletion_pass
add_scalar_repl_aggregates_pass_ssa
add_global_dce_pass
add_scoped_no_alias_aa_pass
add_merge_functions_pass
add_global_optimizer_pass
add_aggressive_dce_pass
add_merge_functions_pass
add_global_dce_pass
add_loop_deletion_pass
add_strip_dead_prototypes_pass
add_constant_merge_pass
add_dead_store_elimination_pass
add_sccp_pass
add_loop_unroll_and_jam_pass
add_jump_threading_pass
add_strip_symbol_pass
add_tail_call_elimination_pass
add_partially_inline_lib_calls_pass
add_ipsccp_pass
add_constant_merge_pass
add_dead_store_elimination_pass
add_cfg_simplification_pass
add_bit_tracking_dce_pass
add_constant_merge_pass
add_scalar_repl_aggregates_pass
add_function_inlining_pass
add_cfg_simplification_pass
add_early_cse_pass
add_loop_deletion_pass
add_scalar_repl_aggregates_pass_ssa
add_global_dce_pass
add_scoped_no_alias_aa_pass
add_merge_functions_pass
add_global_optimizer_pass
add_aggressive_dce_pass
add_merge_functions_pass
add_global_dce_pass
add_loop_deletion_pass
add_strip_dead_prototypes_pass
add_constant_merge_pass
add_dead_store_elimination_pass
add_sccp_pass
add_loop_unroll_and_jam_pass
add_jump_threading_pass
add_strip_symbol_pass
add_tail_call_elimination_pass
add_partially_inline_lib_calls_pass
add_ipsccp_pass
add_constant_merge_pass
add_dead_store_elimination_pass
add_cfg_simplification_pass
add_bit_tracking_dce_pass
add_constant_merge_pass
add_memcpy_optimize_pass
add_instruction_combining_pass
'''
INITIAL_PASSES = INITIAL_PASSES.split('\n')
INITIAL_PASSES = [line.strip() for line in INITIAL_PASSES]
INITIAL_PASSES = [line for line in INITIAL_PASSES if len(line) > 0]

HEADER = '''
// This source file is generated by by passes_optimizer.py.

use super::*;
use inkwell::passes::PassManagerSubType;

pub fn add_passes<T: PassManagerSubType>(passmgr: &PassManager<T>) {
'''

FOOTER = '''
}
'''

ADD_PASS_FORMAT = 'passmgr.{}();'

# Benchmark program should write running time to stdout.
FIX_SOURCE_FILE = './benchmark/prime_loop'  # without extension

RUN_BENCH_ITERATION = 10

ADDED_PASSES_NUM = 10

SIGNIFICANCE_LEVEL = 0.05

ALLOWED_DEGRATION_ON_MINIMIZE = 0.99

# All passes
# Exclude:
#  add_scalar_repl_aggregates_pass_with_threshold (because requires parameter),
#  add_internalize_pass (because requires parameter),
#  add_gvn_pass (segfaults),
#  add_new_gvn_pass (breaks program)
#  add_licm_pass (breaks program)
PASSES = '''
add_instruction_combining_pass
add_memcpy_optimize_pass
add_aggressive_dce_pass
add_aggressive_inst_combiner_pass
add_alignment_from_assumptions_pass
add_always_inliner_pass
add_basic_alias_analysis_pass
add_bit_tracking_dce_pass
add_cfg_simplification_pass
add_constant_merge_pass
add_correlated_value_propagation_pass
add_dead_arg_elimination_pass
add_dead_store_elimination_pass
add_demote_memory_to_register_pass
add_early_cse_mem_ssa_pass
add_early_cse_pass
add_function_attrs_pass
add_function_inlining_pass
add_global_dce_pass
add_global_optimizer_pass
add_ind_var_simplify_pass
add_instruction_simplify_pass
add_ipsccp_pass
add_jump_threading_pass
add_loop_deletion_pass
add_loop_idiom_pass
add_loop_reroll_pass
add_loop_rotate_pass
add_loop_unroll_and_jam_pass
add_loop_unroll_pass
add_loop_vectorize_pass
add_lower_expect_intrinsic_pass
add_lower_switch_pass
add_merge_functions_pass
add_merged_load_store_motion_pass
add_partially_inline_lib_calls_pass
add_promote_memory_to_register_pass
add_prune_eh_pass
add_reassociate_pass
add_scalar_repl_aggregates_pass
add_scalar_repl_aggregates_pass_ssa
add_scalarizer_pass
add_sccp_pass
add_scoped_no_alias_aa_pass
add_simplify_lib_calls_pass
add_slp_vectorize_pass
add_strip_dead_prototypes_pass
add_strip_symbol_pass
add_tail_call_elimination_pass
add_type_based_alias_analysis_pass
'''


def get_all_passes():
    passes = []
    for p in PASSES.split('\n'):
        if len(p.strip()) > 0:
            passes.append(p)
    return passes


def write_source_file(passes):
    with open(SOURCE_FILE, 'w') as f:
        f.write(HEADER)
        f.write('\n')
        for p in passes:
            f.write(ADD_PASS_FORMAT.format(p))
            f.write('\n')
        f.write(FOOTER)
        f.write('\n')


def run_benchmark(run_bench_iteration=RUN_BENCH_ITERATION, timeout=60):
    cp = subprocess.run(['cargo', 'run', '--', 'build', '-f',
                        FIX_SOURCE_FILE + '.fix'], capture_output=True, text=True)
    if cp.returncode != 0:
        print('build failed.')
        print('stdout:')
        print(cp.stdout)
        print('stderr:')
        print(cp.stderr)
        sys.exit(1)

    times = []
    for _ in range(run_bench_iteration):
        try:
            cp = subprocess.run(['time', '-f', '%e', './a.out'],
                                capture_output=True, text=True, timeout=timeout)
            if cp.returncode != 0:
                print('run failed.')
                print('stdout:')
                print(cp.stdout)
                print('stderr:')
                print(cp.stderr)
                times = [1.0 * timeout]
                break
            else:
                times.append(float(cp.stdout))
        except subprocess.TimeoutExpired:
            times = [1.0 * timeout]
            break

    print('times: {}'.format(times))
    return np.array(times)


def print_passes(passes):
    print('  ' + ', '.join(passes), flush=True)


def optimize():
    all_passes = get_all_passes()
    optimum_passes = INITIAL_PASSES.copy()

    print('Initial passes:')
    print_passes(optimum_passes)

    write_source_file(optimum_passes)
    optimum_time = run_benchmark()
    print('Time with initial passes: {}'.format(
        sum(optimum_time) / len(optimum_time)))

    while True:
        added_passes_count = random.randint(1, ADDED_PASSES_NUM)
        added_passes = []
        for _ in range(added_passes_count):
            idx = random.randint(0, len(all_passes)-1)
            added_passes.append(all_passes[idx])
        print('Try adding passes:')
        print_passes(added_passes)
        passes = optimum_passes.copy()
        for p in added_passes:
            passes.append(p)
        write_source_file(passes)
        timeout = int(ceil(np.average(optimum_time) * 2.0))
        time = run_benchmark(timeout=timeout)
        (t_val, p_val, free) = ttest_ind(
            time, optimum_time, alternative="smaller", usevar="unequal")
        if p_val <= SIGNIFICANCE_LEVEL:
            optimum_passes = passes
            optimum_time = time
            print('New optimum passes found! (means=({}, {}), t={}, p={})'.format(
                np.average(time), np.average(optimum_time), t_val, p_val))
        else:
            print('No improvement found. (means=({}, {}), t={}, p={})'.format(
                np.average(time), np.average(optimum_time), t_val, p_val))

        # minimize passes
        passes = []
        removed_passes = []
        for p in optimum_passes:
            if random.randint(0, int(ceil(len(optimum_passes) / ADDED_PASSES_NUM))) != 0:
                passes.append(p)
            else:
                removed_passes.append(p)
        print('Try removing passes:')
        print_passes(removed_passes)
        write_source_file(passes)
        timeout = int(ceil(np.average(optimum_time) * 2.0))
        time = run_benchmark(timeout=timeout)
        (t_val, p_val, free) = ttest_ind(time * ALLOWED_DEGRATION_ON_MINIMIZE,
                                         optimum_time, alternative="smaller", usevar="unequal")
        if p_val <= SIGNIFICANCE_LEVEL:
            optimum_passes = passes
            optimum_time = time
            print("Minimize success! (means=({}, {}), t={}, p={})".format(
                np.average(time), np.average(optimum_time), t_val, p_val))
        else:
            print("Minimize failed (means=({}, {}), t={}, p={})".format(
                np.average(time), np.average(optimum_time), t_val, p_val))

        print('Current optimum passes:')
        print_passes(optimum_passes)
        write_source_file(optimum_passes)
        timeout = int(ceil(np.average(optimum_time) * 2.0))
        optimum_time = np.concatenate(
            [optimum_time, run_benchmark(timeout=timeout)])
        print('Current optimum time: {} ({} samples)'.format(
            np.average(optimum_time), optimum_time.size))


if __name__ == '__main__':
    optimize()
