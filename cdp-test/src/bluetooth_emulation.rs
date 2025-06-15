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
pub struct BluetoothEmulationEnableParams {
    test: (),
    test: (),
}
/// Enable the BluetoothEmulation domain.
pub type BluetoothEmulationEnableReturns = ();
/// Set the state of the simulated central.
pub struct BluetoothEmulationSetSimulatedCentralStateParams {
    test: (),
}
/// Set the state of the simulated central.
pub type BluetoothEmulationSetSimulatedCentralStateReturns = ();
/// Disable the BluetoothEmulation domain.
pub type BluetoothEmulationDisableParams = ();
/// Disable the BluetoothEmulation domain.
pub type BluetoothEmulationDisableReturns = ();
/** Simulates a peripheral with |address|, |name| and |knownServiceUuids|
that has already been connected to the system.*/
pub struct BluetoothEmulationSimulatePreconnectedPeripheralParams {
    test: (),
    test: (),
    test: (),
    test: (),
}
/** Simulates a peripheral with |address|, |name| and |knownServiceUuids|
that has already been connected to the system.*/
pub type BluetoothEmulationSimulatePreconnectedPeripheralReturns = ();
/** Simulates an advertisement packet described in |entry| being received by
the central.*/
pub struct BluetoothEmulationSimulateAdvertisementParams {
    test: (),
}
/** Simulates an advertisement packet described in |entry| being received by
the central.*/
pub type BluetoothEmulationSimulateAdvertisementReturns = ();
/** Simulates the response code from the peripheral with |address| for a
GATT operation of |type|. The |code| value follows the HCI Error Codes from
Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.*/
pub struct BluetoothEmulationSimulateGattOperationResponseParams {
    test: (),
    test: (),
    test: (),
}
/** Simulates the response code from the peripheral with |address| for a
GATT operation of |type|. The |code| value follows the HCI Error Codes from
Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.*/
pub type BluetoothEmulationSimulateGattOperationResponseReturns = ();
/** Simulates the response from the characteristic with |characteristicId| for a
characteristic operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub struct BluetoothEmulationSimulateCharacteristicOperationResponseParams {
    test: (),
    test: (),
    test: (),
    test: (),
}
/** Simulates the response from the characteristic with |characteristicId| for a
characteristic operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateCharacteristicOperationResponseReturns = ();
/** Simulates the response from the descriptor with |descriptorId| for a
descriptor operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub struct BluetoothEmulationSimulateDescriptorOperationResponseParams {
    test: (),
    test: (),
    test: (),
    test: (),
}
/** Simulates the response from the descriptor with |descriptorId| for a
descriptor operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateDescriptorOperationResponseReturns = ();
/// Adds a service with |serviceUuid| to the peripheral with |address|.
pub struct BluetoothEmulationAddServiceParams {
    test: (),
    test: (),
}
/// Adds a service with |serviceUuid| to the peripheral with |address|.
pub type BluetoothEmulationAddServiceReturns = ();
/// Removes the service respresented by |serviceId| from the simulated central.
pub struct BluetoothEmulationRemoveServiceParams {
    test: (),
}
/// Removes the service respresented by |serviceId| from the simulated central.
pub type BluetoothEmulationRemoveServiceReturns = ();
/** Adds a characteristic with |characteristicUuid| and |properties| to the
service represented by |serviceId|.*/
pub struct BluetoothEmulationAddCharacteristicParams {
    test: (),
    test: (),
    test: (),
}
/** Adds a characteristic with |characteristicUuid| and |properties| to the
service represented by |serviceId|.*/
pub type BluetoothEmulationAddCharacteristicReturns = ();
/** Removes the characteristic respresented by |characteristicId| from the
simulated central.*/
pub struct BluetoothEmulationRemoveCharacteristicParams {
    test: (),
}
/** Removes the characteristic respresented by |characteristicId| from the
simulated central.*/
pub type BluetoothEmulationRemoveCharacteristicReturns = ();
/** Adds a descriptor with |descriptorUuid| to the characteristic respresented
by |characteristicId|.*/
pub struct BluetoothEmulationAddDescriptorParams {
    test: (),
    test: (),
}
/** Adds a descriptor with |descriptorUuid| to the characteristic respresented
by |characteristicId|.*/
pub type BluetoothEmulationAddDescriptorReturns = ();
/// Removes the descriptor with |descriptorId| from the simulated central.
pub struct BluetoothEmulationRemoveDescriptorParams {
    test: (),
}
/// Removes the descriptor with |descriptorId| from the simulated central.
pub type BluetoothEmulationRemoveDescriptorReturns = ();
/// Simulates a GATT disconnection from the peripheral with |address|.
pub struct BluetoothEmulationSimulateGattDisconnectionParams {
    test: (),
}
/// Simulates a GATT disconnection from the peripheral with |address|.
pub type BluetoothEmulationSimulateGattDisconnectionReturns = ();
