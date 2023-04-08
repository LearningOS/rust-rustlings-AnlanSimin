// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.


mod sausage_factory {
    pub mod secrete {
        // Don't let anybody outside of this module see this!
        fn get_secret_recipe() -> String {
            String::from("Ginger")
        }
       pub fn make_sausage() {
            get_secret_recipe();
            println!("sausage!");
        }
    }
}

fn main() {
    sausage_factory::secrete::make_sausage();
}
