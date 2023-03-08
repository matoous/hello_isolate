use v8;

fn main() {
    let platform = v8::Platform::new(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Runtime provided by the server which usually follows the WHATWG and W3C
    // specs.
    let runtime = include_str!("runtime.js");

    // Code that would be provided by the user.
    let worker_script = r#"
export function handler(y) {
    return sayHello(y);
};
"#;

    // Actual script that will be executed - combination of runtime and user code.
    let script = format!(
        r#"
{runtime}
{worker_script}
"#
    );

    {
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        let global = setup_runtime(&mut isolate);

        let worker_scope = &mut v8::HandleScope::with_context(isolate.as_mut(), global.clone());

        let handler = build_worker(script.as_str(), worker_scope, &global);

        run_worker(handler, worker_scope, &global);
    }

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::dispose_platform();
}

pub fn say_hello_binding(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let to = args.get(0).to_rust_string_lossy(scope);
    let hello = v8::String::new(scope, format!("Hello, {}!", to).as_str())
        .unwrap()
        .into();
    retval.set(hello);
}

/// Prepares the Isolate.
/// Here we would register global runtime functions such as `console` or `fetch`.
fn setup_runtime(isolate: &mut v8::OwnedIsolate) -> v8::Global<v8::Context> {
    let isolate_scope = &mut v8::HandleScope::new(isolate);
    let globals = v8::ObjectTemplate::new(isolate_scope);
    let resource_name = v8::String::new(isolate_scope, "sayHello").unwrap().into();
    globals.set(
        resource_name,
        v8::FunctionTemplate::new(isolate_scope, say_hello_binding).into(),
    );
    let global_context = v8::Context::new_from_template(isolate_scope, globals);
    v8::Global::new(isolate_scope, global_context)
}

fn build_worker(
    script: &str,
    worker_scope: &mut v8::HandleScope,
    global: &v8::Global<v8::Context>,
) -> v8::Global<v8::Function> {
    let code = v8::String::new(worker_scope, script).unwrap();

    let resource_name = v8::String::new(worker_scope, "script.js").unwrap().into();
    let source_map_url = v8::String::new(worker_scope, "script.js").unwrap().into();
    let source = v8::script_compiler::Source::new(
        code,
        Some(&v8::ScriptOrigin::new(
            worker_scope,
            resource_name,
            0,
            0,
            false,
            i32::from(0),
            source_map_url,
            false,
            false,
            true,
        )),
    );

    let module = v8::script_compiler::compile_module(worker_scope, source).unwrap();
    module.instantiate_module(worker_scope, |_, _, _, _| None);
    module.evaluate(worker_scope);

    // actually getting the handler of the woker
    // could be peristed between calls
    let global = global.open(worker_scope);
    let global = global.global(worker_scope);
    let handler_key = v8::String::new(worker_scope, "workerHandler").unwrap();
    let js_handler = global.get(worker_scope, handler_key.into()).unwrap();
    let local_handler = v8::Local::<v8::Function>::try_from(js_handler).unwrap();
    v8::Global::new(worker_scope, local_handler)
}

pub fn run_worker(
    worker: v8::Global<v8::Function>,
    scope: &mut v8::HandleScope,
    global: &v8::Global<v8::Context>,
) {
    let handler = worker.open(scope);
    let global = global.open(scope);
    let global = global.global(scope);

    let param = v8::String::new(scope, "World").unwrap().into();
    match handler.call(scope, global.into(), &[param]) {
        Some(response) => {
            let result = v8::Local::<v8::String>::try_from(response)
                .expect("Handler did not return a string");
            let result = result.to_string(scope).unwrap();
            println!("{}", result.to_rust_string_lossy(scope));
        }
        None => todo!(),
    };
}
