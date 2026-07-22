#[cfg(test)]
mod tests {

    use super::*;
    use soroban_sdk::{Env, Address, String};

    #[test]
    fn happy_path() {
        // professor creates record successfully
    }

    #[test]
    #[should_panic]
    fn unauthorized_professor() {
        // unauthorized add
    }

    #[test]
    fn storage_updated() {
        // verify storage
    }

    #[test]
    fn multiple_records() {
        // verify append
    }

    #[test]
    fn empty_student() {
        // no records yet
    }

}