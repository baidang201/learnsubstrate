use std::path::PathBuf;
use structopt::StructOpt;
use std::fmt;


#[derive(StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    cmd: Option<Command>

}

#[derive(StructOpt)]
#[structopt(about = "the stupid content tracker")]
enum Command {
    #[structopt(name = "plus")]
    Plus {
        #[structopt(short)]
        a: i64,
        #[structopt(short)]
        b: i64,
    },
    #[structopt(name = "minus")]
    Minus {
        #[structopt(short)]
        a: i64,
        #[structopt(short)]
        b: i64,
    },
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(subcommand) = &self.cmd{
            write!(f, "custom: debug:{}  command:{}", self.debug, subcommand)
        } 
        else {
            write!(f, "custom: debug:{}  ", self.debug)
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Command::Plus{a, b} => {
                write!(f, "custom: struct plus {} {}", a, b)
            },
            Command::Minus{a, b} => {
                write!(f, "custom: struct minus {} {}", a, b)
            },
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", opt);

    
    let mut args = std::env::args();
    if let Some(arg) = args.next() {
        print!("args: {}\n", arg);

        for arg in args {
            print!("arg {} \n", arg);
        }
    }

    if let Some(subcommand) = opt.cmd{
        match subcommand {
            Command::Plus{a, b}=>{
                println!("handle Plus:  {:?}", a+b);
            },
            Command::Minus{a, b}=>{
                println!("handle Minus:  {:?}", a-b);
            }
        }
    }
}