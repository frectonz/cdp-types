use crate::common::*;
/// Indicates the various states of Central.
pub enum CentralState {
    Absent,
    PoweredOff,
    PoweredOn,
}
/// Indicates the various types of GATT event.
pub enum GattOperationType {
    Connection,
    Discovery,
}
/// Indicates the various types of characteristic write.
pub enum CharacteristicWriteType {
    WriteDefaultDeprecated,
    WriteWithResponse,
    WriteWithoutResponse,
}
/// Indicates the various types of characteristic operation.
pub enum CharacteristicOperationType {
    Read,
    Write,
    SubscribeToNotifications,
    UnsubscribeFromNotifications,
}
/// Indicates the various types of descriptor operation.
pub enum DescriptorOperationType {
    Read,
    Write,
}
/// Stores the manufacturer data
pub struct ManufacturerData {
    pub key: i64,
    pub data: String,
}
/// Stores the byte data of the advertisement packet sent by a Bluetooth device.
pub struct ScanRecord {
    pub name: String,
    pub uuids: Vec<String>,
    pub appearance: i64,
    pub tx_power: i64,
    pub manufacturer_data: Vec<ManufacturerData>,
}
/// Stores the advertisement packet information that is sent by a Bluetooth device.
pub struct ScanEntry {
    pub device_address: String,
    pub rssi: i64,
    pub scan_record: Box<ScanRecord>,
}
/** Describes the properties of a characteristic. This follows Bluetooth Core
Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.*/
pub struct CharacteristicProperties {
    pub broadcast: bool,
    pub read: bool,
    pub write_without_response: bool,
    pub write: bool,
    pub notify: bool,
    pub indicate: bool,
    pub authenticated_signed_writes: bool,
    pub extended_properties: bool,
}
pub type BluetoothEmulationEnable = ();
pub type BluetoothEmulationSetSimulatedCentralState = ();
pub type BluetoothEmulationDisable = ();
pub type BluetoothEmulationSimulatePreconnectedPeripheral = ();
pub type BluetoothEmulationSimulateAdvertisement = ();
pub type BluetoothEmulationSimulateGattOperationResponse = ();
pub type BluetoothEmulationSimulateCharacteristicOperationResponse = ();
pub type BluetoothEmulationSimulateDescriptorOperationResponse = ();
pub type BluetoothEmulationAddService = ();
pub type BluetoothEmulationRemoveService = ();
pub type BluetoothEmulationAddCharacteristic = ();
pub type BluetoothEmulationRemoveCharacteristic = ();
pub type BluetoothEmulationAddDescriptor = ();
pub type BluetoothEmulationRemoveDescriptor = ();
pub type BluetoothEmulationSimulateGattDisconnection = ();
