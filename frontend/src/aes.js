const crypto = require('crypto-browserify');

global.Buffer = global.Buffer || require("buffer").Buffer;

export default function aes() {

    const alice = crypto.createECDH('secp256k1');
    alice.generateKeys();

    // Generate Bob's keys...
    const bob = crypto.createECDH('secp256k1');
    bob.generateKeys();

    const alicePublic = alice.getPublicKey().toString('base64');
    const bobPublic = bob.getPublicKey().toString('base64');

    const aliceShared = alice.computeSecret(bobPublic, 'base64', 'hex');
    const bobShared = bob.computeSecret(alicePublic, 'base64', 'hex');

    const message = "i like poop";

    const iv = crypto.randomBytes(16);
    const cipher = crypto.createCipheriv('aes-256-gcm', Buffer.from(aliceShared, 'hex'), iv);

    let encrypted = cipher.update(message, 'utf8', 'hex');
    encrypted += cipher.final('hex');

    const auth_tag = cipher.getAuthTag().toString('hex');
    console.table({iv: iv.toString('hex'), encrypted: encrypted, auth_tag: auth_tag});

    const payload = iv.toString('hex') + encrypted + auth_tag;
    const payload_base64 = Buffer.from(payload, 'hex').toString('base64');



    const bob_payload = Buffer.from(payload_base64, 'base64').toString('hex');

    const bob_iv = bob_payload.substring(0, 32);
    const bob_encrypted = bob_payload.substring(32, bob_payload.length - 32);
    const bob_auth_tag = bob_payload.substring(bob_payload.length - 32, bob_payload.length);

    console.table({bob_iv: bob_iv, bob_encrypted: bob_encrypted, bob_auth_tag: bob_auth_tag});

    try {
        const decipher = crypto.createDecipheriv(
            'aes-256-gcm',
            Buffer.from(bobShared, 'hex'),
            Buffer.from(bob_iv, 'hex')
        );

        decipher.setAuthTag(Buffer.from(bob_auth_tag, 'hex'));

        let decrypted = decipher.update(bob_encrypted, 'hex', 'utf8');
        decrypted += decipher.final('utf8');
        console.log('decrypted', decrypted);
    } catch (error) {
        console.log(error);
    }

    return;
}
