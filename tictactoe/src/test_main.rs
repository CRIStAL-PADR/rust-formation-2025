// will only be build in the 'test' configuration 
#[cfg(test)]

// a module
mod test {
    #[test]
    fn should_pass(){
        assert!(true); 
    }

    #[test]
    #[should_panic]
    fn should_panic(){
        panic!("Doing wrong thing on purpose here!");
    }
}
