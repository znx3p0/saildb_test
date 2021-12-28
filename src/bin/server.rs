use saildb::Sail;
use sia::Addr;
use sia::Result;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "saildb-test-bin")]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Addr::new))]
    bind: Addr,
}

#[sia::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    opt.bind.bind().await?;
    Sail::<String, String>::bind()?;
    println!("listening at {:?}", opt.bind);
    std::future::pending().await
}





