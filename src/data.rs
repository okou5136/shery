use std::fs;
use anyhow::Context;
use std::path::Path;
use serde::{Serialize, Deserialize};
use clap::{
    Parser,
};


trait Convert {
    fn simplify(&self) -> anyhow::Result<IPAddr>;
}

#[derive(Debug, Default, Clone)]
pub struct IPAddr {
    pub ip: String,
    pub port: Option<u16>,
    pub iptype: IPType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IPType {
    IPV4,
    IPV6,
}

impl Default for IPType {
    fn default() -> Self {
        IPType::IPV4
    }
}

pub fn ip_type(string: &String) -> anyhow::Result<IPType> {
    let mut collons = 0usize;
    let mut dots = 0usize;

    for ch in string.chars().into_iter() {
        if ch == ':' {
            collons += 1;
        }
        else if ch == '.' {
            dots += 1;
        }
    }

    // possible formats are:
    //  000::000:000:000:000
    //  000::000:000:000:000:port
    //  000.000.000.000
    //  000.000.000.000:port
    if collons >= 5 || collons <= 6 {
        Ok(IPType::IPV4)
    }
    else if dots == 3 && collons <= 1 {
        Ok(IPType::IPV6)
    } else {
        return Err(anyhow::anyhow!("Unknown IP Address Type"));
    }
}

impl Convert for Vec<IPAddr> {
    fn simplify(&self) -> anyhow::Result<IPAddr> {
        if self.len() != 1 {
            return Err(anyhow::anyhow!("conversion from Vec<IPAddr> to IPAddr is allowed only when the vector contains exactly one content"));
        }
        return Ok(self[0].clone());
    }
}

impl IPAddr {
    pub fn from_str<P: AsRef<Vec<String>>>(stringvec: P) -> anyhow::Result<Vec<IPAddr>> {
        let mut result: Vec<IPAddr> = Vec::new();

        for string in stringvec.as_ref() {
            let mut chars: Vec<char>= string.chars().collect::<Vec<char>>();
            let iptyperef = ip_type(&string)?;
            let mut collons = 0usize;
            let mut portind: bool = false;
            let mut ipref: String = String::new();
            let mut portref = String::new();

            if iptyperef == IPType::IPV4 {
                for ch in chars.iter() {
                    if *ch == ':' {
                        portind = true;
                    }
                    else if portind == false {
                        ipref.push(*ch);
                    }
                    else if portind == true {
                        portref.push(*ch);
                    }
                }
                result.push(IPAddr {
                    ip: ipref,
                    port: if portref != "".to_string() {Some(portref.parse::<u16>().unwrap())} else {None},
                    iptype: iptyperef,
                });
            } else {
                println!("No Support for IPv6");
                result.push(IPAddr::default());
            }
        }
        return Ok(result);
    }
}

#[derive(Debug, Default)]
pub struct Settings {
    pub host_ip: IPAddr,
    pub allow_ip: Option<Vec<IPAddr>>,
    pub inputpath: Option<String>,
    pub permiscuous: bool,
    pub secure: bool,
}

impl Settings {
    pub fn combine(args: Arguments, conf: Config) -> anyhow::Result<Settings> {
        let mut returnval: Settings = Settings::default();

        returnval.host_ip = if let Some(host) = args.host_ip {
            IPAddr::from_str(vec![host])?.simplify()?
        } else {
            IPAddr::from_str(vec![conf.host_ip])?.simplify()?
        };

        returnval.allow_ip = if let Some(allow) = args.allow_ip {
            Some(IPAddr::from_str(allow)?)
        } 
        else if let  Some(confallow) = conf.allow_ip {
            Some(IPAddr::from_str(confallow)?)
        } else {
            None
        };

        returnval.inputpath = if let Some(input) = args.inputfile {
            Some(input)
        } else {
            None
        };

        return Ok(returnval)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// ip address of the host
    pub host_ip: String,

    /// allowed IP Addresses
    pub allow_ip: Option<Vec<String>>,

    /// determine whether only allow_ip is accepted
    pub premiscuous: bool,

    pub secure: bool,
}

#[derive(Debug, Parser)]
#[clap(author="D.A.", about="simple file-sharing webapp", version="prototype")]
pub struct Arguments {
    #[clap(short='i', long="input", global=true)]

    pub inputfile: Option<String>, 

    #[clap(short='H', long="host", global=true)]
    pub host_ip: Option<String>,

    #[clap(short='a', long="allow", global=true)]
    pub allow_ip: Option<Vec<String>>,

    #[clap(short='p', long="premiscuous", global=true)]
    pub premiscuous: bool,

    #[clap(short='s', long="secure", global=true)]
    pub secure_connection: bool,
}

impl Config {
    pub fn generate() -> anyhow::Result<Self> {
        let default_path: String = {
            let mut returnval: String = String::new();
            println!("Checking if the config file exists...");
            for x in vec!["/home/normie/userconf/shery/conf.json"] {
                if Path::new(x).exists() {
                    returnval = x.to_string();
                    println!("Config file {x} exists!");
                    break;
                }
            }
            returnval
        };

        let config: Config = {
            print!("Opening config file...");
            let text = fs::read_to_string(default_path)?;
            println!(" Done!");
            serde_json::from_str(&text)?
        };

       Ok(config)
    }
}
