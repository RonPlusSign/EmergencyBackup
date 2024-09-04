use std::io::Write;
use std::process::Command;
use rusb::{Context, Device, UsbContext};

/// This function searches for a connected USB device and returns the device and the drive letter (on Windows)
/// TODO: maybe not useful, check if it can be removed
pub fn find_usb_device() -> Option<(Device<Context>, String)> {
    let context = Context::new().unwrap();
    let mut device: Option<(Device<Context>, String)> = None;

    for dev in context.devices().unwrap().iter() {
        let device_desc = dev.clone().device_descriptor().unwrap();

        for i in 0..device_desc.num_configurations() {
            let config_desc = dev.clone().config_descriptor(i).unwrap();

            for interface in config_desc.interfaces() {
                for interface_desc in interface.descriptors() {
                    // Check if the device is a mass storage device
                    if interface_desc.class_code() == 0x08 {
                        println!("Dispositivo di memorizzazione di massa trovato!");

                        // Find the drive letter of the USB device
                        if let Some(drive_letter) = get_usb_drive_path() {
                            device = Some((dev.clone(), drive_letter));
                            break;
                        }
                    }
                }
            }
        }

        if device.is_some() {
            break;
        }
    }

    device
}

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
    fn test_find_usb_device() {
        let device = find_usb_device();
        assert!(device.is_some());
        let device_desc = device.as_ref().unwrap().0.device_descriptor().unwrap();
        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.clone().unwrap().0.bus_number(),
            device.clone().unwrap().0.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }

    #[test]
    fn test_get_usb_drive_letter() {
        let drive_letter = get_usb_drive_path();
        assert!(drive_letter.is_some());
        println!("Drive letter: {}", drive_letter.unwrap());
    }
}
