use std::env;
use std::process::Command;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  let sail_home = Command::new("sail")
    .arg("-dir")
    .output()
    .expect("failed to find Sail's home directory")
    .stdout;
  let sail_home = std::str::from_utf8(&sail_home).expect("Sail's output is not UTF-8?").trim_end(); 

  let sail_src = vec![
    "model/prelude.sail",
    "model/operators.sail",
    "model/immediates.sail",
    "model/types.sail",
    "model/registers/predicates.sail",
    "model/registers/registers.sail",
    "model/registers/system.sail",
    "model/state.sail",
    "model/instructions/ast.sail",
    "model/instructions/encoding_word.sail",
    "model/instructions/block.sail",
    "model/instructions/execute.sail",
    "model/instructions/assembly.sail",
    "model/fetch.sail",
    "model/main.sail"
  ];

  println!("cargo:rustc-link-lib=gmp");

  for file in &sail_src {
    println!("cargo:rerun-if-changed={}", file);
  }

  let sail = Command::new("sail")
    .arg("-c")
    .args(["-o", &format!("{}/foxmulator", out_dir), "-c_no_main"])
    .args(&sail_src)
    .output()
    .expect("Sail model failed to execute");

  if !sail.status.success() {
    let stdout = std::str::from_utf8(&sail.stdout).expect("Sail's output is not UTF-8?");
    let stderr = std::str::from_utf8(&sail.stderr).expect("Sail's output is not UTF-8?");

    panic!("Sail: {} {} {}", sail.status, stdout, stderr);
  }

  cc::Build::new()
    .include("include")
    .include(format!("{}/lib", sail_home))
    .include(env::var("DEP_GMP_INCLUDE_DIR").unwrap())
    .file(format!("{}/lib/sail.c", sail_home))
    .file(format!("{}/lib/sail_failure.c", sail_home))
    .file(&format!("{}/foxmulator.c", out_dir))
    .warnings(false) /* These a really spammy, but it is what it is … */
    .compile("foxmulator-model");

    println!("cargo:rustc-link-lib=foxmulator-model");

}
