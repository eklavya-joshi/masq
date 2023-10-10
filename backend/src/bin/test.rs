use ring::error::Unspecified;
use ring::rand::SystemRandom;
use ring::agreement;
use ring::agreement::Algorithm;
use ring::agreement::X25519;
use ring::agreement::EphemeralPrivateKey;
use ring::agreement::PublicKey;
use ring::agreement::UnparsedPublicKey;

fn main() -> Result<(), Unspecified> {
    // Derived a shared secret using ECDH

    let rng = SystemRandom::new();
    let alg: &Algorithm = &X25519;
    let my_private_key: EphemeralPrivateKey = EphemeralPrivateKey::generate(alg, &rng)?;
    let _my_public_key: PublicKey = my_private_key.compute_public_key()?;

    // Send our public key to the peer here
    let peer_public_key: PublicKey = { // Simulate receiving a public key from the peer
        let peer_private_key = EphemeralPrivateKey::generate(alg, &rng)?;
        peer_private_key.compute_public_key()?
    };

    let peer_public_key = UnparsedPublicKey::new(alg, peer_public_key);
    let str = agreement::agree_ephemeral(my_private_key,
                    &peer_public_key,
                    |_shared_secret: &[u8]| {
                        "i love butts"
                    }).unwrap();
    println!("{}", str);
    Ok(())
}