// const crypto = require('crypto');
// var aes256 = require("aes256");
// //the secret key used for encrypting and decrypting messages
// var secret_key = "uI2ooxtwHeI6q69PS98fx9SWVGbpQohO";
// //returns the encrypted text
// export const to_Encrypt = (text) => {
//     var encrypted = aes256.encrypt(secret_key, text);
//     return encrypted;
// };
// //welcome message is not decrypted
// export const to_Decrypt = (cipher) => {

//     var decrypted = aes256.decrypt(secret_key, cipher);
//     return decrypted;
// };

const crypto = require('crypto-browserify');

global.Buffer = global.Buffer || require("buffer").Buffer;

export default function aes() {

    console.log(crypto.randomBytes(32));
    console.log(crypto.createECDH('secp521r1'));

    const alice = crypto.createECDH('secp521r1');
    const aliceKey = alice.generateKeys();

    // Generate Bob's keys...
    const bob = crypto.createECDH('secp521r1');
    const bobKey = bob.generateKeys();

    // Exchange and generate the secret...
    const aliceSecret = alice.computeSecret(bobKey);
    const bobSecret = bob.computeSecret(aliceKey);

    console.log("alice: " + aliceSecret.toString('hex'));
    console.log("bob: " + bobSecret.toString('hex'));
    console.log(aliceSecret.toString('hex') === bobSecret.toString('hex'))

    return;
}
