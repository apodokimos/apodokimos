use apodokimos_log::{LogClient, SignedEntry, WitnessSignature};
use ed25519_dalek::{Signer, SigningKey};

fn test_entry(i: u8) -> SignedEntry {
    SignedEntry {
        payload: vec![i, i.wrapping_add(1), i.wrapping_add(2)],
        signer_did: format!("did:apodokimos:submitter:{}", i),
        signature: "deadbeef".to_string(),
    }
}

fn keypair(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

#[test]
fn local_log_submit_returns_verifiable_inclusion_proof() {
    let mut client = LogClient::new("apodokimos-local", "00");
    let entry = test_entry(7);

    let proof = client.submit(entry.clone()).expect("submit should succeed");
    let sth = client.current_sth();

    assert!(client.verify_inclusion(&entry, &proof, &sth));
}

#[test]
fn local_log_consistency_holds_between_snapshots() {
    let mut client = LogClient::new("apodokimos-local", "00");

    client.submit(test_entry(1)).expect("submit 1");
    let old_sth = client.current_sth();

    client.submit(test_entry(2)).expect("submit 2");
    client.submit(test_entry(3)).expect("submit 3");
    let new_sth = client.current_sth();

    assert!(client.verify_consistency(&old_sth, &new_sth));
}

#[test]
fn local_log_witness_cosignatures_verify() {
    let witness_a = keypair(11);
    let witness_b = keypair(22);

    let client = LogClient::new("apodokimos-local", "00");
    let sth = client.current_sth();

    let message = sth.signing_bytes();
    let sig_a = witness_a.sign(&message);
    let sig_b = witness_b.sign(&message);

    let witnesses = vec![
        WitnessSignature {
            witness_id: "did:apodokimos:witness:A".to_string(),
            public_key: hex::encode(witness_a.verifying_key().to_bytes()),
            signature: hex::encode(sig_a.to_bytes()),
        },
        WitnessSignature {
            witness_id: "did:apodokimos:witness:B".to_string(),
            public_key: hex::encode(witness_b.verifying_key().to_bytes()),
            signature: hex::encode(sig_b.to_bytes()),
        },
    ];

    assert!(client.verify_witness_signatures(&sth, &witnesses));
}
