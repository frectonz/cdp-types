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
/// Enable the BluetoothEmulation domain.
pub type BluetoothEmulationEnableParams = ();
/// Enable the BluetoothEmulation domain.
pub type BluetoothEmulationEnableResults = ();
/// Set the state of the simulated central.
pub type BluetoothEmulationSetSimulatedCentralStateParams = ();
/// Set the state of the simulated central.
pub type BluetoothEmulationSetSimulatedCentralStateResults = ();
/// Disable the BluetoothEmulation domain.
pub type BluetoothEmulationDisableParams = ();
/// Disable the BluetoothEmulation domain.
pub type BluetoothEmulationDisableResults = ();
/** Simulates a peripheral with |address|, |name| and |knownServiceUuids|
that has already been connected to the system.*/
pub type BluetoothEmulationSimulatePreconnectedPeripheralParams = ();
/** Simulates a peripheral with |address|, |name| and |knownServiceUuids|
that has already been connected to the system.*/
pub type BluetoothEmulationSimulatePreconnectedPeripheralResults = ();
/** Simulates an advertisement packet described in |entry| being received by
the central.*/
pub type BluetoothEmulationSimulateAdvertisementParams = ();
/** Simulates an advertisement packet described in |entry| being received by
the central.*/
pub type BluetoothEmulationSimulateAdvertisementResults = ();
/** Simulates the response code from the peripheral with |address| for a
GATT operation of |type|. The |code| value follows the HCI Error Codes from
Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.*/
pub type BluetoothEmulationSimulateGattOperationResponseParams = ();
/** Simulates the response code from the peripheral with |address| for a
GATT operation of |type|. The |code| value follows the HCI Error Codes from
Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.*/
pub type BluetoothEmulationSimulateGattOperationResponseResults = ();
/** Simulates the response from the characteristic with |characteristicId| for a
characteristic operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateCharacteristicOperationResponseParams = ();
/** Simulates the response from the characteristic with |characteristicId| for a
characteristic operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateCharacteristicOperationResponseResults = ();
/** Simulates the response from the descriptor with |descriptorId| for a
descriptor operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateDescriptorOperationResponseParams = ();
/** Simulates the response from the descriptor with |descriptorId| for a
descriptor operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateDescriptorOperationResponseResults = ();
/// Adds a service with |serviceUuid| to the peripheral with |address|.
pub type BluetoothEmulationAddServiceParams = ();
/// Adds a service with |serviceUuid| to the peripheral with |address|.
pub type BluetoothEmulationAddServiceResults = ();
/// Removes the service respresented by |serviceId| from the simulated central.
pub type BluetoothEmulationRemoveServiceParams = ();
/// Removes the service respresented by |serviceId| from the simulated central.
pub type BluetoothEmulationRemoveServiceResults = ();
/** Adds a characteristic with |characteristicUuid| and |properties| to the
service represented by |serviceId|.*/
pub type BluetoothEmulationAddCharacteristicParams = ();
/** Adds a characteristic with |characteristicUuid| and |properties| to the
service represented by |serviceId|.*/
pub type BluetoothEmulationAddCharacteristicResults = ();
/** Removes the characteristic respresented by |characteristicId| from the
simulated central.*/
pub type BluetoothEmulationRemoveCharacteristicParams = ();
/** Removes the characteristic respresented by |characteristicId| from the
simulated central.*/
pub type BluetoothEmulationRemoveCharacteristicResults = ();
/** Adds a descriptor with |descriptorUuid| to the characteristic respresented
by |characteristicId|.*/
pub type BluetoothEmulationAddDescriptorParams = ();
/** Adds a descriptor with |descriptorUuid| to the characteristic respresented
by |characteristicId|.*/
pub type BluetoothEmulationAddDescriptorResults = ();
/// Removes the descriptor with |descriptorId| from the simulated central.
pub type BluetoothEmulationRemoveDescriptorParams = ();
/// Removes the descriptor with |descriptorId| from the simulated central.
pub type BluetoothEmulationRemoveDescriptorResults = ();
/// Simulates a GATT disconnection from the peripheral with |address|.
pub type BluetoothEmulationSimulateGattDisconnectionParams = ();
/// Simulates a GATT disconnection from the peripheral with |address|.
pub type BluetoothEmulationSimulateGattDisconnectionResults = ();
