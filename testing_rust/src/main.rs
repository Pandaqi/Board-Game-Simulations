use std::collections::HashMap;

// You can also use this trick to iterate over an enum: https://stackoverflow.com/questions/59600739/access-struct-field-by-variable
// But it requires lots of work from me anyway and feels non-idiomatic

// Technique for inserting enums inside other enums, so our strategies can be in a HashMap
// Numeric ones simply get integer from 0 to 15. Later, they are divided by 15 to get a percentage probability
#[derive(Eq, PartialEq, Debug)]
enum TypeA {
    Random,
    Never,
    Always
}

#[derive(Eq, PartialEq, Debug)]
enum TypeB {
    Random,
    Never,
    Always
}

#[derive(Eq, PartialEq, Debug)]
enum TypeC {
    Always = 15,
    Never = 0,
    Sometimes = 8
}

#[derive(Eq, PartialEq, Debug)]
enum StratAll {
    TypeA(TypeA),
    TypeB(TypeB)
}


fn main() {
    let strat = StratAll::TypeA(TypeA::Always);
    println!("{}", strat == StratAll::TypeA(TypeA::Always));

    let strats: HashMap<&str, StratAll> = HashMap::from([
        ("typeA", StratAll::TypeA(TypeA::Never)),
        ("typeB", StratAll::TypeB(TypeB::Random))
    ]);

    for (k,v) in strats.iter()
    {
        println!("{:#?}", k);
        println!("{:#?}", v);
    }
}
