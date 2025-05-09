/// Indicates the various states of Central.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-CentralState>
pub enum BluetoothEmulationCentralState {
    Absent,
    PoweredOff,
    PoweredOn,
}
/// Indicates the various types of GATT event.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-GATTOperationType>
pub enum BluetoothEmulationGattOperationType {
    Connection,
    Discovery,
}
/// Indicates the various types of characteristic write.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-CharacteristicWriteType>
pub enum BluetoothEmulationCharacteristicWriteType {
    WriteDefaultDeprecated,
    WriteWithResponse,
    WriteWithoutResponse,
}
/// Indicates the various types of characteristic operation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-CharacteristicOperationType>
pub enum BluetoothEmulationCharacteristicOperationType {
    Read,
    Write,
    SubscribeToNotifications,
    UnsubscribeFromNotifications,
}
/// Indicates the various types of descriptor operation.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-DescriptorOperationType>
pub enum BluetoothEmulationDescriptorOperationType {
    Read,
    Write,
}
/// Stores the manufacturer data
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-ManufacturerData>
pub struct BluetoothEmulationManufacturerData {
    pub key: (),
    pub data: (),
}
/// Stores the byte data of the advertisement packet sent by a Bluetooth device.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-ScanRecord>
pub struct BluetoothEmulationScanRecord {
    pub name: (),
    pub uuids: (),
    pub appearance: (),
    pub tx_power: (),
    pub manufacturer_data: (),
}
/// Stores the advertisement packet information that is sent by a Bluetooth device.
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-ScanEntry>
pub struct BluetoothEmulationScanEntry {
    pub device_address: (),
    pub rssi: (),
    pub scan_record: (),
}
/** Describes the properties of a characteristic. This follows Bluetooth Core
Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.*/
/// <https://chromedevtools.github.io/devtools-protocol/tot/BluetoothEmulation/#type-CharacteristicProperties>
pub struct BluetoothEmulationCharacteristicProperties {
    pub broadcast: (),
    pub read: (),
    pub write_without_response: (),
    pub write: (),
    pub notify: (),
    pub indicate: (),
    pub authenticated_signed_writes: (),
    pub extended_properties: (),
}
