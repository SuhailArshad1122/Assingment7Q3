
//we can write :use mytable::mod_in_lib1::sub_mod_in_lib2::alpha;
// and call alpha() directly in main ..... or
// we can do as below.

use mytable;
fn main() {
    mytable::mod_in_lib1::sub_mod_in_lib2::alpha();
}