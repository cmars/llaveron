extern crate sequoia_openpgp as openpgp;
use openpgp::parse::Parse;

const KEY: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----
 
     xjMEXAfmvxYJKwYBBAHaRw8BAQdAVNM03IK1KDgDNCbf4XcARhfqzyx425FEJMQ5
     qF+DrwHNF+G8iM+BzrnPg8+Ezr/PhM6tzrvOt8+CwoQEExYKADYCHgMCmwEFglwH
     5r8FiQWfpgAWIQTAh0R4plxUCh9zcrSiLq1hTRF0SgkQoi6tYU0RdEoCFQoAALip
     AP4sSVgNJogb/v0Qst0+WlmrJ6upG8Ynao5mnRFmfx2LjAEAyGJJBaEBB+x4kOse
     9uACwAXFhBRLN9zGgbyySQ3fRwjOMwRcB+a/FgkrBgEEAdpHDwEBB0BXBFWMeVd1
     nNn/VqTVEgY3wknX/KkKfMWhslFJoyZ4L8LAOAQYFgoAMwKbAgWCXAfmvwWJBZ+m
     ABYhBMCHRHimXFQKH3NytKIurWFNEXRKCRCiLq1hTRF0SgIVCgB3dqAEGRYKACcF
     glwH5r8WIQRnpIdTo4Cms7fffcXmxol6TO+JJAkQ5saJekzviSQAAMuvAQDdRfbM
     u2bDtVqNLIP/0WD/5X0us49r1yXMH+Ilg5NEEQEAuSQ1pY+reS62ETUS0uKYhxxv
     7OOsr8YM/ZMQ0exZsw/u+QEAuakAXrR7uFmWyigopQ7qMYfnK5zNfQNykvony5tS
     HpEBAJs3ZwHq+Q0ziAZNgcvdp0mklx8IXd8x59NjiP1t3mUBzjgEXAfmvxIKKwYB
     BAGXVQEFAQEHQJuIvcDm3Sh0+ZOE5hj7jCBas2xOCqYiG6+bWWieoxRrAwEICcKB
     BBgWCgAzApsMBYJcB+a/BYkFn6YAFiEEwIdEeKZcVAofc3K0oi6tYU0RdEoJEKIu
     rWFNEXRKAgsJAADx4wD/VrXZ7I/hBC37lzhyVEcCaHcorVXVn8ACCiyRmgmNbY4A
     /1lJmQJoDlpYlx3BAJ6RYuXRJoyU5KpcBf5afBPn8ncB
     =MHBq
     -----END PGP PUBLIC KEY BLOCK-----";

fn main() -> openpgp::Result<()> {
    let cert = openpgp::Cert::from_bytes(KEY.as_bytes())?;

    assert_eq!(
        cert.fingerprint().to_hex(),
        "C0874478A65C540A1F7372B4A22EAD614D11744A"
    );

    // Iterate over UserIDs.
    assert_eq!(cert.userids().count(), 1);
    assert_eq!(cert.userids().nth(0).unwrap().to_string(), "Ἀριστοτέλης");

    // Iterate over subkeys.
    assert_eq!(cert.keys().subkeys().count(), 2);
    assert_eq!(
        cert.keys()
            .subkeys()
            .nth(0)
            .unwrap()
            .key()
            .fingerprint()
            .to_hex(),
        "67A48753A380A6B3B7DF7DC5E6C6897A4CEF8924"
    );
    assert_eq!(
        cert.keys()
            .subkeys()
            .nth(1)
            .unwrap()
            .key()
            .fingerprint()
            .to_hex(),
        "185CDAA12723042319E47F67108F2CAF9034356D"
    );

    Ok(())
}
