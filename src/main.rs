use clap::Parser;

#[derive(Parser)]
struct Cli{
	ip: Option<String>,
	port: Option<String>,
	
	#[arg(short, long)]
	persist:bool,
	
	#[arg(short, long)]
	clear:bool
}

fn main(){
	let args = Cli::parse();
	
	if args.clear{
		print!("unset http_proxy; unset https_proxy;");
	}
	else if let( Some(ip), Some(port)) = (args.ip, args.port){
		let proxy_str = format!("http://{}:{}", ip, port);
		print!("export http_proxy=\"{}\"; export https_proxy=\"{}\";", proxy_str, proxy_str);
	}
}
