use pretty_assertions::assert_eq;

use testing::attractions::Museum;
use testing::management::VenueManagement;

#[test]
fn venue_management_interacts_with_venue() {
    let mut venue = Museum::new();
    venue.buy_painting("Foo");

    let mut management = VenueManagement::new(venue);
    management.make_money();

    assert_eq!(management.venue.revenue, 25);
}
