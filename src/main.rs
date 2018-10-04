mod rsm_data;
mod rsm_tui;

use crate::rsm_data::*;

fn main() {
    let mut rs = RemoteServer::new();

    let name_set = RemoteNameSetter::new("remote1".to_string());
    rs.update(&name_set);

    let url_set = RemoteUrlSetter::new("root@127.0.0.1".to_string());
    rs.update(&url_set);
    
    let port_set = RemotePortSetter::new("23".to_string());
    rs.update(&port_set);

    let rsa_set = RemoteRsaSetter::new("$HOME/.ras/Identi.rsa".to_string());
    rs.update(&rsa_set);
    
    let passwd_set = RemotePasswdSetter::new("123456".to_string());
    rs.update(&passwd_set);

    println!("{:?}", rs);
    println!("{}", rs.format_to_ssh_cmd());
}

