use std::{fs, os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt}, path::Path};

pub fn get_total_blocks(dir: &Path, flag_a: bool) -> std::io::Result<u64> {
    let mut total_blocks = if flag_a {
        let current = dir.join(Path::new("."));
        let meta = current.metadata()?;
        let current2 = dir.join(Path::new(".."));
        let meta2 = current2.metadata()?;
        meta.blocks() + meta2.blocks()
    } else {
        0
    };

    for entry in dir.read_dir()? {
        let entry = entry?;
        if !flag_a
            && entry
                .file_name()
                .to_string_lossy()
                .to_string()
                .starts_with(".")
        {
            continue;
        }
        let metadata = entry.metadata()?;
        total_blocks += metadata.blocks();
    }

    Ok(total_blocks / 2)
}

pub fn is_executable(path: &Path) -> std::io::Result<bool> {
    let metadata = path.metadata()?;
    let mode = metadata.permissions().mode();

    // Check owner, group, or others execute bit
    const OWNER_X: u32 = 0o100;
    const GROUP_X: u32 = 0o010;
    const OTHER_X: u32 = 0o001;

    Ok((mode & (OWNER_X | GROUP_X | OTHER_X)) != 0)
}

pub fn permissions(path: &Path) -> std::io::Result<String> {
    let metadata = fs::symlink_metadata(path)?;
    let mode = metadata.mode();
    let file_type = metadata.file_type();

    // Determine the file type character
    let file_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_block_device() {
        'b'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_socket() {
        's'
    } else {
        '-'
    };

    // Permission characters
    let mut perms = String::new();
    perms.push(file_char);

    // Special mode bits
    let setuid = mode & 0o4000 != 0;
    let setgid = mode & 0o2000 != 0;
    let sticky = mode & 0o1000 != 0;

    for i in 0..3 {
        let shift = (2 - i) * 3;
        let bits = (mode >> shift) & 0b111;
        let r = if bits & 0o4 != 0 { 'r' } else { '-' };
        let w = if bits & 0o2 != 0 { 'w' } else { '-' };
        let mut x = if bits & 0o1 != 0 { 'x' } else { '-' };

        // Apply special bits
        if i == 0 && setuid {
            x = if x == 'x' { 's' } else { 'S' };
        } else if i == 1 && setgid {
            x = if x == 'x' { 's' } else { 'S' };
        } else if i == 2 && sticky {
            x = if x == 'x' { 't' } else { 'T' };
        }

        perms.push(r);
        perms.push(w);
        perms.push(x);
    }

    // Optional: Check for extended attributes (simplified approach)
    // This is a basic implementation - you might want to use a more robust method
    // depending on your specific needs
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    if let Ok(path_cstring) = CString::new(path.as_os_str().as_bytes()) {
        // Simple check for extended attributes
        println!("{path_cstring:?}");
        unsafe {
            let result = libc::listxattr(path_cstring.as_ptr(), std::ptr::null_mut(), 0);
            if result > 0 {
                perms.push('+');
            }
        }
    }

    Ok(perms)
}