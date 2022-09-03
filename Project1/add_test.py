test_code = r'''

#[cfg(test)]
mod test {
    use super::Address;
    #[test]
    fn sp2022autograder001() {
        let test_key = hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d");
        let addr = Address::from_public_key_bytes(&test_key);
        let correct_addr: Address = hex!("1851a0eae0060a132cf0f64a0ffaea248de6cba0").into();
        assert_eq!(addr, correct_addr);
    }
}

'''
test_code_2 = r'''

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::key_pair;
    use ring::signature::KeyPair;
    #[test]
    fn sp2022autograder002() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, key.public_key().as_ref(), signature.as_ref()));
    }
    #[test]
    fn sp2022autograder003() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        let key_2 = key_pair::random();
        let t_2 = generate_random_transaction();
        assert!(!verify(&t_2, key.public_key().as_ref(), signature.as_ref()));
        assert!(!verify(&t, key_2.public_key().as_ref(), signature.as_ref()));
    }
}

'''
import re
import sys
import os.path as path
file_path = path.join(sys.argv[1], 'src','types','address.rs')
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

file_path = path.join(sys.argv[1], 'src','types','transaction.rs')
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
            file_changed.append(test_code_2)
if change_before and change_after:
    print("\033[92m {}\033[00m".format("Changed the test code"))
    with open(file_path, "w") as fout:
        fout.write(''.join(file_changed))
else:
    print("\033[91m {}\033[00m".format("Code format wrong"))