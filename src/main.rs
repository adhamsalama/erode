use deno_core::Extension;
use deno_core::{error::AnyError, FastString};
use erode::node_api;
use erode::ts_transpiler::TsModuleLoader;
use erode::web_api;
use std::rc::Rc;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    // get current dir
    let current_dir = std::env::current_dir()?;
    let main_module = deno_core::resolve_path(file_path, &current_dir)?;
    let runjs_extension = Extension::builder("erode")
        .ops(vec![
            node_api::op_read_file::decl(),
            node_api::op_write_file::decl(),
            node_api::op_remove_file::decl(),
            web_api::op_fetch::decl(),
            web_api::op_set_timeout::decl(),
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
        .execute_script("[erode:runtime.js]", runtime_js)
        .unwrap();
    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        eprintln!("Usage: erode <file>");
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
