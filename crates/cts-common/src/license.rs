use cts_license::license::rsa::License;

pub fn license(server_name: &str) {
    License::check_license("./license.lic", server_name).expect("failed to check licence");
}


#[cfg(test)]
mod test {

    use crate::license::license;

    #[test]
    fn test() {
        license("FeatureServer");
    }
}
