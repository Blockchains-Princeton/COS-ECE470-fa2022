test_code = r'''

#[cfg(test)]
mod tests {
    use crate::types::hash::H256;
    use super::*;
    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }
    #[test]
    fn sp2022autograder011() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }
    #[test]
    fn sp2022autograder012() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }
    #[test]
    fn sp2022autograder013() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }
}

'''
import re
import sys
import os.path as path
file_path = path.join(sys.argv[1], 'src','types','merkle.rs')
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
