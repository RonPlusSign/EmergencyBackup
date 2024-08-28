use std::io::Write;
use std::process::Command;
use rusb::{Context, Device, UsbContext};
use std::fs;

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
                        if let Some(drive_letter) = get_usb_drive_letter() {
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

pub fn get_usb_drive_letter() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("Get-WmiObject Win32_LogicalDisk | Where-Object { $_.DriveType -eq 2 } | Select-Object -ExpandProperty DeviceID")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let drive_letter = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !drive_letter.is_empty() {
                return Some(drive_letter);
            }
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
        }
    }

    None
}

fn list_usb_devices() -> Result<(), rusb::Error> {
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }

    Ok(())
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
    fn test_list_usb_devices() {
        let result = list_usb_devices();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_usb_drive_letter() {
        let drive_letter = get_usb_drive_letter();
        assert!(drive_letter.is_some());
        println!("Drive letter: {}", drive_letter.unwrap());
    }
}
