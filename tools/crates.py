## Download crates
## python3 scripts/crates.py download --category "Data structures" --top-n 10
##
##

import argparse
from pathlib import Path
import tarfile
from urllib.error import HTTPError, URLError
from urllib.request import urlopen

DB_DUMP_URL = "https://static.crates.io/db-dump.tar.gz"

class Crate:
    def __init__(self, name: str, repository: str, version: str):
        self.name = name
        self.repository = repository
        self.version = version

def get_crates_from_db_dump(db_dump_dir: str, top_n: int = 1000, category: str = None) -> list[Crate]:
    import pandas as pd

    crates_csv_path = f"{db_dump_dir}/data/crates.csv"
    versions_csv_path = f"{db_dump_dir}/data/versions.csv"

    # Step 1: Read the CSV files
    crates_df = pd.read_csv(crates_csv_path, usecols=['name', 'repository', 'id'])
    versions_df = pd.read_csv(versions_csv_path, parse_dates=['created_at'], usecols=['crate_id', 'yanked', 'created_at', 'num'])
    crate_downloads_csv_path = f"{db_dump_dir}/data/crate_downloads.csv"
    crate_downloads_df = pd.read_csv(crate_downloads_csv_path, usecols=['crate_id', 'downloads'])
    crates_df = pd.merge(crates_df, crate_downloads_df, left_on='id', right_on='crate_id').drop(columns=['crate_id'])

    # Step 2: Filter out rows where the repository is empty or yanked column is 't'
    crates_df = crates_df[crates_df['repository'].notna()]
    versions_df = versions_df[versions_df['yanked'] == 'f']

    # Get the latest version of each crate based on the created_at column
    versions_df = versions_df.sort_values('created_at').groupby('crate_id', as_index=False).last()

    merged_df = pd.merge(crates_df, versions_df, left_on='id', right_on='crate_id')
    
    categories_csv_path = f"{db_dump_dir}/data/categories.csv"
    crates_categories_csv_path = f"{db_dump_dir}/data/crates_categories.csv"
    categories_df = pd.read_csv(categories_csv_path, usecols=['id', 'category'])
    crates_categories_df = pd.read_csv(crates_categories_csv_path, usecols=['crate_id', 'category_id'])
    merged_df = pd.merge(merged_df, crates_categories_df, left_on='id', right_on='crate_id')
    merged_df = pd.merge(merged_df, categories_df, left_on='category_id', right_on='id', suffixes=('', '_category'))
    if category:
        merged_df = merged_df[merged_df['category'] == category]
    # By descending order of downloads, get the top N crates
    merged_df = merged_df.sort_values('downloads', ascending=False).head(top_n) 
    crates = [Crate(row['name'], row['repository'], row['num']) for _, row in merged_df.iterrows()]
    return crates


def main():
    parser = argparse.ArgumentParser(prog='./crates.py', description='Download and extract crates from crates.io')
    parser.add_argument('--db-dump-dir', type=str, default='.local/db-dump', help='Directory to store database dumps (tar -xvf db-dump.tar.gz)')
    subparsers = parser.add_subparsers(required=True, dest='command')
    db_dump_parser = subparsers.add_parser('download-db-dump', help='Download and extract the crates.io database dump')
    db_dump_parser.add_argument('--archive-path', type=str, default='.local/db-dump.tar.gz', help='File path to save the database dump archive')
    db_dump_parser.add_argument('--db-dump-dir', type=str, default=None, help='Directory to extract the database dump')
    db_dump_parser.add_argument('--force', action='store_true', help='Re-download the archive even if it already exists')
    db_dump_parser.set_defaults(func=command_download_db_dump)
    download_parser = subparsers.add_parser('download', help='Download crates from crates.io')
    download_parser.add_argument('--top-n', type=int, default=1000, help='Number of top crates to download')
    download_parser.add_argument('--category', type=str, help='Category of crates to download')
    download_parser.add_argument('--temp-dir', type=str, default='.local/crates', help='Temporary directory for downloading and extracting crates')
    download_parser.add_argument('--output-dir', type=str, default='crates', help='Directory to save downloaded crates')
    download_parser.set_defaults(func=command_download)
    args = parser.parse_args()

    args.func(args)


def command_download(args: argparse.Namespace) -> None:
    crates = get_crates_from_db_dump(args.db_dump_dir, top_n=args.top_n, category=args.category)
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    temp_dir = Path(args.temp_dir)
    temp_dir.mkdir(parents=True, exist_ok=True)

    for crate in crates:
        try:
            dst = download_crate(crate, temp_dir)
            extract_tar_gz(dst, output_dir)
        except Exception as e:
            print(f"Error downloading {crate.name} v{crate.version}: {e}")


def command_download_db_dump(args: argparse.Namespace) -> None:
    archive_path = Path(args.archive_path)
    db_dump_dir = Path(args.db_dump_dir or '.local/db-dump')

    download_file(DB_DUMP_URL, archive_path, force=args.force)
    safe_extract_tar_gz(archive_path, db_dump_dir)
    print(f"Database dump extracted to: {db_dump_dir}")


def download_file(url: str, dst: Path, force: bool = False) -> Path:
    if dst.exists() and not force:
        print(f"Using existing archive: {dst}")
        return dst

    dst.parent.mkdir(parents=True, exist_ok=True)
    try:
        with urlopen(url) as response, open(dst, 'wb') as file:
            while True:
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                file.write(chunk)
    except HTTPError as e:
        raise Exception(f"status code: {e.code}, error: {e.reason}") from e
    except URLError as e:
        raise Exception(f"failed to download {url}: {e.reason}") from e
    print(f"Downloaded: {dst}")
    return dst


def safe_extract_tar_gz(file_path: Path, output_directory: Path) -> None:
    output_directory.mkdir(parents=True, exist_ok=True)
    output_directory = output_directory.resolve()

    with tarfile.open(file_path, 'r:gz') as file:
        for member in file.getmembers():
            member_path = (output_directory / member.name).resolve()
            if output_directory != member_path and output_directory not in member_path.parents:
                raise Exception(f"refusing to extract path outside {output_directory}: {member.name}")
        file.extractall(path=output_directory)



def download_crate(crate: Crate, output_dir: Path) -> Path:
    import requests

    download_url = f"https://crates.io/api/v1/crates/{crate.name}/{crate.version}/download"
    dst = output_dir / f"{crate.name}_{crate.version}.crate"
    if dst.exists():
        return dst
    response = requests.get(download_url)
    if response.status_code == 200:
        with open(dst, 'wb') as file:
            file.write(response.content)
        print(f"Downloaded: {dst}")
    else:
        raise Exception(f"status code: {response.status_code}, error: {response.text}")
    return dst
    
def extract_tar_gz(file_path: Path, output_directory: Path) -> None:
    try:
        with tarfile.open(file_path, 'r:gz') as file:
            members = file.getmembers()
            if members:
                top_folder_name = members[0].name.split('/')[0]
                dst = output_directory / top_folder_name
                if dst.exists():
                    # print(f"skip {file_path} since {dst} existed")
                    return
            else:
                print("The tar file is empty.")
            file.extractall(path=output_directory)
    except Exception as e:
        print(f"Failed to extract {file_path}. Error: {e}")

if __name__ == "__main__":
    main()
