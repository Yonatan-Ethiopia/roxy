# Roxy🦀

**Roxy** is a minimalist CLI proxy manager built in rust. It solves the headache of manually exporting `http_proxy` and `https_proxy` variables or constantly editing your .bashrc when your proxy settings change.

---

## The problem

Usually, setting a proxy in the terminal involves repetitve tasks:
  - Typing export  `http_proxy=http://IP:PORT`  and export `https_proxy=http://IP:PORT`
  - Forgetting to unset them and wonder why ***curl*** is failing later.
  - Manually digging through `bashrc`  to make a proxy permanent.

---
## The solution
**Roxy** allow you to update your permanent config with a single command: `roxy <IP> <PORT>`.

---
## Installation

For now you would need to already have rust to install and use it.
 - ***1. Install the binary***
    `cargo install --git https://github.com/YOUR_USERNAME/roxy`
 - **2.Add the Shell Wrapper**
     Add this snippet to the bottom of your `~/.bashrc` (or `~/.zshrc` if you use Zsh):
     `roxy(){
        local output
        output=$(command roxy "$@")
        eval "$output"
       }`
 - ***3.Refresh your shell***
    `source ~/.bashrc`

---
## Usage🛠

**Set a temporary proxy(for current session only):**  

  ` roxy <IP> <PORT> `  
  
**Show current proxy status:**  

  `roxy --show`  
  
**Clear all proxy variables:**  

  `roxy --clear` or `roxy -c`  
  
**Save proxy setting globally in `.bashrc` file**  

  `roxy --persist <IP> <PORT>`  
  
**Remove proxy settings from `.bashrc` file**  
  `roxy --persist --clear`  
  

---
## Available features

 **1.Setting a proxy for the current session**  
 
 **2.Looking up what proxy setup is being used in this session**  
 
 **3.Clearing or unsetting the current setup being used in this session**  
 
 **4.Setting a proxy globally in `.bashrc` file**  
 
 **5.Removing proxy settings from `.bashrc` file**  
 
 
 ---

## Roadmap( Upcoming Features)

 I am actively working on expanding Roxy to be a universal proxy manager:
   - **SOCKS% Support**: Support for `ALL_PROXY` and SOCKS configuraions.
   - **SSH Config Integration**: Easily toggle proxies for you SSH config files.
   - **Profile Management**: Save presets like `roxy --save home` or `roxy --load work` . 
   -  **Health Check**: Ping a URL through the proxy to verify its actually working before setting it.
   
## Built with🏗
  -**Rust**  
  
  -**Clap**

---
## Why Rust?
 I am using this project(and some others i will be releasing) to learn and get my self familiar with the Rust language and its systems programming capabilities.
