/* Reading unsupported data types throws error
 * See issue 35: https://github.com/simonrw/rust-fitsio/issues/35
 *
 *
 */
use fitsio::hdu::HduInfo;
use fitsio::images::ImageType;
use fitsio::FitsFile;

#[test]
fn test_ushort_reading() {
    let filename = "../testdata/ushort.fits";
    let mut f = FitsFile::open(filename).unwrap();
    let hdu = f.hdu(0).unwrap();
    match hdu.info {
        HduInfo::ImageInfo { image_type, .. } => assert_eq!(image_type, ImageType::UnsignedShort),
        _ => panic!("Invalid hdu type, should be image with short integers"),
    }
}
