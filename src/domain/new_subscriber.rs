use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

/// A validated subscriber.
///
/// This struct can only be constructed if both the email and name have passed
/// their respective validation rules.
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
