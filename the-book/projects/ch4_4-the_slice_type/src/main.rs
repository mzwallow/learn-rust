fn main() {
    let s = String::from("hello world");

    // Slices are references, they also change permission on referenced data.
    let _hello: &str = &s[0..5]; // s:> R-- | _hello:^ R-O
    let _world: &str = &s[6..11];
    let _s2: &String = &s;

    // Range syntax
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..];
    let _slice = &s[0..len];
    let _slice = &s[..len];
    let _slice = &s[..];

    // String Literals Are Slices
    let _s = "Hello, world!";
}
