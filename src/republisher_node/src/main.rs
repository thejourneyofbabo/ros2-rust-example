use std::sync::{Arc, Mutex}; // (1)
use std_msgs::msg::String as StringMsg;

struct RepublisherNode {
    node: Arc<rclrs::Node>,
    _subscription: Arc<rclrs::Subscription<StringMsg>>,
    data: Arc<Mutex<Option<StringMsg>>>, // (2)
    // Add this new field to the RepublisherNode struct, after the subscription:
    publisher: Arc<rclrs::Publisher<StringMsg>>,
}

impl RepublisherNode {
    fn new(context: &rclrs::Context) -> Result<Self, rclrs::RclrsError> {
        let node = rclrs::Node::new(context, "republisher")?;
        let data = Arc::new(Mutex::new(None)); // (3)
        let data_cb = Arc::clone(&data);
        let publisher = node.create_publisher("out_topic", rclrs::QOS_PROFILE_DEFAULT)?;
        let _subscription = {
            // Create a new shared pointer instance that will be owned by the closure
            node.create_subscription(
                "in_topic",
                rclrs::QOS_PROFILE_DEFAULT,
                move |msg: StringMsg| {
                    // This subscription now owns the data_cb variable
                    *data_cb.lock().unwrap() = Some(msg); // (4)
                },
            )?
        };
        Ok(Self {
            node,
            _subscription,
            publisher,
            data,
        })
    }

    fn republish(&self) -> Result<(), rclrs::RclrsError> {
        if let Some(s) = &*self.data.lock().unwrap() {
            self.publisher.publish(s)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;
    let republisher = Arc::new(RepublisherNode::new(&context)?);
    let republisher_other_thread = Arc::clone(&republisher);
    std::thread::spawn(move || -> Result<(), rclrs::RclrsError> {
        loop {
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(1000));
            republisher_other_thread.republish()?;
        }
    });
    rclrs::spin(Arc::clone(&republisher.node))
}
