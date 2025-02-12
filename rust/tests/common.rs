// Licensed to the Software Freedom Conservancy (SFC) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SFC licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use assert_cmd::Command;
use std::env::consts::OS;
use std::path::Path;

use is_executable::is_executable;
use selenium_manager::logger::JsonOutput;
use selenium_manager::shell;
use selenium_manager::shell::run_shell_command_by_os;

pub fn assert_driver(cmd: &mut Command) {
    let stdout = &cmd.unwrap().stdout;
    let output = std::str::from_utf8(stdout).unwrap();
    println!("{}", output);

    let json: JsonOutput = serde_json::from_str(output).unwrap();
    let driver_path = Path::new(&json.result.driver_path);
    assert!(driver_path.exists());
    assert!(is_executable(driver_path));
}

pub fn assert_browser(cmd: &mut Command) {
    let stdout = &cmd.unwrap().stdout;
    let output = std::str::from_utf8(stdout).unwrap();
    let json: JsonOutput = serde_json::from_str(output).unwrap();
    let browser_path = Path::new(&json.result.browser_path);
    assert!(browser_path.exists());
    assert!(is_executable(browser_path));
}

#[allow(dead_code)]
pub fn exec_driver(cmd: &mut Command) -> String {
    let stdout = &cmd.unwrap().stdout;
    let output = std::str::from_utf8(stdout).unwrap();
    let json: JsonOutput = serde_json::from_str(output).unwrap();
    let driver_path = Path::new(&json.result.driver_path);

    let driver_version_command =
        shell::Command::new_single(format!("{} --version", driver_path.to_str().unwrap()));
    let output = run_shell_command_by_os(OS, driver_version_command).unwrap();
    println!("**** EXEC DRIVER: {}", output);
    output
}

#[allow(dead_code)]
pub fn display_output(cmd: &mut Command) {
    let stdout = &cmd.unwrap().stdout;
    let output = std::str::from_utf8(stdout).unwrap();
    println!("{}", output);
}
