
fn calculate_private_key(p: &u32, q: &u32, e: &u32) -> u32 {
    let totient = (p-1) * (q-1);

    let mut d = 1;
    let mut de = *e;

    loop {
    	d += 1;
    	de += e;
    	if totient < de {
    		de -= totient;
    	}
    	if de == 1 {
	    	break
    	}
    };
    d
}


fn x_power_e_mod_n(x: u32, e: &u32, n: &u32) -> u32 {
	let mut result = 1;
    for _ in 0..*e {
    	result = (result * x) % n;
    }
	result
}


fn encrypt_message(msg: String, e: &u32, n: &u32) -> Vec<u32> {
	let bytes = msg.as_bytes();

    let mut encrypted = vec![];
    for b in bytes {
	    encrypted.push(
	    	x_power_e_mod_n(*b as u32, &e, &n)
	    );
    }
	encrypted
}


fn decrypt_message(bytes: Vec<u32>, d: &u32, n: &u32) -> String {
    let mut decrypted = vec![];
   	for b in bytes {
	    decrypted.push(
	    	x_power_e_mod_n(b, &d, &n) as u8
	    );
	}
   	String::from_utf8(decrypted).expect("Found invalid UTF-8 when decrypting")
}


fn run_demo(p: u32, q: u32, e: u32, msg: String) {
	println!("Running RSA demo...");

	println!("\nProvided message to encrypt:");
   	println!("{:?}", msg);

	println!("\nProvided prime numbers:");
   	println!("p: {:?}", p);
   	println!("q: {:?}", q);

	println!("\nProvided public key:");
   	println!("e: {:?}", e);

    let d = calculate_private_key(&p, &q, &e);

	println!("\nCalculated private key (ssh!):");
   	println!("d: {:?}", d);

    let n = p * q;
    let encrypted = encrypt_message(msg, &e, &n);

	println!("\nEncrypted message:");
   	println!("{:?}", encrypted);

    let msg = decrypt_message(encrypted, &d, &n);

	println!("\nDecrypted message:");
   	println!("{:?}", msg);
}


fn main() {
    let p = 61;
    let q = 53;
    let e = 17;

    /* Currently only UTF-8 is valid */
    let msg = "Secret Message".to_string();

    /* Demo only encrypts / decrypts with a block size of 1 */
    /* I.e. each character is handled separately */
    run_demo(p, q, e, msg);
}
