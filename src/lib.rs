use neon::prelude::*;
mod fit_utils;

fn parse_fit(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path = cx.argument::<JsString>(0)?;

    let rust_string = file_path.value(&mut cx);

    // read the file and do the things
    match fit_utils::parse_one_file2(&rust_string) {
        Ok(serialized_result) => Ok(cx.string(serialized_result)),
        Err(_) => Ok(cx.string("")),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseFit", parse_fit)?;
    Ok(())
}
