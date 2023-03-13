fn main() -> headr::MyResult<()> {
    headr::run(headr::get_flags().unwrap()).unwrap();
    Ok(())
}
