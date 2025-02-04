use std::path::{Path, PathBuf};
use log::warn;
use tokio::{fs::{create_dir_all, File, OpenOptions}, io::{AsyncReadExt, BufReader}};
use async_zip::{base::read::seek::ZipFileReader, error::ZipError, tokio::write::ZipFileWriter, Compression, StoredZipEntry, ZipEntryBuilder};
use sanitize_filename::sanitize;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Returns a relative path without reserved names, redundant separators, ".", or "..".
fn sanitize_file_path(path: &str) -> PathBuf {
  path.replace('\\', "/")
    .split('/')
    .map(sanitize)
    .collect()
}

fn get_root_of_zip(file: &StoredZipEntry) -> String {
  let file_path = sanitize_file_path(file.filename().as_str().unwrap());

  let mut components = file_path.components();

  let first = components.next();
  if first == Some(std::path::Component::CurDir) {
    // Its relative, need to grab the next path.
    return components.next().unwrap().as_os_str().to_str().unwrap().to_string();
  } else {
    // Its an absolute path.
    return first.unwrap().as_os_str().to_str().unwrap().to_string();
  }
}

/// Extracts everything from the ZIP archive to the output directory.
pub async fn unpack_zip(archive: File, out_dir: &Path) -> Result<PathBuf, ZipError> {
  let archive = BufReader::new(archive).compat();
  let mut reader = ZipFileReader::new(archive).await?;

  let first_file: &StoredZipEntry = reader.file().entries().get(0).unwrap();
  let zip_root_folder = get_root_of_zip(first_file);

  for index in 0..reader.file().entries().len() {
    let entry = reader.file().entries().get(index).unwrap();
    let path = out_dir.join(sanitize_file_path(entry.filename().as_str().unwrap()));
    let entry_is_dir = entry.dir().unwrap();

    let mut entry_reader = reader.reader_without_entry(index).await?;

    if entry_is_dir {
      if !path.exists() {
        create_dir_all(&path).await.expect("Failed to create extracted directory");
      }
    } else {
      let parent = path.parent().expect("A file entry should have parent directories");
      if !parent.is_dir() {
        create_dir_all(parent).await.expect("Failed to create parent directories");
      }

      let writer = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
        .expect("Failed to create extracted file");

      futures_util::io::copy(&mut entry_reader, &mut writer.compat_write())
        .await
        .expect("Failed to copy to extracted file");
    }
  }

  return Ok(out_dir.join(zip_root_folder));
}


/// Writes an entry to a zip.
async fn write_entry(filename: &str, input_path: &Path, writer: &mut ZipFileWriter<File>) -> Result<(), ZipError> {
  let mut input_file = File::open(input_path).await?;
  let input_file_size = input_file.metadata().await?.len() as usize;

  let mut buffer = Vec::with_capacity(input_file_size);
  input_file.read_to_end(&mut buffer).await?;

  let builder = ZipEntryBuilder::new(filename.into(), Compression::Deflate);
  writer.write_entry_whole(builder, &buffer).await?;

  return Ok(());
}

/// Gets all of the files in the directory
async fn walk_dir(dir: PathBuf) -> Result<Vec<PathBuf>, tokio::io::Error> {
  let mut dirs = vec![dir];
  let mut files = vec![];

  while !dirs.is_empty() {
    let mut dir_iter = tokio::fs::read_dir(dirs.remove(0)).await?;

    while let Some(entry) = dir_iter.next_entry().await? {
      let entry_path_buf = entry.path();

      if entry_path_buf.is_dir() {
        dirs.push(entry_path_buf);
      } else {
        files.push(entry_path_buf);
      }
    }
  }

  return Ok(files);
}


/// Compresses the provided directory into a ZIP archive.
pub async fn pack_zip(dir: &Path) -> Result<PathBuf, ZipError> {
  let parent = dir.parent().unwrap().to_path_buf();
  let output_path = parent.join(format!("{}.zip", dir.file_name().unwrap().to_str().unwrap()));
  let output_file = File::create(&output_path).await?;
  let mut output_writer = ZipFileWriter::with_tokio(output_file);


  let entries = walk_dir(dir.into()).await?;
  let input_dir_str = dir.as_os_str().to_str().unwrap();

  for entry_path_buf in entries {
    let entry_path = entry_path_buf.as_path();
    let entry_str = entry_path.as_os_str().to_str().unwrap();

    if !entry_str.starts_with(input_dir_str) {
      warn!("Directory file path does not start with base input directory path.");
    }

    write_entry(entry_str, entry_path, &mut output_writer).await?;
  }


  output_writer.close().await?;

  return Ok(output_path);
}
