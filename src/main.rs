use clap::Parser;

#[derive(Parser)]
struct Cli{
	ip: Option<String>,
	port: Option<String>,
	
	#[arg(short, long)]
	persist:bool,
	
	#[arg(short, long)]
	clear:bool,
	
	#[arg(long)]
	show:bool
}

fn main(){
	let args = Cli::parse();
	
	if args.clear{
		print!("echo \"Proxy unset for this session\"; unset http_proxy; unset https_proxy;");
	}
	else if let( Some(ip), Some(port)) = (args.ip, args.port){
		let proxy_str = format!("http://{}:{}", ip, port);
		print!("echo \"proxy updated for this session\"; export http_proxy=\"{}\"; export https_proxy=\"{}\";", proxy_str, proxy_str);
	}
	else if args.show{
		print!("echo \"The current proxy setup:\"; echo \"http_proxy = \" {}; echo \"https_proxy = \" {};", "${http_proxy}", "${https_proxy}");
	}
}
