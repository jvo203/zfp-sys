#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod zfp {
    use super::*;

    #[test]
    fn compress_array() {
        //based on examples/simple.c
        let nx = 100;
        let ny = 100;
        let nz = 100;

        let mut array: Vec<f64> = vec![0.0; nx * ny * nz];

        for i in 0..nx {
            for j in 0..ny {
                for k in 0..nz {
                    let x = 2.0 * (i as f64) / (nx as f64);
                    let y = 2.0 * (j as f64) / (ny as f64);
                    let z = 2.0 * (k as f64) / (nz as f64);
                    array[i + nx * (j + ny * k)] = (-(x * x + y * y + z * z)).exp();
                }
            }
        }

        println!("original data sample: {:?}", &array[0..nx]);

        //compression
        /* allocate meta data for the 3D array a[nz][ny][nx] */
        let data_type = zfp_type_zfp_type_double;
        let field = unsafe {
            zfp_field_3d(
                array.as_mut_ptr() as *mut std::ffi::c_void,
                data_type,
                nx as usize,
                ny as usize,
                nz as usize,
            )
        };

        /* allocate meta data for a compressed stream */
        let zfp = unsafe { zfp_stream_open(std::ptr::null_mut() as *mut bitstream) };

        /* set compression mode and parameters via one of three functions */
        unsafe { zfp_stream_set_rate(zfp, 8.0, data_type, 3, 0) };
        /*  zfp_stream_set_precision(zfp, precision); */
        //let tolerance = 1.0e-3;
        //unsafe { zfp_stream_set_accuracy(zfp, tolerance) };

        #[cfg(feature = "cuda")]
        {
            let ret = unsafe { zfp_stream_set_execution(zfp, zfp_exec_policy_zfp_exec_cuda) };

            if ret == 0 {
                println!("failed to set the execution policy to zfp_exec_cuda");
                assert!(false);
            }
        }

        /* allocate buffer for compressed data */
        let bufsize = unsafe { zfp_stream_maximum_size(zfp, field) };
        let mut buffer: Vec<u8> = vec![0; bufsize as usize];

        /* associate bit stream with allocated buffer */
        let stream = unsafe { stream_open(buffer.as_mut_ptr() as *mut std::ffi::c_void, bufsize) };
        unsafe {
            zfp_stream_set_bit_stream(zfp, stream);
            zfp_stream_rewind(zfp);
        }

        /* compress array and output compressed stream */
        let zfpsize = unsafe { zfp_compress(zfp, field) };
        if zfpsize == 0 {
            println!("compression failed");
            assert!(false);
        } else {
            let original_size = nx * ny * nz * std::mem::size_of::<f64>();
            let ratio = (original_size as f64) / (zfpsize as f64);

            println!(
                "bufsize: {} bytes, original size: {} bytes, compressed size: {} bytes, ratio: {}",
                bufsize, original_size, zfpsize, ratio
            );
        }

        /* rewind compressed stream and decompress array */
        unsafe { zfp_stream_rewind(zfp) };
        let ret = unsafe { zfp_decompress(zfp, field) };
        if ret == 0 {
            println!("decompression failed");
            assert!(false);
        } else {
            println!("ret: {}", ret);
        }

        println!("decompressed data sample: {:?}", &array[0..nx]);

        /* clean up */
        unsafe {
            zfp_field_free(field);
            zfp_stream_close(zfp);
            stream_close(stream);
        }

        assert!(true);
    }
}
