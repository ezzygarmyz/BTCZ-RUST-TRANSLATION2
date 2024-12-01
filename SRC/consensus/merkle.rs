use crate::uint256::Uint256;

pub fn compute_merkle_root(leaves: &[Uint256]) -> (Uint256, bool) {
    let mut mutated = false;

    if leaves.is_empty() {
        return (Uint256::zero(), false);
    }

    let mut tree = leaves.to_vec();
    while tree.len() > 1 {
        let mut next_level = Vec::new();

        for i in (0..tree.len()).step_by(2) {
            let left = &tree[i];
            let right = if i + 1 < tree.len() {
                &tree[i + 1]
            } else {
                left
            };

            if left == right {
                mutated = true;
            }

            next_level.push(Uint256::hash(&[left.as_bytes(), right.as_bytes()].concat()));
        }
        tree = next_level;
    }

    (tree[0].clone(), mutated)
}
