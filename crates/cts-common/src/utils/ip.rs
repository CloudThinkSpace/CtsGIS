use std::net::IpAddr;

///
/// # 获取本地服务器ip地址
pub fn get_local_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
    // 使用网络库来获取本机的IP地址
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_ip = socket.local_addr()?.ip();
    Ok(local_ip)
}

#[cfg(test)]
mod tests {
    use crate::utils::ip::get_local_ip;

    #[test]
    fn test_get_local_ip() {
        let aa = get_local_ip().unwrap();
        println!("{:?}", aa);
    }
}