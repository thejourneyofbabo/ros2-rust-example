use rclrs::{create_node, Context, Node, Publisher, RclrsError, QOS_PROFILE_DEFAULT};
use std::{env, sync::Arc, time::Duration};
use tutorial_interfaces::msg::Num as NumMsg;

struct MinimalPublisher {
    node: Arc<Node>,
    publisher: Arc<Publisher<NumMsg>>,
    count: usize,
}

impl MinimalPublisher {
    fn new(context: &Context) -> Result<Self, RclrsError> {
        let node = create_node(context, "minimal_publisher")?;
        let publisher = node.create_publisher("topic", QOS_PROFILE_DEFAULT)?;
        Ok(Self {
            node,
            publisher,
            count: 0,
        })
    }

    fn timer_callback(&mut self) -> Result<(), RclrsError> {
        let mut message = NumMsg::default();
        message.num = self.count as i64;
        println!("Publishing: '{}'", message.num);
        self.publisher.publish(&message)?;
        self.count += 1;
        Ok(())
    }
}

fn main() -> Result<(), RclrsError> {
    let context = Context::new(env::args())?;
    let mut publisher = MinimalPublisher::new(&context)?;

    loop {
        publisher.timer_callback()?;
        std::thread::sleep(Duration::from_millis(500));
    }
}
