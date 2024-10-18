use rclrs::{create_node, Context, Node, RclrsError, Subscription, QOS_PROFILE_DEFAULT};
use std::{env, sync::Arc};
use tutorial_interfaces::msg::Num;

pub struct MinimalSubscriber {
    node: Arc<Node>,
    _subscription: Arc<Subscription<Num>>,
}

impl MinimalSubscriber {
    fn new(context: &Context) -> Result<Self, RclrsError> {
        let node = create_node(context, "minimal_subscriber")?;
        let _subscription =
            node.create_subscription::<Num, _>("topic", QOS_PROFILE_DEFAULT, |msg: Num| {
                println!("I heard: '{}'", msg.num);
            })?;

        Ok(Self {
            node,
            _subscription,
        })
    }
}

fn main() -> Result<(), RclrsError> {
    let context = Context::new(env::args())?;
    let subscriber = MinimalSubscriber::new(&context)?;
    rclrs::spin(subscriber.node)
}
