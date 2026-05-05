use sha2::{Digest, Sha256};

/// SHA-256 over `0x00 || leaf_bytes` per RFC 6962 leaf hashing.
pub fn leaf_hash(leaf_bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update([0x00]);
    hasher.update(leaf_bytes);
    hasher.finalize().into()
}

/// SHA-256 over `0x01 || left || right` per RFC 6962 node hashing.
pub fn node_hash(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update([0x01]);
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

/// Computes Merkle root from leaf hashes.
pub fn merkle_root(leaf_hashes: &[[u8; 32]]) -> [u8; 32] {
    if leaf_hashes.is_empty() {
        return [0u8; 32];
    }
    if leaf_hashes.len() == 1 {
        return leaf_hashes[0];
    }

    let k = largest_power_of_two_less_than(leaf_hashes.len() as u64) as usize;
    let left = merkle_root(&leaf_hashes[..k]);
    let right = merkle_root(&leaf_hashes[k..]);
    node_hash(left, right)
}

/// Returns inclusion audit path for `leaf_index`.
pub fn inclusion_path(leaf_hashes: &[[u8; 32]], leaf_index: usize) -> Option<Vec<[u8; 32]>> {
    if leaf_hashes.is_empty() || leaf_index >= leaf_hashes.len() {
        return None;
    }
    Some(inclusion_path_rec(leaf_hashes, leaf_index))
}

fn inclusion_path_rec(leaf_hashes: &[[u8; 32]], leaf_index: usize) -> Vec<[u8; 32]> {
    if leaf_hashes.len() <= 1 {
        return Vec::new();
    }

    let k = largest_power_of_two_less_than(leaf_hashes.len() as u64) as usize;

    if leaf_index < k {
        let mut path = inclusion_path_rec(&leaf_hashes[..k], leaf_index);
        path.push(merkle_root(&leaf_hashes[k..]));
        path
    } else {
        let mut path = inclusion_path_rec(&leaf_hashes[k..], leaf_index - k);
        path.push(merkle_root(&leaf_hashes[..k]));
        path
    }
}

/// Verifies an inclusion proof against an expected root.
pub fn verify_inclusion_path(
    leaf_hash_value: [u8; 32],
    leaf_index: u64,
    tree_size: u64,
    path: &[[u8; 32]],
    expected_root: [u8; 32],
) -> bool {
    if tree_size == 0 || leaf_index >= tree_size {
        return false;
    }

    let mut idx = leaf_index;
    let mut size = tree_size;
    let mut hash = leaf_hash_value;

    for sibling in path {
        if idx % 2 == 1 || idx == size - 1 {
            hash = node_hash(*sibling, hash);
        } else {
            hash = node_hash(hash, *sibling);
        }
        idx /= 2;
        size = size.div_ceil(2);
    }

    hash == expected_root
}

/// Largest power of two strictly less than n, for n > 1.
fn largest_power_of_two_less_than(n: u64) -> u64 {
    debug_assert!(n > 1);
    let mut k = 1u64;
    while (k << 1) < n {
        k <<= 1;
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inclusion_round_trip_small_tree() {
        let leaves = [b"a".to_vec(), b"b".to_vec(), b"c".to_vec(), b"d".to_vec()];
        let hashes: Vec<[u8; 32]> = leaves.iter().map(|l| leaf_hash(l)).collect();
        let root = merkle_root(&hashes);

        for i in 0..hashes.len() {
            let proof = inclusion_path(&hashes, i).unwrap();
            assert!(verify_inclusion_path(
                hashes[i],
                i as u64,
                hashes.len() as u64,
                &proof,
                root
            ));
        }
    }
}
