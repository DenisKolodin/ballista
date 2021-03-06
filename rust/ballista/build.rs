// Copyright 2020 Andy Grove
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate configure_me_codegen;

fn main() -> Result<(), String> {
    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    println!("cargo:rerun-if-changed=proto/ballista.proto");
    tonic_build::configure()
        .compile(&["proto/ballista.proto"], &["proto"])
        .map_err(|e| format!("protobuf compilation failed: {}", e))
        .unwrap();

    println!("cargo:rerun-if-changed=src/bin/executor_config_spec.toml");
    println!("cargo:rerun-if-changed=src/bin/scheduler_config_spec.toml");
    configure_me_codegen::build_script_auto()
        .map_err(|e| format!("configure_me code generation failed: {}", e))
}
