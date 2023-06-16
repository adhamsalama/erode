use deno_core::op;
use deno_core::Extension;
use deno_core::{error::AnyError, FastString};
use edon::ts_transpiler::TsModuleLoader;
use reqwest;
use std::rc::Rc;

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[op]
async fn op_fetch(url: String) -> Result<String, AnyError> {
    let res = reqwest::get(url)
        .await?
        // .json::<HashMap<String, String>>()
        .text()
        .await?;
    Ok(res)
}

#[op]
async fn op_set_timeout(delay: u64) -> Result<(), AnyError> {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
    Ok(())
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    // get current dir
    let current_dir = std::env::current_dir()?;
    let main_module = deno_core::resolve_path(file_path, &current_dir)?;
    let runjs_extension = Extension::builder("runjs")
        .ops(vec![
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
            op_fetch::decl(),
            op_set_timeout::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });
    let runtime_js: String = include_str!("./runtime.js").to_string();
    let runtime_js: FastString = format!("{}", runtime_js).into();
    js_runtime
        .execute_script("[runjs:runtime.js]", runtime_js)
        .unwrap();
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        eprintln!("Usage: runjs <file>");
        std::process::exit(1);
    }
    let file_path = &args[1];

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js(&file_path)) {
        eprintln!("error: {}", error);
    }
}
