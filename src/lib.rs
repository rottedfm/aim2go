use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn create_directory(dir_name: &str) -> io::Result<()> {
	let base_path = Path::new(dir_name);

	// Create the main directory
	match fs::create_dir(base_path) {
		Ok(_) => println!("Directory '{}' created successfully!", dir_name),
		Err(e) => {
			eprintln!("Failed to create directory '{}': {}", dir_name, e);
			return Err(e);	
		}
	}
	
	// Create 'dataset' and 'model' subdirectories
	for sub_dir in [ "dataset", "model"].iter() {
		let sub_dir_path = base_path.join(sub_dir);
		match fs::create_dir(&sub_dir_path) {
			Ok(_) => println!("Subdirectory '{}' created successfully!", sub_dir_path.display()),
			Err(e) => {
				println!("Failed to create subdirectory '{}': {}", sub_dir_path.display(), e);
				return Err(e);
			}
		}
	}

	// Create a blank 'config.yaml'
	let config_path = base_path.join("config.yaml");
	match fs::File::create(&config_path) {
		Ok(mut file) => {
			println!("File '{}' create successfully!", config_path.display());
			let _ = file.write_all(b"# Config");
		}
		Err(e) => {
			eprintln!("Failed to create file '{}': {}", config_path.display(), e);
			return Err(e);
		}
	}

	Ok(())

}

pub fn check_requirements(dir_name: &str) -> bool {
	let base_path = Path::new(dir_name);

	let dataset_path = base_path.join("dataset");
	let model_path = base_path.join("model");
	let config_path = base_path.join("config.yaml");
        
        base_path.is_dir() && model_path.is_dir() && dataset_path.is_dir() && config_path.is_file()
}

pub fn check_and_delete_directory(dir_name: &str) -> io::Result<()> {
	let base_path = Path::new(dir_name);
	
	let dataset_path = base_path.join("dataset");
	let model_path = base_path.join("model");
	let config_path = base_path.join("config.yaml");

	if dataset_path.exists() && model_path.exists() && config_path.exists() {
		println!("All required items found in '{}'. Deleting the parent directory...", dir_name);
		fs::remove_dir_all(base_path)?;
		println!("Directory '{}' deleted successfully!", dir_name);
	} else {
		println!("Directory '{}' does not contain all required items. No action taken.", dir_name);
	}

	Ok(())
}