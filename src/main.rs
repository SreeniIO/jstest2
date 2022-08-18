#![deny(warnings)]

mod utils;

use backtrace::Backtrace;
use hirofa_utils::js_utils::{adapters::JsRealmAdapter, facades::JsRuntimeFacade, Script};
use log::{error, LevelFilter};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::panic;

use crate::utils::{get_as_string, make_rt};

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    pub rows: Vec<Box<Map<String, Value>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::new();
        println!(
            "thread panic occurred: {}\nbacktrace: {:?}",
            panic_info, backtrace
        );
        log::error!(
            "thread panic occurred: {}\nbacktrace: {:?}",
            panic_info,
            backtrace
        );
    }));

    simple_logging::log_to_stderr(LevelFilter::Info);

    let rt = make_rt();
    let rt2 = rt.clone();

    let id = "id-1";
    let id2 = "id-1";

    match rt.create_context(id) {
        Ok(_) => {}
        Err(e) => error!("Error calling create_context {}: {}", id, e),
    };
    for i in 0..10 {
        println!("\nloop {}\n", i);
        let rt = rt.clone();
        match rt.js_loop_realm_sync(Some(id), move |_q_js_rt, q_ctx| {
            let res = q_ctx.eval(Script::new(
                "test.js",
                r#"JSON.stringify({rows: [{"appUid":"cloudio","comment":"👍","commentedBy":"admin","contextId":"$DD$","contextValue":"cloudio.home","createdBy":"admin","creationDate":"2022-08-18T06:17:06.871617Z","lastUpdateDate":"2022-08-18T06:17:06.871617Z","lastUpdatedBy":"admin","md":{},"orgUid":"cloudio","uid":"01GAQSJAHQ7WE54JNRC1J9HVH3","_rs":"Q"}]})"#,
            ));

            match res {
                Ok(js) => q_ctx.to_js_value_facade(&js),
                Err(e) => Err(e),
            }
        }) {
            Ok(r) => {
                let fut = get_as_string(rt, r, "return value".to_owned(), id.to_string()).await;
                match fut {
                    Ok(val) => println!("result={}", val),
                    Err(e) => eprintln!("err: {}", e),
                };
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        };
    }
    // drop the above created context
    rt2.drop_context(id2);

    println!("done");

    Ok(())
}
