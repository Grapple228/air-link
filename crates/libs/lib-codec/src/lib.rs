// region:    --- Modules

mod error;

use bincode::Decode;
use bincode::Encode;
pub use error::{Error, Result};

// endregion: --- Modules

pub fn encode<D: Encode>(data: &D) -> Result<Vec<u8>> {
    bincode::encode_to_vec(data, bincode::config::standard()).map_err(|_| Error::Encode)
}

pub fn encode_to_stack<D: Encode>(data: &D, buf: &mut [u8]) -> Result<usize> {
    let mut writer = std::io::Cursor::new(buf);
    bincode::encode_into_std_write(data, &mut writer, bincode::config::standard())
        .map_err(|_| Error::Encode)?;
    Ok(writer.position() as usize)
}

pub fn decode<D: Decode<()>>(data: &[u8]) -> Result<D> {
    let (decoded, _) =
        bincode::decode_from_slice(data, bincode::config::standard()).map_err(|_| Error::Decode)?;

    Ok(decoded)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    const FX_RESULT: [u8; 31] = [
        1, 4, 110, 97, 109, 101, 3, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0,
        0, 0, 8, 64,
    ];

    #[derive(Debug, Encode, Decode, PartialEq)]
    struct Data {
        id: u32,
        name: String,
        values: Vec<f64>,
    }

    fn get_data(id: u32) -> Data {
        Data {
            id,
            name: "name".to_string(),
            values: vec![1.0, 2.0, 3.0],
        }
    }

    #[test]
    fn test_encode() -> Result<()> {
        let data = get_data(1);
        let encoded = encode(&data)?;

        assert_eq!(encoded, FX_RESULT);

        Ok(())
    }

    #[test]
    fn test_decode() -> Result<()> {
        let fx_data = get_data(1);
        let decoded = decode::<Data>(&FX_RESULT)?;

        assert_eq!(fx_data, decoded);

        Ok(())
    }

    #[test]
    fn test_encode_to_stack() -> Result<()> {
        let data = get_data(1);
        let mut buf = [0u8; 1024];
        let len = encode_to_stack(&data, &mut buf)?;

        assert_eq!(&buf[..len], FX_RESULT);
        assert_eq!(len, FX_RESULT.len());
        Ok(())
    }

    #[test]
    fn test_speed_compare() -> Result<()> {
        use std::time::Instant;

        // Маленькая команда для теста
        #[derive(Debug, Encode, Decode, PartialEq)]
        struct SmallCommand {
            x: i32,
            y: i32,
            button: u8,
        }

        let command = SmallCommand {
            x: 100,
            y: 200,
            button: 1,
        };

        let iterations = 100_000;

        // ТЕСТ 1: encode (Vec с аллокацией)
        let vec_start = Instant::now();
        for _ in 0..iterations {
            let encoded = encode(&command)?;
            assert!(!encoded.is_empty());
        }
        let vec_duration = vec_start.elapsed();

        // ТЕСТ 2: encode_to_stack (стековый буфер)
        let stack_start = Instant::now();
        for _ in 0..iterations {
            let mut buf = [0u8; 128];
            let len = encode_to_stack(&command, &mut buf)?;
            assert!(len > 0);
        }
        let stack_duration = stack_start.elapsed();

        // ТЕСТ 3: decode (из Vec)
        let encoded_data = encode(&command)?;
        let decode_start = Instant::now();
        for _ in 0..iterations {
            let decoded: SmallCommand = decode(&encoded_data)?;
            assert_eq!(decoded.x, 100);
        }
        let decode_duration = decode_start.elapsed();

        println!(
            "\n========== SPEED TEST ({} iterations) ==========",
            iterations
        );
        println!("encode (Vec):           {:?}", vec_duration);
        println!("encode_to_stack:        {:?}", stack_duration);
        println!("decode:                 {:?}", decode_duration);
        println!("\n========== RATIOS ==========");
        println!(
            "Stack vs Vec encode:    {:.2}x faster",
            vec_duration.as_secs_f64() / stack_duration.as_secs_f64()
        );
        println!(
            "Encode vs Decode:       {:.2}x",
            decode_duration.as_secs_f64() / vec_duration.as_secs_f64()
        );

        Ok(())
    }

    #[test]
    fn test_stack_overflow() -> Result<()> {
        // Создаём данные, которые не влезут в 256 байт
        let large_data = Data {
            id: 1,
            name: "x".repeat(1000),
            values: vec![1.0; 100],
        };

        let mut buf = [0u8; 256];
        let result = encode_to_stack(&large_data, &mut buf);

        assert!(
            result.is_err(),
            "Should fail - data too large for stack buffer"
        );
        println!("Stack overflow test passed: data didn't fit");

        // Проверяем что с большим буфером работает
        let mut large_buf = vec![0u8; 10000];
        let len = encode_to_stack(&large_data, &mut large_buf)?;
        assert!(len > 0);
        println!(
            "Large data encoded into {} bytes buffer, used {} bytes",
            large_buf.len(),
            len
        );

        Ok(())
    }
}

// endregion: --- Tests
