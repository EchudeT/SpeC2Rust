### env

Python 3.12.12
rustc 1.95.0 (59807616e 2026-04-14)

### steps

0. install requirements and api
```
pip install -r requirements.txt
```

You need to change the `api_key` and `model_name` in `./local_config.example.json` to your own api key and model name, then rename it to `local_config.json`.

1. start translation
```
./scripts/agent.sh [repo_name]
```

eg:
```
./scripts/agent.sh c4
```

output dir is ./output/[repo_name]
note: some repos you may need to rebuild, because these repos have hard-coded path.

If you want to test a new C project, place the C project under the `datasets/` directory, and put the test scripts (shell scripts) into the `test/` directory under the C project's root. Make sure the C project can be compiled into an executable binary, because our testing logic compares the behavior of the compiled C and Rust binaries. Both the C and Rust binaries must be able to execute the test shell script.

### code map

The repository contains some legacy and experimental files. For artifact review, the main SpeC2Rust workflow starts from `scripts/agent.sh`, which invokes `src/agent/main.py` with the default agents used in the paper.

Key files for the paper workflow:

- `scripts/agent.sh`: command-line entry point used to run the full translation workflow on a project under `datasets/`.
- `src/agent/main.py`: top-level orchestrator that connects specification generation, Rust generation, compilation repair, and test-driven repair.
- `src/parse/c_ast.py`: tree-sitter based C parser and static-analysis component. It extracts function records, source spans, call sites, data definitions, and project metadata.
- `src/agent/split.py`: hierarchical module splitting logic used to form module and cluster units from static-analysis evidence.
- `src/agent/spec_agent.py`: Stage 1 specification generation. It builds project-level rewrite context and module-level specification bundles.
- `src/agent/alternatives/contextual_rust_agent.py`: Stage 2 spec-guided Rust generation. It assembles file-specific context and maintains generated-interface evidence during translation.
- `src/agent/rust_structural_repair/`: deterministic structural normalization before LLM-based repair.
- `src/agent/error_organizer_agent.py`: compiler-diagnostic organization and batching for repair.
- `src/agent/rust_repair_agent.py`: compilation-guided repair loop for generated Rust projects.
- `src/agent/rtest/`: test-driven repair closure. The most relevant files are `rust_test_agent.py`, `test_runner.py`, `repair_prompt.py`, and `c_project_builder.py`.
- `src/llm/model.py`, `src/llm/custom_api.py`, and `src/config/config.py`: LLM backend and configuration loading.
- `src/config/prompt.py`: prompt templates used by the agents.

Useful evaluation scripts:

- `scripts/get_unsafe_rate.py`: reports unsafe-code statistics for a generated Rust project.
- `scripts/clippy_check.sh`: runs `cargo clippy` on generated Rust projects.
- `scripts/count_raw_ptrs.py`: counts raw-pointer declarations and dereferences.

Generated artifacts are written to `output/[repo_name]/`. In a completed run, `output/[repo_name]/c_docs/` contains the generated specification documents, and `output/[repo_name]/[repo_name]-rust/` contains the translated Rust project.

Files such as `*.bak`, most files under `src/agent/alternatives/` other than `contextual_rust_agent.py`, `src/parse/test_c_project/`, and cached examples under `src/parse/res/` are retained for development history or auxiliary experiments and are not required to inspect the main paper workflow.

2. check unsafe rate/clippy

You can use 'python ./scripts/get_unsafe_rate.py [repo_name]' and './scripts/clippy_check.sh [repo_name]' to get unsafe rate and clippy rate.
