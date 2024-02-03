use image::{GenericImageView, ImageBuffer, RgbaImage};
use std::io;

fn main() {
    let mut image_name = String::new();
    let mut password = String::new();
    let mut choice = String::new();

    println!("Choose what to do:\n1. Encode a password\n2. Decode a password");
    io::stdin().read_line(&mut choice).unwrap();
    println!("Insert image name:");
    io::stdin().read_line(&mut image_name).unwrap();
    let image_name = image_name.trim().to_string();

    if choice.trim().to_string().parse::<i32>().unwrap() == 1 {
        println!("Insert password to save into the image:");
        io::stdin().read_line(&mut password).unwrap();

        let password = password.trim().to_string();

        encode_password(image_name, password);
    } else {
        decode_password(image_name);
    }
}

fn decode_password(image_name: String) {
    let mut folder: String = "out/".to_string();
    folder.push_str(&image_name);
    let img = image::open(folder).unwrap();
    let mut length_to_decode = 0;
    let mut bits_to_decode: Vec<u8> = vec![];
    let mut bits_of_length: Vec<u8> = vec![];
    let mut counter = 0;

    for (_, _, pixel) in img.pixels() {
        if length_to_decode == 0 || counter < length_to_decode {
            bits_to_decode.push(pixel[0].to_be_bytes()[0] >> 0 & 1);
        }
        if length_to_decode == 0 || counter <= length_to_decode {
            counter = counter + 1;
        }
        if length_to_decode == 0 || counter < length_to_decode {
            bits_to_decode.push(pixel[1].to_be_bytes()[0] >> 0 & 1);
        }
        if length_to_decode == 0 || counter <= length_to_decode {
            counter = counter + 1;
        }
        if counter == 8 {
            for i in 0..8 {
                bits_of_length.push(bits_to_decode[i]);
            }
            length_to_decode = convert_binary(bits_of_length.clone())[0] * 8;
            bits_to_decode = vec![];
            counter = 0;
        }
        if length_to_decode == 0 || counter < length_to_decode {
            bits_to_decode.push(pixel[2].to_be_bytes()[0] >> 0 & 1);
        }
        if length_to_decode == 0 || counter <= length_to_decode {
            counter = counter + 1;
        }
    }

    let mut decoded_bits = convert_binary(bits_to_decode);
    decoded_bits.reverse();

    let str = String::from_utf8(decoded_bits);

    println!("Password: {:?}", str);
}

fn encode_password(image_name: String, password: String) {
    let mut folder: String = "img/".to_string();
    folder.push_str(&image_name);

    let bytes_of_password = password.as_bytes().to_vec();
    let length_of_password = bytes_of_password.len().to_be_bytes();
    let mut bits_to_encode: Vec<u8> = vec![];

    for byte in bytes_of_password {
        for bits_index in 0..8 {
            bits_to_encode.push(byte >> bits_index & 1);
        }
    }

    for bits_index in 0..8 {
        bits_to_encode.push(length_of_password[7] >> bits_index & 1);
    }

    bits_to_encode.reverse();

    let img = image::open(folder).unwrap();

    let (width, height) = img.dimensions();
    let mut out: RgbaImage = ImageBuffer::new(width, height);

    let mut counter = 0;
    let length_to_encode = bits_to_encode.len();

    for (x, y, mut pixel) in img.pixels() {
        if counter < length_to_encode
            && pixel[0].to_be_bytes()[0] >> 0 & 1 != bits_to_encode[counter]
        {
            pixel[0] ^= 0b0000_0001;
        }
        if counter <= length_to_encode {
            counter = counter + 1;
        }
        if counter < length_to_encode
            && pixel[1].to_be_bytes()[0] >> 0 & 1 != bits_to_encode[counter]
        {
            pixel[1] ^= 0b0000_0001;
        }
        if counter <= length_to_encode {
            counter = counter + 1;
        }
        if counter < length_to_encode
            && pixel[2].to_be_bytes()[0] >> 0 & 1 != bits_to_encode[counter]
        {
            pixel[2] ^= 0b0000_0001;
        }
        if counter <= length_to_encode {
            counter = counter + 1;
        }

        out.put_pixel(x, y, pixel);
    }

    let mut out_folder: String = "out/".to_string();
    out_folder.push_str(&image_name);

    out.save(out_folder).unwrap();
}

fn convert_binary(bits_to_decode: Vec<u8>) -> Vec<u8> {
    let values = [128, 64, 32, 16, 8, 4, 2, 1];
    let mut results: Vec<u8> = vec![];
    let mut counter = 0;

    for _ in 0..bits_to_decode.len() / 8 {
        let mut result = 0;
        for i in 0..8 {
            result += bits_to_decode[counter] * values[i];
            counter = counter + 1;
        }
        results.push(result);
    }

    return results;
}
