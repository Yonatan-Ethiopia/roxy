use clap::Parser;
use std::fs;
use regex::Regex;
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
fn reg_check(proxy_str: String){
	let start = Regex::new(r"#ROXY_START").unwrap();
	let end = Regex::new(r"ROXY_END").unwrap();
	let re = Regex::new(r"(?s)(#ROXY_START).*?(#ROXY_END)").unwrap();
	if let Some(home) = home::home_dir(){
		let bash_path = home.join(".bashrc");
	
		match fs::read_to_string(&bash_path){
			Ok(mut content)=>{
				if !start.is_match(&content) || !end.is_match(&content){
					content = content + "\n#ROXY_START\n#ROXY_END\n";
				}
				let new_con = format!("$1\nexport http_proxy=\"{}\"\nexport https_proxy=\"{}\"\n$2\n", proxy_str, proxy_str);
				let result = re.replace_all(&content, new_con);
				let _ = file_upd(result.into_owned());
			}
			Err(e)=>{
				print!("Sorry the following error occured:\n {}", e);
			}
		}
	}
}
fn clear_bash(){
	let start = Regex::new(r"#ROXY_START").unwrap();
	let end = Regex::new(r"ROXY_END").unwrap();
	let re = Regex::new(r"(?s)(#ROXY_START).*?(#ROXY_END)").unwrap();
	if let Some(home) = home::home_dir(){
		let bash_path = home.join(".bashrc");
	
		match fs::read_to_string(&bash_path){
			Ok(mut content)=>{
				if !start.is_match(&content) || !end.is_match(&content){
					content = content + "\n#ROXY_START\n#ROXY_END\n";
				}
				let new_con = format!("$1\n\n$2\n");
				let result = re.replace_all(&content, new_con);
				let _ = file_upd(result.into_owned());
			}
			Err(e)=>{
				print!("Sorry the following error occured:\n {}", e);
			}
		}
	}
}
fn file_upd( content: String)-> Result<(), std::io::Error>{
	if let Some(home) = home::home_dir(){
		let bash_path = home.join(".bashrc");
		let _ = fs::write(&bash_path, content);
		print!("source ~/.bashrc");
	}
	Ok(())
}

fn main(){
	let args = Cli::parse();
	
	if args.clear{
		print!("echo \"Proxy unset for this session\"; unset http_proxy; unset https_proxy;");
	}
	if args.persist && args.clear{
		clear_bash();
	}
	else if let( Some(ip), Some(port)) = (args.ip, args.port){
		let proxy_str = format!("http://{}:{}", ip, port);
		if args.persist{
			reg_check(proxy_str);
		}
		else{
			print!("echo \"proxy updated for this session\"; export http_proxy=\"{}\"; export https_proxy=\"{}\";", proxy_str, proxy_str);
		}
	}
	else if args.show{
		print!("echo \"The current proxy setup:\"; echo \"http_proxy = \" {}; echo \"https_proxy = \" {};", "${http_proxy}", "${https_proxy}");
	}

}
