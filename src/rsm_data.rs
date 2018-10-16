
//#[derive(Debug)]
//pub struct RemoteServer {
//    // 名称
//    name: String,
//    // url
//    url: String,
//    // 端口
//    port: String,
//    // 密码
//    password: String,
//    // rsa key
//    rsa_key: String,
//}
//
//impl RemoteServer {
//    pub fn new() -> RemoteServer {
//        RemoteServer{
//            name: "".to_string(),
//            url: "".to_string(),
//            port: "22".to_string(),
//            password: "".to_string(),
//            rsa_key: "".to_string(),
//        }
//    }
//
//    // 更新信息
//    pub fn update(&mut self, sr: &SetRemoteServer) {
//        sr.set(self);
//    }
//
//    // 格式化成ssh命令
//    pub fn format_to_ssh_cmd(&self) -> String {
//        let mut cmd = String::new();
//
//        cmd.push_str("ssh url -p ");
//        //cmd.push_str(self.port.into_string());
//
//        if ! self.rsa_key.is_empty() {
//            cmd.push_str("-i ");
//        //    cmd.push_str(self.rsa_key);
//            return cmd
//        }
//
//        cmd
//    }
//}
//
//pub trait SetRemoteServer {
//    fn set(&self, rs: &mut RemoteServer);
//}
//
//// 设置名称
//pub struct RemoteNameSetter {
//    pub name: String
//}
//
//impl RemoteNameSetter {
//    pub fn new(name: String) -> RemoteNameSetter {
//        RemoteNameSetter{
//            name
//        }
//    }
//}
//
//impl SetRemoteServer for RemoteNameSetter {
//    fn set(&self, rs: &mut RemoteServer) {
//        rs.name = self.name.clone();
//    }
//}
//
//// 设置url
//pub struct RemoteUrlSetter {
//    url: String
//}
//
//impl RemoteUrlSetter {
//    pub fn new(url: String) -> RemoteUrlSetter {
//        RemoteUrlSetter {
//            url
//        }
//    }
//}
//
//impl SetRemoteServer for RemoteUrlSetter{
//    fn set(&self, rs: &mut RemoteServer) {
//        rs.url = self.url.clone();
//    }
//}
//
//// 设置密码
//pub struct RemotePasswdSetter {
//    passwd: String
//}
//
//impl RemotePasswdSetter {
//    pub fn new(passwd: String) -> RemotePasswdSetter {
//        RemotePasswdSetter {
//            passwd
//        }
//    }
//}
//
//impl SetRemoteServer for RemotePasswdSetter {
//    fn set(&self, rs: &mut RemoteServer) {
//        rs.password = self.passwd.clone();
//    }
//}
//
//// 设置rsa key
//pub struct RemoteRsaSetter {
//    rsa_key: String
//}
//
//impl RemoteRsaSetter {
//    pub fn new(rsa_key: String) -> RemoteRsaSetter {
//        RemoteRsaSetter{
//            rsa_key
//        }
//    }
//}
//
//impl SetRemoteServer for RemoteRsaSetter {
//    fn set(&self, rs: &mut RemoteServer) {
//        rs.rsa_key = self.rsa_key.clone();
//    }
//}
//
//// 设置端口
//pub struct RemotePortSetter {
//    port: String
//}
//
//impl RemotePortSetter {
//    pub fn new(port: String) -> RemotePortSetter {
//        RemotePortSetter {
//            port
//        }
//    }
//}
//
//impl SetRemoteServer for RemotePortSetter {
//    fn set(&self, rs: &mut RemoteServer) {
//        rs.port = self.port.clone();
//    }
//}
//
//// 待学习更好的更简洁的方法
