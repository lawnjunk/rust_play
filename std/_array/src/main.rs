fn main() {
    // MAP
    let mut name_list = ["glorb", "chorpo", "snov", "lixolix"];
    println!("name_list {:?}", name_list);
    let name_length_list = name_list.map(|name| name.len());
    println!("name_length_list {:?}", name_length_list);
    println!("name_list {:?}", name_list);

    let name_list_ref = name_list.as_ref();
    println!("name_list_ref {:?}", name_list_ref);

    let name_list_mut = name_list.as_mut();
    name_list_mut[0] = "zond";
    println!("name_list_mut {:?}", name_list_mut);

    // ZIP (only in nightly)
    // let age_list = [101, 42, 33, 45];
    // let name_age_list_zip = name_list.zip(age_list);
    // println!("name_age_list_zip {:?}", name_age_list_zip);
}
