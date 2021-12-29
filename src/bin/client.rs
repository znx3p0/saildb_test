use saildb::{Sail, DistributedList};
use canary::{Addr, Result};
use structopt::{StructOpt, clap::arg_enum};

#[derive(StructOpt, Debug)]
#[structopt(name = "saildb-test")]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Addr::new))]
    addr: Addr,
    #[structopt(subcommand)]
    action: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Insert {
        #[structopt(short, long)]
        key: String,
        #[structopt(short, long)]
        value: String,
    },
    Remove {
        #[structopt(short, long)]
        key: String,
    },
    Get {
        #[structopt(short, long)]
        key: String,
    }
}

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    enum Action {
        Insert,
        Remove,
        Get,
    }
}

#[canary::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    use srpc::IntoClient;

    let mut c = opt.addr.clone().service("id").connect().await?.client::<DistributedList<i32>>();
    c.push(&2).await?;
    c.push(2).await?;

    let mut db: Sail<String, String> = Sail::new(opt.addr).await?;
    match opt.action {
        Command::Insert { key, value } => {
            db.insert(&key, &value).await?;
        },
        Command::Remove { key } => {
            db.remove(&key).await?;
        },
        Command::Get { key } => {
            let res = db.get(&key).await?;
            println!("{:?}", res);
        },
    }
    Ok(())
}

// canary
// dingo
// iguana
// beskar
// selene / luna

