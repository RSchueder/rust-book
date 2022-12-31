mod front_of_house; 

use crate::front_of_house::hosting;
// re-exporting involves bringing an item into scope but also making that item available for others to bring into their scope.
pub use crate::front_of_house::hosting as restaurant_hosting;

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative path
    hosting::seat_at_table();
}

mod customer {
    pub fn eat_at_restaurant(){
        // hosting is defined in a use statement within the parent module
        super::hosting::add_to_waitlist();
    }
}