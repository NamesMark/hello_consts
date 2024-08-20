use hello_world_macro::*;

#[test]
fn generation_works() {
    generate_named_constants!(1..=15);
    assert_eq!(15, fifteen);

    generate_named_constants!(50..50);
    assert_eq!(50, fifty);

    generate_named_constants!(999_999..=1_000_000);
    println!("{}", onemillion);


    generate_named_constants!(51..=100000);
    println!("{}", onehundred);
}
