use litesvm::LiteSVM;

pub fn setup_svm() -> LiteSVM {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(logs::ID, "../../target/deploy/logs.so")
        .unwrap();
    svm
}
