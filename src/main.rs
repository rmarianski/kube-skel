use anyhow::Context as _;
use clap::Parser;
use serde::Serialize;
use std::str::FromStr;
use tinytemplate::TinyTemplate;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long = "type")]
    typ: ResourceType,

    #[clap(long)]
    name: String,

    #[clap(long)]
    storage: Option<String>,

    #[clap(long)]
    port: Option<u16>,

    #[clap(long)]
    image: Option<String>,
}

#[derive(Debug)]
enum ResourceType {
    Deploy,
    StatefulSet,
    PersistentVolumeClaim,
    Service,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid type {0}")]
struct InvalidResourceType(String);

impl FromStr for ResourceType {
    type Err = InvalidResourceType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deploy" | "deployment" | "d" => Ok(ResourceType::Deploy),
            "statefulset" | "stateful-set" | "ss" => Ok(ResourceType::StatefulSet),
            "persistent-volume-claim" | "pvc" => Ok(ResourceType::PersistentVolumeClaim),
            "service" | "svc" => Ok(ResourceType::Service),
            _ => Err(InvalidResourceType(s.into())),
        }
    }
}

#[derive(Debug, Serialize)]
struct Context {
    name: String,
    storage: Option<String>,
    port: Option<u16>,
    image: Option<String>,
}

impl From<Args> for Context {
    fn from(args: Args) -> Self {
        Context {
            name: args.name,
            storage: args.storage,
            port: args.port,
            image: args.image,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let template = match args.typ {
        ResourceType::StatefulSet => include_str!("../var/stateful-set.tpl"),
        ResourceType::Deploy => include_str!("../var/deploy.tpl"),
        ResourceType::PersistentVolumeClaim => include_str!("../var/persistent-volume-claim.tpl"),
        ResourceType::Service => include_str!("../var/service.tpl"),
    };
    let mut tt = TinyTemplate::new();
    tt.add_template("template", template)
        .context("add template")?;
    let context = Context::from(args);
    let output = tt.render("template", &context).context("render")?;
    println!("{}", output);
    Ok(())
}
