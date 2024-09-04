use std::io::Write;
use std::process::Command;
use rusb::{UsbContext};

/// This function executes platform-specific commands to find the drive letter (on Windows)
/// or mount point (on Linux and macOS) of a connected USB device.
/// It returns "None" if no USB device is found.
/// # Returns
///
/// * `Option<String>` - The drive letter or mount point of the USB device if found, otherwise `None`.
pub fn get_usb_drive_path() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("Get-WmiObject Win32_LogicalDisk | Where-Object { $_.DriveType -eq 2 } | Select-Object -ExpandProperty DeviceID")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Split the output by whitespace and get the first drive letter in case there are multiple USB devices
            let mut drive_letters = output_str.split_whitespace();

            if let Some(drive_letter) = drive_letters.next() {
                return Some(drive_letter.to_string());
            }
        } else {
            println!("No USB device found.");
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("lsblk")
            .arg("-o")
            .arg("MOUNTPOINT")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let mounts = String::from_utf8_lossy(&output.stdout);
            for mount in mounts.lines() {
                if mount.contains("/media/") || mount.contains("/mnt/") {
                    return Some(mount.to_string());
                }
            }
        } else {
            println!("No USB device found.");
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("df")
            .arg("-h")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let mounts = String::from_utf8_lossy(&output.stdout);
            for line in mounts.lines() {
                if line.contains("/Volumes/") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(path) = parts.get(parts.len() - 1) {
                        return Some(path.to_string());
                    }
                }
            }
        } else {
            println!("No USB device found.");
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_usb_drive_letter() {
        let drive_letter = get_usb_drive_path();
        assert!(drive_letter.is_some());
        println!("Drive letter: {}", drive_letter.unwrap());
    }
}
