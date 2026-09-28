use crate::Sirk;

#[test]
fn reports_unavailable_service() {
    let error = match Sirk::connect_to("http://127.0.0.1:0", ".") {
        Ok(_) => panic!("connection unexpectedly succeeded"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("Start S.I.R.K. with `sirk http`")
    );
}
