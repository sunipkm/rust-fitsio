pub use sys::*;
use libc::c_int;

pub unsafe fn fits_close_file(fptr: *mut fitsfile, status: *mut c_int) -> c_int {
    ffclos(fptr, status)
}

/*
fits_copy_hdu ffcopy
fits_create_img ffcrim
fits_create_tbl ffcrtb
fits_delete_col ffdcol
fits_delete_hdu ffdhdu
fits_file_mode ffflmd
fits_get_bcolparms ffgbcl
fits_get_col_display_width ffgcdw
fits_get_colnum ffgcno
fits_read_col_str ffgcvs
fits_get_hdu_num ffghdn
fits_get_hdu_type ffghdt
fits_get_img_dim ffgidm
fits_get_img_equivtype ffgiet
fits_get_img_size ffgisz
fits_read_key_str ffgkys
fits_get_num_cols ffgncl
fits_get_num_rows ffgnrw
fits_read_img ffgpv
fits_read_subset ffgsv
fits_insert_col fficol
fits_create_file ffinit
fits_movabs_hdu ffmahd
fits_movnam_hdu ffmnhd
fits_open_file ffopen
fits_write_col ffpcl
fits_write_col_str ffpcls
fits_write_imghdr ffphps
fits_write_key_lng ffpkyj
fits_write_key_str ffpkys
fits_write_img ffppr
fits_write_subset ffpss
fits_resize_img ffrsim
fits_get_num_hdus ffthdu
*/
