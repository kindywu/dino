use std::{collections::HashMap, env, fs, path::Path};

use super::{build_project, CmdExector};
use anyhow::Result;
use clap::Parser;
use rquickjs::{Context, FromJs, Function, IntoJs, Object, Promise, Runtime};
use typed_builder::TypedBuilder;

#[derive(Debug, Parser)]
pub struct RunOpts {}

impl CmdExector for RunOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        run_project(&path)?;
        Ok(())
    }
}

fn run_project(path: &Path) -> Result<()> {
    let file = build_project(&path)?;
    let module = fs::read_to_string(file)?;
    let worker = JsWorker::try_new(&module)?;

    // TODO: normally this should run axum and let it load the worker
    let req = Req::builder()
        .method("GET")
        .url("https://example.com")
        .headers(HashMap::new())
        .build();
    let ret = worker.run_http("hello", req)?;
    println!("Response: {:?}", ret);

    Ok(())
}

struct JsWorker {
    ctx: Context,
}

impl JsWorker {
    pub fn try_new(module: &str) -> Result<Self> {
        let rt = Runtime::new()?;
        let ctx = Context::full(&rt)?;

        ctx.with(|ctx| {
            let global = ctx.globals();
            let ret: Object = ctx.eval(module)?;
            global.set("handlers", ret)?;
            // setup print function
            let fun = Function::new(ctx.clone(), print)?.with_name("print")?;
            global.set("print", fun)?;

            Ok::<_, anyhow::Error>(())
        })?;

        Ok(Self { ctx })
    }

    #[allow(unused)]
    pub fn run(&self, code: &str) -> anyhow::Result<()> {
        self.ctx.with(|ctx| {
            ctx.eval_promise(code)?.finish()?;
            Ok::<_, anyhow::Error>(())
        })?;

        Ok(())
    }

    pub fn run_http(&self, name: &str, req: Req) -> anyhow::Result<Res> {
        Ok(self.ctx.with(|ctx| {
            let global = ctx.globals();
            let handlers: Object = global.get("handlers")?;
            let fun: Function = handlers.get(name)?;
            let v: Promise = fun.call((req,))?;
            let res = v.finish()?;

            Ok::<_, anyhow::Error>(res)
        })?)
    }
}

#[derive(Debug, TypedBuilder)]
pub struct Req {
    pub headers: HashMap<String, String>,
    #[builder(setter(into))]
    pub method: String,
    #[builder(setter(into))]
    pub url: String,
    #[builder(default, setter(strip_option))]
    pub body: Option<String>,
}

impl<'js> IntoJs<'js> for Req {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        let obj = Object::new(ctx.clone())?;

        obj.set("header", self.headers)?;
        obj.set("method", self.method)?;
        obj.set("url", self.url)?;
        obj.set("body", self.body)?;

        Ok(obj.into())
    }
}

#[allow(unused)]
#[derive(Debug, TypedBuilder)]
pub struct Res {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl<'js> FromJs<'js> for Res {
    fn from_js(_ctx: &rquickjs::Ctx<'js>, value: rquickjs::Value<'js>) -> rquickjs::Result<Self> {
        let obj = value.into_object().unwrap();

        let status = obj.get("status")?;
        let headers = obj.get("headers")?;
        let body = obj.get("body")?;

        Ok(Res {
            status,
            headers,
            body,
        })
    }
}

fn print(msg: String) {
    println!("{msg}");
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, env};

    use anyhow::Result;

    use crate::cli::run_opts::run_project;

    use super::*;

    #[tokio::test]
    async fn run_project_should_work() -> Result<()> {
        let demo_path = env::current_dir()?.join("demo");
        run_project(&demo_path)?;
        Ok(())
    }

    #[test]
    fn js_worker_should_run() {
        let code = r#"
    (function(){async function hello(){print("hello world");return"hello";}return{hello:hello};})();
    "#;
        let worker = JsWorker::try_new(code).unwrap();
        worker.run("await handlers.hello()").unwrap();
    }

    #[test]
    fn js_worker_should_run_http() {
        let code = r#"
            (function(){
                async function hello(req){
                    return {
                        status:200,
                        headers:{
                            "content-type":"application/json"
                        },
                        body: JSON.stringify(req),
                    };
                }
                return{hello:hello};
            })();
        "#;
        let req = Req::builder()
            .method("GET")
            .url("https://example.com")
            .headers(HashMap::new())
            .build();
        let worker = JsWorker::try_new(code).unwrap();
        worker.run_http("hello", req).unwrap();
    }
}
