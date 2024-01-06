pub fn ownership_rs_tuto(){


    // Variable Scope
    let s = "Hello";


    {  // s is not declare here and not available
        let s = "Coded"; // s is declare here and available

    } // this scope is now over, and s is no longer valid

} 