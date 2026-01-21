# AI Assistant Guide for Rucksack Development

**Version:** 2.0
**Last Updated:** 2025-12-31
**Purpose:** Comprehensive guidelines for AI assistants working with the Rucksack Rust project

## About This Document

This document provides essential guidance for AI assistants (like Claude Code) when working with the Rucksack codebase. It focuses on **Rucksack-specific** conventions, patterns, and workflows, while deferring to authoritative Rust guidelines for general best practices.

### Document Hierarchy

**For Rust Code Quality:**

1. **`assets/ai/ai-rust/skills/claude/SKILL.md`** - Advanced Rust programming skill (**use this**)
2. **`assets/ai/ai-rust/guides/*.md`** - Comprehensive Rust guidelines referenced by the skill
3. **`assets/ai/CLAUDE-CODE-COVERAGE.md`** - Comprehensive test coverage guide
4. **This file (CLAUDE.md)** - Rucksack-specific conventions only

**Important:** If `assets/ai/ai-rust` does not exist on the file system, ask permission to clone it:

```bash
git clone https://github.com/oxur/ai-rust assets/ai/ai-rust
```

---

## Quick Reference Checklists

### Before Starting Work

- [ ] Read relevant design docs (`./bin/odm show <number>`)
- [ ] Load Rust anti-patterns guide (`11-anti-patterns.md`)
- [ ] Load relevant Rust topic guides
- [ ] Understand existing code patterns (read related files)
- [ ] Check test coverage of related code
- [ ] Understand the "why" behind the task

### Before Submitting Changes

- [ ] All tests pass (`make test`)
- [ ] Coverage ≥ 95% (`make coverage`)
- [ ] Linting passes (`make lint`)
- [ ] Code formatted (`make format`)
- [ ] Run the CLI integration tests (`./tests/rucksack.sh`)
- [ ] Run the DB migration tests (`./tests/old_db_formats.sh`)
- [ ] No compiler warnings
- [ ] Checked against Rust anti-patterns (`11-anti-patterns.md`)
- [ ] Documentation updated (doc comments on public items)
- [ ] Design docs updated (if architectural changes)
- [ ] Commit message is clear and references design docs if relevant

### When Writing Rust Code

- [ ] Loaded `11-anti-patterns.md` first
- [ ] Loaded `01-core-idioms.md` for standard patterns
- [ ] Loaded topic-specific guides as needed
- [ ] Followed established patterns in the crate
- [ ] Added Position tracking to errors (if parse/build code)
- [ ] Used project error handling patterns
- [ ] Checked against AP-01 through AP-20
- [ ] Self-reviewed before submitting

### When Testing

- [ ] Followed test naming convention: `test_<fn>_<scenario>_<expectation>`
- [ ] Used project test data helpers (`parse_example`, etc.)
- [ ] Tested happy path
- [ ] Tested all error paths
- [ ] Tested edge cases (empty, boundary values)
- [ ] Added round-trip tests (if conversion code)
- [ ] Verified coverage ≥ 95%
- [ ] See `CLAUDE-CODE-COVERAGE.md` for comprehensive approach

### When Refactoring

- [ ] Ensured tests exist before starting
- [ ] Loaded `11-anti-patterns.md`
- [ ] Identified violations with pattern IDs
- [ ] Made incremental changes
- [ ] Ran tests after each change
- [ ] Preserved existing behavior
- [ ] Updated design docs if needed
- [ ] Referenced pattern IDs in commits

### When Reviewing Code

- [ ] Loaded `11-anti-patterns.md`
- [ ] Checked each pattern (AP-01 to AP-20)
- [ ] Loaded topic guides for code content
- [ ] Verified Rucksack conventions
- [ ] Checked test coverage (≥95%)
- [ ] Verified design doc alignment
- [ ] Checked error handling
- [ ] Used pattern IDs in feedback

---

## Summary

**This document provides Rucksack-specific guidance. For Rust best practices:**

📖 **Use the Rust Guidelines Skill:** `assets/ai/ai-rust/skills/claude/SKILL.md`

**Key takeaways:**

1. **Rust code quality** → Use the skill and guides
2. **Rucksack conventions** → Use this document
3. **Testing** → Use CLAUDE-CODE-COVERAGE.md + Rust guides
4. **Architecture** → Check design docs
5. **Always** load anti-patterns guide first

**Document End**

**Last Updated:** 2025-12-31
**Version:** 2.0
