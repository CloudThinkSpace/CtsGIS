use cts_license::license::rsa::License;

pub fn license(server_name: &str) {
    License::check_license("./licence.lic", server_name).expect("failed to check licence");
}
