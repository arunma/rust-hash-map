use std::collections::HashMap;

fn main() {
    //let mut vc: Vec<Vec<String>> = Vec::with_capacity(5);
    //vecvec.extend((0..5).map(|_| Vec::new()));
    // let cs = 1.to_string();
    // let cst = &cs[..];
    // let vec = vec![cst; 10];
    //vc.extend((0..5).map(|c| vec![c.to_string(); c]));
    // println!("{:?}", &vc);
    // let res: Vec<String> = vc.iter_mut().flat_map(|v| v.drain(..)).collect();
    // println!("{:?}", res());

    let key = "foo";
    let map = HashMap::from([("foo", 42), ("bar", 43)]);
    /* let result = map
    .iter()
    .find(|(&ekey, _)| ekey == key)
    .map(|(_, value)| value); */

    let result = map
        .iter()
        .find_map(|(&ekey, value)| if ekey == key { Some(value) } else { None });

    /*     let mut map = HashMap::new();
       map.insert(1, "a");
       map.insert(2, "b");
       map.insert(3, "c");

       let result = map
           .iter()
           .find_map(|(key, &value)| if value == "a" { Some(key) } else { None });
    */
    println!("{:?}", result);
}
