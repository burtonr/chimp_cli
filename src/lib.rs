pub trait CodeTraits {
    fn clone(&self) -> String;
}

pub mod code {
    use crate::CodeTraits;

    pub struct GitHub {
        pub owner: String
    }
    
    impl CodeTraits for GitHub {
        fn clone(&self) -> String {
            format!("git clone ({})", self.owner)
        }
    }
}