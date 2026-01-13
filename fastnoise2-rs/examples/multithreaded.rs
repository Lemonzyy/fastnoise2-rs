use fastnoise2::SafeNode;

fn main() {
    let node = SafeNode::from_encoded_node_tree("E@BBZEE@BD8JFgIECArXIzwECiQIw/UoPwkuAAE@BJDQAE@BC@AIEAJBwQDZmYmPwsAAIA/HAMAAHBCBA==").unwrap();

    // A SafeNode is an Arc wrapping a Node. Thus, cloning a SafeNode does not reinstantiate it.
    let n1 = node.clone();
    let t1 = std::thread::spawn(move || {
        for i in 0..50 {
            let noise = n1.gen_single_2d(i as f32, 0.0, 1234);
            println!("[Thread 1] {i}: {noise}");
        }
    });

    let t2 = std::thread::spawn(move || {
        for i in 0..50 {
            let noise = node.gen_single_2d(i as f32, 0.0, 1234);
            println!("[Thread 2] {i}: {noise}");
        }
    });

    // Don't forget to wait for threads to finish before quitting, to avoid segmentation errors.
    t1.join().unwrap();
    t2.join().unwrap();
}
