use gtk::prelude::*;
use gtk::{Box, Image, Label, ListBox, ListBoxRow, Orientation};
use nmrs::models;

pub fn bluetooth_devices_view(devices: &[models::BluetoothDevice]) -> ListBox {
    let list = ListBox::new();

    for device in devices {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 6);

        row.add_css_class("network-selection");

        if device.state == models::DeviceState::Activated {
            row.add_css_class("connected");
        }

        let name = device
            .alias
            .as_deref()
            .or(device.name.as_deref())
            .unwrap_or("Unknown device");

        let display_name = format!("{} ({})", name, device.bdaddr);
        hbox.append(&Label::new(Some(&display_name)));

        if device.state == models::DeviceState::Activated {
            let connected_label = Label::new(Some("Connected"));
            connected_label.add_css_class("connected-label");
            hbox.append(&connected_label);
        }

        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        hbox.append(&spacer);

        let state_text = match device.state {
            models::DeviceState::Activated => Some("Connected"),
            models::DeviceState::Disconnected => Some("Disconnected"),
            models::DeviceState::Unavailable => Some("Unavailable"),
            models::DeviceState::Failed => Some("Failed"),
            _ => None,
        };

        if let Some(text) = state_text {
            let state_label = Label::new(Some(text));
            state_label.add_css_class(match device.state {
                models::DeviceState::Activated => "network-good",
                models::DeviceState::Unavailable
                | models::DeviceState::Disconnected
                | models::DeviceState::Failed => "network-poor",
                _ => "network-okay",
            });
            hbox.append(&state_label);
        }

        let icon = Image::from_icon_name("bluetooth-active-symbolic");
        icon.add_css_class("wired-icon");
        hbox.append(&icon);

        row.set_child(Some(&hbox));
        list.append(&row);
    }

    list
}
