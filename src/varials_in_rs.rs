pub fn variables_in_rust() {



    // Mutable variables: {v2}
    let v2 = 4;

    // UnMutable variables: {v3}
    let v3 = 6;


    //nMutable and Const variables: {VCONST}
    const VCONST: u32 = 60* 60* 4;

    let x = 5;

    let x = x+1;
    {
        let x= x *2;
        println!("The value of x in the inner scope is: {x}");
    }

      println!("The value of x is: {x}");

    println!("Mutable variables: {v2}");
    println!("UnMutable variables: {v3}");
    println!("UnMutable and Const variables: {VCONST}");

   
}