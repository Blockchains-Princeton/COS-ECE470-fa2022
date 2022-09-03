test_code = r'''

#[cfg(test)]
mod test {
    use ntest::timeout;
    use crate::types::hash::Hashable;

    #[test]
    #[timeout(60000)]
    fn sp2022autograder031() {
        let (miner_ctx, miner_handle, finished_block_chan) = super::test_new();
        miner_ctx.start();
        miner_handle.start(0);
        let mut block_prev = finished_block_chan.recv().unwrap();
        for _ in 0..2 {
            let block_next = finished_block_chan.recv().unwrap();
            assert_eq!(block_prev.hash(), block_next.get_parent());
            block_prev = block_next;
        }
    }
}

'''
import re
import sys
import os.path as path
file_path = path.join(sys.argv[1], 'src','miner','mod.rs')
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
