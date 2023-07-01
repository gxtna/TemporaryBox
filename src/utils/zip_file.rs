use super::nanoid;
use anyhow::{Ok, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Seek};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

pub fn zip_file(map: HashMap<String, Vec<u8>>) -> Result<Vec<u8>> {
    let src_path = "/home/temp/zip_files/";
    for (file_name, file_vec) in map {
        let path = format!("{}{}", src_path, file_name);
        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .write(true)
            .create(true)
            .open(path.clone())?;
        file.write_all(&file_vec)?;
    }
    let target = format!("/home/temp/temp-{}.zip", nanoid::nano_id());
    compress_dir(Path::new(src_path), Path::new(&target));
    let zip_vec = fs::read(target.clone()).unwrap();
    let dir = fs::read_dir(src_path).unwrap();
    for data in dir {
        let x = data.unwrap().file_name().into_string().unwrap();
        let data_path = format!("{}{}", src_path, x);
        let _ = fs::remove_file(data_path).unwrap();
    }
    let _ = fs::remove_file(target).unwrap();
    Ok(zip_vec)
}

/// 压缩文件夹
/// test文件夹下有a.jpg和b.txt 两个文件
/// 压缩成test.zip文件
fn compress_dir(src_dir: &Path, target: &Path) {
    let zip_file = std::fs::File::create(target).unwrap();
    let dir = WalkDir::new(src_dir);
    let _ = zip_dir(
        &mut dir.into_iter().filter_map(|e| e.ok()),
        src_dir.to_str().unwrap(),
        zip_file,
    );
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2) //直接用了bzip2压缩方式，其它参看枚举
        .unix_permissions(0o755); //unix系统权限

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
