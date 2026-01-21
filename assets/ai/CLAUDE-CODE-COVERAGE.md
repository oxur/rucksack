# Comprehensive Test Coverage Prompt for Claude Code

## Objective

Achieve **95%+ test coverage** through systematic, intelligent test development. Do not stop until this threshold is met.

## Core Principles

### 1. Coverage is Non-Negotiable

- Target: **95%+ line coverage** minimum
- **Never settle** for "good enough" at 70-80%
- Track progress explicitly: "Currently at X%, need Y% more"
- Continue iterating until threshold is met
- If you encounter obstacles, document them but **keep going**

### 2. Warnings Are There For A Reason

- Fixing warnings is a bug prevention measure
- Deprecation warnings will bite you with breaks eventually
- Warnings are a bad user experience and may cause them to lose confidence in the project
- Do NOT ignore warnings; when you see then, fix them
- You may temporarily set warning fix tasks to a lower priority, but you MUST return to them

### 3. Linting/Formatting Must Always Be Checked

- To assist with this, after each change, be sure to run `make format`
- Before running new tests, be make sure linting passes `make lint`

### 4. Tests Must Pass - No Exceptions

- **Zero broken tests** is the only acceptable state
- A broken test indicates one of three things:
  1. The test is incorrectly written (fix the test)
  2. The implementation has a bug (fix the implementation)
  3. The test revealed an incorrect assumption (investigate and fix root cause)
- **Never** accept broken tests as "okay" or "not critical"
- **Never** mark tests with `#[ignore]` to hide failures

### 5. Fix Root Causes, Not Symptoms

When a test fails, follow this process:

**Step 1: Understand**

- Read the error message completely
- Identify what the test expects vs. what actually happened
- Trace the execution path that led to the failure
- Ask: "What assumption is being violated?"

**Step 2: Diagnose**

- Is the test's expectation correct? (validate against spec/requirements)
- Is the implementation's behavior correct? (validate against intended design)
- Is there a mismatch in understanding?

**Step 3: Fix Intelligently**

- If test is wrong: Fix the test to match correct behavior
- If implementation is wrong: Fix the implementation bug
- If design assumption is wrong: Fix the design, then update both
- **Never** change implementation just to make test pass without understanding why

**Anti-patterns to Avoid:**

- ❌ Changing return types to match test without understanding why
- ❌ Adding special cases in code just for tests
- ❌ Commenting out failing assertions
- ❌ Making tests less strict to avoid failures
- ✅ Understanding the contract, then ensuring both code and tests honor it

### 6. Systematic Coverage Approach

Follow this order:

**Phase 1: Module-by-Module Coverage**

```
For each module in the project:
  1. Run coverage: `cargo llvm-cov --html`
  2. Open coverage report
  3. Identify uncovered lines in this module
  4. Write tests for uncovered code paths
  5. Verify tests pass
  6. Re-run coverage
  7. Repeat until module is 95%+ covered
```

**Phase 2: Integration Coverage**

```
After all modules are covered:
  1. Check for uncovered integration paths
  2. Write integration tests
  3. Verify all tests pass
  4. Re-run full coverage
```

**Phase 3: Edge Cases & Error Paths**

```
Systematically test:
  - Error conditions
  - Boundary values
  - Null/empty inputs
  - Concurrent access scenarios
  - Resource exhaustion
  - Invalid state transitions
```

### 7. Progress Tracking

After each testing session, report:

```
Coverage Progress Report:
========================
Current Coverage: X.X%
Target Coverage: 95.0%
Gap: Y.Y%

Modules Completed (95%+):
- module_a: 98.5%
- module_b: 96.2%

Modules In Progress (<95%):
- module_c: 87.3% (needs: error path tests, edge cases)
- module_d: 72.1% (needs: full test suite)

Next Steps:
1. [Specific action for module_c]
2. [Specific action for module_d]

Blockers: [None | Describe any technical blockers]
```

## Testing Strategy by Code Type

### Pure Functions

```rust
// Test:
// - Happy path with typical inputs
// - Boundary values (0, 1, max, min)
// - Invalid inputs (if applicable)
// - Empty/null inputs
// - Large inputs (stress test)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() { /* ... */ }

    #[test]
    fn test_boundary_zero() { /* ... */ }

    #[test]
    fn test_boundary_max() { /* ... */ }

    #[test]
    fn test_empty_input() { /* ... */ }

    #[test]
    fn test_invalid_input() { /* ... */ }
}
```

### Functions with Side Effects

```rust
// Test:
// - Expected side effects occur
// - Side effects are idempotent (if applicable)
// - Cleanup happens on error
// - State transitions are correct
// - Concurrent access is safe (if applicable)

#[test]
fn test_side_effect_occurs() { /* ... */ }

#[test]
fn test_idempotent() { /* ... */ }

#[test]
fn test_cleanup_on_error() { /* ... */ }
```

### Error Handling

```rust
// Test EVERY error condition
// - Each possible error variant
// - Error propagation
// - Error messages are helpful
// - Cleanup on error

#[test]
fn test_handles_file_not_found() { /* ... */ }

#[test]
fn test_handles_permission_denied() { /* ... */ }

#[test]
fn test_error_propagates_correctly() { /* ... */ }
```

### Async Code

```rust
// Test:
// - Happy path
// - Timeouts
// - Cancellation
// - Concurrent operations
// - Resource cleanup

#[tokio::test]
async fn test_async_happy_path() { /* ... */ }

#[tokio::test]
async fn test_timeout() { /* ... */ }

#[tokio::test]
async fn test_concurrent_access() { /* ... */ }
```

### State Machines

```rust
// Test:
// - All state transitions
// - Invalid transitions (should error)
// - State persistence
// - State recovery

#[test]
fn test_transition_a_to_b() { /* ... */ }

#[test]
fn test_invalid_transition_fails() { /* ... */ }

#[test]
fn test_state_persistence() { /* ... */ }
```

## Handling Common Obstacles

### "This code is hard to test"

**Response:** Make it testable, don't skip it.

- Refactor for testability (dependency injection, traits, etc.)
- Use test doubles/mocks for external dependencies
- Break large functions into smaller, testable units
- **Document** why refactoring was needed

### "This is just a wrapper/trivial"

**Response:** Test it anyway.

- Wrappers can have subtle bugs
- Tests document expected behavior
- Coverage tools count these lines too
- It takes 2 minutes to write a trivial test

### "Coverage tool shows 100% but we're at 80%"

**Response:** Investigate the discrepancy.

- Check if tests are actually running (they might be ignored)
- Verify cargo test includes all test files
- Check for conditional compilation (#[cfg])
- Look for code in examples/ or benches/ that's not tested

### "Tests are slow"

**Response:** Optimize, don't skip.

- Use `cargo test --lib` for fast unit tests
- Move slow tests to integration tests
- Use `#[ignore]` for slow tests, run separately
- Parallelize test execution
- Mock expensive operations

### "Can't reach this line"

**Response:** Understand why.

- Is it dead code? Remove it.
- Is it defensive programming? Test the defense.
- Is it unreachable error handling? Inject failure to test it.
- Document if truly unreachable, add comment explaining why

## Coverage Report Interpretation

When reading `cargo llvm-cov --html`:

**Green lines (covered):**

- ✅ Good, move on

**Red lines (uncovered):**

- 🔴 **MUST** be covered (unless legitimately unreachable)
- Write test to execute this line
- If multiple conditions, test all branches

**Yellow lines (partially covered):**

- ⚠️ Some branches tested, others not
- Identify which branches are uncovered
- Write tests for missing branches

**Example:**

```rust
// Yellow line - partially covered
if condition_a && condition_b {  // Only tested with both true
    do_something();
}

// Need tests for:
// - condition_a=true, condition_b=false
// - condition_a=false, condition_b=true
// - condition_a=false, condition_b=false
```

## Quality Gates

Before considering testing "complete":

- [ ] Overall coverage ≥ 95%
- [ ] All modules ≥ 90% coverage (no stragglers)
- [ ] All tests pass (0 failures, 0 ignored)
- [ ] All error paths tested
- [ ] All public APIs tested
- [ ] Integration tests exist
- [ ] Edge cases covered
- [ ] No TODO/FIXME in test code
- [ ] Tests are readable and maintainable
- [ ] Tests run in reasonable time (< 30s for unit tests)

## Iterative Process

```
WHILE coverage < 95%:
    1. Run: cargo llvm-cov --html
    2. Open: target/llvm-cov/html/index.html
    3. Find lowest-covered module
    4. Identify specific uncovered lines
    5. Write tests for those lines
    6. Run: cargo test
    7. Fix any failures by understanding root cause
    8. Verify tests pass
    9. Re-run coverage
    10. Report progress

    If progress stalls for 2 iterations:
        - Analyze why (blockers, complexity, etc.)
        - Refactor for testability if needed
        - Ask for help/clarification if truly stuck
        - Document the blocker
        - **But keep trying alternative approaches**
END WHILE
```

## Anti-Patterns - Never Do These

❌ **Don't:** Skip tests because coverage is "high enough"
✅ **Do:** Continue until 95%+ threshold is met

❌ **Don't:** Mark failing tests as `#[ignore]` to make CI pass
✅ **Do:** Fix the root cause of the failure

❌ **Don't:** Change code to make test pass without understanding why
✅ **Do:** Understand the failure, then fix correctly

❌ **Don't:** Write tests that don't actually test anything (just for coverage)
✅ **Do:** Write meaningful tests that validate behavior

❌ **Don't:** Test implementation details
✅ **Do:** Test behavior and contracts

❌ **Don't:** Give up at 80% coverage
✅ **Do:** Push to 95%+ systematically

❌ **Don't:** Accept "close enough" on test assertions
✅ **Do:** Make assertions precise and correct

## Sample Test Development Session

```
Step 1: Initial Coverage Check
$ cargo llvm-cov --summary-only
Coverage: 67.3%

Step 2: Identify Gaps
$ cargo llvm-cov --html
Opening target/llvm-cov/html/index.html...

Findings:
- src/parser.rs: 45% (error paths not tested)
- src/validator.rs: 72% (edge cases missing)
- src/formatter.rs: 89% (close, needs a few tests)

Step 3: Start with Lowest
Working on: src/parser.rs (45%)

Uncovered lines:
- Line 42-45: Error path when input is empty
- Line 78-82: Error path when syntax invalid
- Line 95-98: Handling of escaped characters

Step 4: Write Tests
Writing test_parse_empty_input()...
Writing test_parse_invalid_syntax()...
Writing test_parse_escaped_chars()...

Step 5: Run Tests
$ cargo test
test parser::tests::test_parse_empty_input ... FAILED

Step 6: Debug Failure
Error: expected Err(EmptyInput), got Err(InvalidSyntax)

Analysis: Empty input falls through to syntax checker first.
Root cause: Validation order is wrong in implementation.

Step 7: Fix Root Cause
Moving empty check before syntax check in parser.rs:38...

Step 8: Verify Fix
$ cargo test
test parser::tests::test_parse_empty_input ... ok
All tests passed!

Step 9: Check Progress
$ cargo llvm-cov --summary-only
Coverage: 71.2% (was 67.3%, gained 3.9%)

Step 10: Continue
src/parser.rs now at 78%
Still need: unicode edge cases, boundary conditions
Continuing...
```

## Final Reminder

**Your job is not done until:**

1. Coverage is ≥ 95%
2. All tests pass
3. All code paths are tested
4. You understand why every test exists
5. You understand why every test passes

**Persistence beats perfection.** Keep iterating, keep testing, keep improving.

## Success Criteria Checklist

At the end of testing work, verify:

```
Testing Completion Checklist:
=============================
[ ] Coverage ≥ 95% overall
[ ] No module below 90% coverage
[ ] Zero failing tests
[ ] Zero ignored tests (except explicitly slow/integration tests)
[ ] All error paths tested
[ ] All edge cases tested
[ ] All public APIs have tests
[ ] Integration tests exist and pass
[ ] Performance tests exist (if applicable)
[ ] Tests are documented and maintainable
[ ] CI/CD pipeline includes coverage checks
[ ] Coverage report reviewed and understood
[ ] All uncovered lines justified (with code comments if unreachable)

Total Test Count: ___
Total Assertions: ___
Test Execution Time: ___ seconds
Coverage: ___%
```

**Sign off only when ALL boxes are checked.**
