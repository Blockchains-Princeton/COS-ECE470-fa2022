test_code = r'''

#[cfg(test)]
mod test {
    use ntest::timeout;
    use crate::types::block::generate_random_block;
    use crate::types::hash::Hashable;

    use super::super::message::Message;
    use super::generate_test_worker_and_start;

    #[test]
    #[timeout(60000)]
    fn sp2022autograder041() {
        let (test_msg_sender, _server_receiver, v) = generate_test_worker_and_start();
        let random_block = generate_random_block(v.last().unwrap());
        let mut peer_receiver = test_msg_sender.send(Message::NewBlockHashes(vec![random_block.hash()]));
        let reply = peer_receiver.recv();
        if let Message::GetBlocks(v) = reply {
            assert_eq!(v, vec![random_block.hash()]);
        } else {
            panic!();
        }
    }
    #[test]
    #[timeout(60000)]
    fn sp2022autograder042() {
        let (test_msg_sender, _server_receiver, v) = generate_test_worker_and_start();
        let h = v.last().unwrap().clone();
        let mut peer_receiver = test_msg_sender.send(Message::GetBlocks(vec![h.clone()]));
        let reply = peer_receiver.recv();
        if let Message::Blocks(v) = reply {
            assert_eq!(1, v.len());
            assert_eq!(h, v[0].hash())
        } else {
            panic!();
        }
    }
    #[test]
    #[timeout(60000)]
    fn sp2022autograder043() {
        let (test_msg_sender, server_receiver, v) = generate_test_worker_and_start();
        let random_block = generate_random_block(v.last().unwrap());
        let mut _peer_receiver = test_msg_sender.send(Message::Blocks(vec![random_block.clone()]));
        let reply = server_receiver.recv().unwrap();
        if let Message::NewBlockHashes(v) = reply {
            assert_eq!(v, vec![random_block.hash()]);
        } else {
            panic!();
        }
    }
}

'''
import re
import sys
import os.path as path
file_path = path.join(sys.argv[1], 'src','network','worker.rs')
print(path.dirname(file_path), end=' ')
before_pat = r'// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. BEFORE TEST'
after_pat = r'// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. AFTER TEST'
change_before = False
change_after = False
file_changed = []
with open(file_path) as fin:
    for line in fin:
        if after_pat in line:
            change_after = True
        if not change_before or change_before and change_after:
            file_changed.append(line)
        if before_pat in line:
            change_before = True
            file_changed.append(test_code)
if change_before and change_after:
    print("\033[92m {}\033[00m".format("Changed the test code"))
    with open(file_path, "w") as fout:
        fout.write(''.join(file_changed))
else:
    print("\033[91m {}\033[00m".format("Code format wrong"))
