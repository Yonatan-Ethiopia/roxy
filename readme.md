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

### 1) If you already have rust and cargo set up use this :  
 - ***1. Install the binary***
    `cargo install --git https://github.com/Yonatan-Ethiopia/roxy`
 - **2.Add the Shell Wrapper**
     Add this snippet to the bottom of your `~/.bashrc`:  
     ```
     roxy(){
        local output
        output=$(command roxy "$@")
        if [[ "$1" == "--help" || "$1" == "-h" ]]; then
            echo "$output"
        else 
            eval "$output"
        fi
    }
     ```
 - **3.Refresh your shell**
    `source ~/.bashrc`  
    
### 2) If dont have rust and cargo setup and you want to directly download the binary file follow this:
  - **1. Install the binary from** https://github.com/Yonatan-Ethiopia/roxy/releases/tag/v0.1.0 
  
  - **2. Make roxy executable**  
     Do as below.  
      ```
      # Go to where you downloaded it
      cd ~/Downloads

      # Make it executable
      chmod +x roxy

      # Move it to a global location
      sudo mv roxy /usr/local/bin/
      ``` 
  
  - **3. Add the Shell Wrapper like in the first option**  
    ```
    roxy(){
        local output
        output=$(command roxy "$@")
        if [[ "$1" == "--help" || "$1" == "-h" ]]; then
            echo "$output"
        else 
            eval "$output"
        fi
     }
    ```  

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

**Check for your current internet speed:**  
  `roxy --speed`  
    

---
## Available features

 **1.Setting a proxy for the current session**  
 
 **2.Looking up what proxy setup is being used in this session**  
 
 **3.Clearing or unsetting the current setup being used in this session**  
 
 **4.Setting a proxy globally in `.bashrc` file(It will keep a backup of your bashrc file in ~/.bashrc.bak incase something goes wrong)**  
 
 **5.Removing proxy settings from `.bashrc` file**  
 
 **6.Checking your internet speed**  
 
 
 ---

## Roadmap( Upcoming Features)

 I am actively working on expanding Roxy to be a universal proxy manager:
   - **SOCKS% Support**: Support for `ALL_PROXY` and SOCKS configuraions.
   - **SSH Config Integration**: Easily toggle proxies for you SSH config files.
   - **Profile Management**: Save presets like `roxy --save home` or `roxy --load work` . 
   -  **Health Check**: Ping a URL through the proxy to verify its actually working before setting it.  
   - **Support for zsh files**
   
## Built with🏗
  -**Rust**  
  
  -**Clap**

---
## Why Rust?
 I am using this project(and some others i will be releasing) to learn and get my self familiar with the Rust language and its systems programming capabilities.
